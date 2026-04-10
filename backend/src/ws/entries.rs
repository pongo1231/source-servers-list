use crate::handler::{InitFunc, MFnResult};
use crate::ws::WS_GLOBAL_CHANNEL;
use crate::ws::handler::WSMsgHandler;
use crate::{env, rcon};
use lazy_static::lazy_static;
use rocket::tokio;
use rocket::tokio::sync::broadcast;
use rocket::tokio::time::sleep;
use serde::Deserialize;
use shared::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

#[derive(Deserialize)]
struct GameListing {
	game: String,
	icon: Option<String>,
	rconpass: Option<String>,
	servers: Vec<String>,
}

lazy_static! {
	static ref SERVER_CACHE_LAST_TIMESTAMP: Mutex<SystemTime> = Mutex::new(SystemTime::MIN);
	static ref SERVER_CACHED_RESULTS: Arc<Mutex<Vec<ServerListing>>> = Arc::new(Mutex::new({
		static mut ID: u16 = 0;
		let mut entries = Vec::<ServerListing>::new();

		let file_content = std::fs::read("servers.yaml").expect("servers.yaml not found!");
		for listing in serde_yaml::from_slice::<Vec<GameListing>>(&file_content)
			.expect("servers.yaml is malformed!")
		{
			for server in listing.servers {
				entries.push(ServerListing {
					id: unsafe {
						ID += 1;
						ID - 1
					},
					game: listing.game.clone(),
					addr: server.clone(),
					status: ServerListingStatus::Pending,
					icon_name: listing.icon.clone(),
					rcon_password: listing.rconpass.clone(),
					..Default::default()
				});
			}
		}
		entries
	}));
	static ref SERVER_CACHED_PLAYERS: Arc<Mutex<HashMap<u16, Vec<String>>>> =
		Arc::new(Mutex::new(HashMap::new()));
}

inventory::submit! {
	InitFunc {
		init
	}
}
fn init() -> MFnResult<'static> {
	Box::pin(async {
		loop {
			sleep(Duration::from_mins(5)).await;

			if WS_GLOBAL_CHANNEL.receiver_count() > 0 {
				stream_entries(None).await;
			}
		}
	})
}

inventory::submit! {
	WSMsgHandler {
		handler
	}
}
fn handler(ws_channel: broadcast::Sender<WSServerMsg>, msg: WSClientMsg) -> MFnResult<'static> {
	Box::pin(async move {
		match msg {
			WSClientMsg::ReqEntries => {
				stream_entries(Some(ws_channel.clone())).await;
			}
			WSClientMsg::ReqPlayers(id) => {
				let player_names = SERVER_CACHED_PLAYERS
					.lock()
					.unwrap()
					.get(&id)
					.unwrap_or(&Vec::new())
					.clone();

				_ = ws_channel.send(WSServerMsg::ResPlayers(id, player_names));
			}
			_ => {}
		}
	})
}

async fn stream_entries(mut ws_channel: Option<broadcast::Sender<WSServerMsg>>) {
	let mut cached_results = SERVER_CACHED_RESULTS.lock().unwrap().clone();

	if let Some(ref mut stream) = ws_channel {
		_ = stream.send(WSServerMsg::ResEntries(cached_results.clone()));
	}

	if SERVER_CACHE_LAST_TIMESTAMP
		.lock()
		.unwrap()
		.elapsed()
		.unwrap()
		< Duration::from_mins(2)
	{
		return;
	}

	*SERVER_CACHE_LAST_TIMESTAMP.lock().unwrap() = SystemTime::now();

	let (channel_tx, mut channel_rx) = broadcast::channel::<(usize, ServerListing)>(100);

	for (i, listing) in cached_results.iter_mut().enumerate() {
		let channel_tx = channel_tx.clone();
		let mut listing = listing.clone();
		tokio::spawn(async move {
			let Some(rcon) = rcon::rcon(
				&listing.addr,
				listing
					.rcon_password
					.clone()
					.unwrap_or(env::RCON_PASSWORD.get().unwrap().clone())
					.as_str(),
				"status",
			) else {
				listing.status = ServerListingStatus::Unreachable;
				_ = channel_tx.send((i, listing.clone()));
				return;
			};

			let orig_listing = listing.clone();

			listing.status = ServerListingStatus::Reachable;
			listing.info = Some(ServerListingInfo::default());

			let mut lines = rcon.lines();

			'hostname: {
				let Some(hostname) = lines.find(|l| l.contains("hostname:")) else {
					break 'hostname;
				};

				let Some(hostname) = hostname.split("hostname: ").nth(1) else {
					break 'hostname;
				};

				listing.name = hostname.trim().to_string();
			}

			let info = listing.info.as_mut().unwrap();

			'map: {
				let Some(map) = lines.find(|l| l.starts_with("map")) else {
					break 'map;
				};

				let Some(map) = map.strip_prefix("map") else {
					break 'map;
				};

				let Some(map) = map.split(":").nth(1) else {
					break 'map;
				};

				let Some(map) = map.split(" at").next() else {
					break 'map;
				};

				info.map = map.trim().to_string();
			}

			'players: {
				let Some(players) = lines.find(|l| l.starts_with("players")) else {
					break 'players;
				};

				/*'active: {
					let Some(active) = players.split(":").nth(1) else {
						break 'active;
					};

					let Some(active) = active.trim().split(" ").next() else {
						break 'active;
					};

					let Ok(active) = active.parse::<u8>() else {
						break 'active;
					};

					info.active = active;
				}*/

				'max: {
					let Some(max) = players.split("(").nth(1) else {
						break 'max;
					};

					let Some(max) = max.trim().split(" ").next() else {
						break 'max;
					};

					let Ok(max) = max.parse::<u8>() else {
						break 'max;
					};

					info.max = max
						- if listing.game.contains("Team Fortress") && max > 0 {
							1
						} else {
							0
						}; // HACK
				}

				let mut player_names = Vec::<String>::new();
				for line in lines {
					if !line.starts_with("#") {
						continue;
					}

					let Some(split_pos) = line.rfind("\"") else {
						continue;
					};

					let (l_split, r_split) = line.split_at(split_pos);

					let Some(split_pos) = l_split.find("\"") else {
						continue;
					};

					let (_, name) = l_split.split_at(split_pos + 1);
					let mut name = name.to_string();

					if name == "SourceTV" {
						continue;
					} else if r_split.contains("BOT") {
						name = format!("(BOT) {}", name);
					} else {
						info.active += 1;
					}

					player_names.push(name.to_string());
				}
				SERVER_CACHED_PLAYERS
					.lock()
					.unwrap()
					.insert(listing.id, player_names);
			}

			if info.map.is_empty() && info.active == 0 && info.max == 0 {
				listing.status = ServerListingStatus::Unreachable;
			}

			if listing != orig_listing {
				_ = channel_tx.send((i, listing.clone()));
			}
		});
	}

	drop(channel_tx);

	while let Ok(msg) = channel_rx.recv().await {
		let (i, listing) = msg;

		_ = WS_GLOBAL_CHANNEL.send(WSServerMsg::ResEntries(Vec::from([listing.clone()])));

		SERVER_CACHED_RESULTS.lock().unwrap()[i] = listing;
	}
}
