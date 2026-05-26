use crate::{
	handler::MFnResult,
	notif,
	ui::{
		UI_CHANNEL,
		handler::UIMsgHandler,
		listings::{ClientServerEntry, GAME_SECTIONS, GameSection, SERVER_ENTRIES, SERVER_PLAYERS},
		msg::UIMsg,
	},
	ws::WS_CHANNEL,
};
use gloo::events::EventListener;
use shared::{ServerListingStatus, WSClientMsg};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlElement, window};

inventory::submit! {
	UIMsgHandler {
		handler
	}
}
fn handler(msg: UIMsg) -> MFnResult<'static> {
	Box::pin(async {
		let UIMsg::UpdateListing(listing) = msg else {
			return;
		};

		let win = window().unwrap();
		let doc = win.document().unwrap();

		let servers_container = doc.get_element_by_id("ServersContainer").unwrap();

		let mut server_entries = SERVER_ENTRIES.lock().await;
		let client_entry = match server_entries.get_mut(&listing.id) {
			Some(item) => item,
			None => {
				let game_key = listing.game.clone();
				let mut game_sections = GAME_SECTIONS.lock().await;
				let game_grid = if let Some(section) = game_sections.get(&game_key) {
					section.grid.clone()
				} else {
					let section_div = doc
						.create_element("div")
						.unwrap()
						.unchecked_into::<HtmlElement>();
					section_div.set_class_name("GameSection");

					let icon_html = match &listing.icon_name {
						Some(icon) => format!("<img class='game-icon' src='{}' width='32'/>", icon),
						None => String::new(),
					};

					section_div.set_inner_html(
						format!(
							"<div class='game-header'>{}<h3>{}</h3></div><hr class='game-separator'><div class='game-grid'></div>",
							icon_html,
							listing.game
						).as_str()
					);

					_ = servers_container.append_child(&section_div);

					let grid = section_div
						.query_selector(".game-grid")
						.unwrap()
						.unwrap()
						.unchecked_into::<HtmlElement>();

					game_sections.insert(game_key.clone(), GameSection { grid: grid.clone() });
					grid
				};

				let item = doc
					.create_element("div")
					.unwrap()
					.unchecked_into::<HtmlElement>();
				_ = game_grid.append_child(&item);
				item.set_class_name("ServerContainer");

				server_entries.insert(
					listing.id,
					ClientServerEntry {
						html_element: item.clone(),
						first_update: true,
					},
				);

				{
					let id = listing.id;
					EventListener::new(&item, "click", move |_| {
						spawn_local(async move {
							let win = window().unwrap();
							let doc = win.document().unwrap();

							let Some(detailed_item) =
								doc.get_element_by_id(format!("{}expanded", id).as_str())
							else {
								return;
							};

							if detailed_item.class_list().contains("expanded") {
								_ = detailed_item.class_list().remove_1("expanded");
							} else {
								let server_players = SERVER_PLAYERS.lock().await;
								let Some(players) = server_players.get(&id) else {
									_ = WS_CHANNEL.send(WSClientMsg::ReqPlayers(id));
									_ = UI_CHANNEL.send(UIMsg::UpdatePlayers(
										id,
										Vec::from(["<img class='Spinner'/>".to_string()]),
									));
									return;
								};

								_ = UI_CHANNEL.send(UIMsg::UpdatePlayers(id, players.clone()));
							}
						});
					})
					.forget();
				}

				server_entries.get_mut(&listing.id).unwrap()
			}
		};

		let item = &client_entry.html_element;

		if !client_entry.first_update {
			_ = item.class_list().remove_1("flash-highlight");
			_ = item.client_width();
			_ = item.class_list().add_1("flash-highlight");

			if listing.status == ServerListingStatus::Reachable
				&& doc.hidden()
				&& !doc.has_focus().unwrap_or_default()
			{
				notif::show(
					"Server Update",
					format!(
						"{}\n{}{}{}",
						listing.game,
						listing.name,
						if let Some(ref info) = listing.info
							&& !info.map.is_empty()
						{
							format!("\nMap: {}", info.map.as_str())
						} else {
							"".to_string()
						},
						if let Some(ref info) = listing.info
							&& (info.active > 0 || info.max != 0)
						{
							format!("\nPlayers: {} / {}", info.active, info.max)
						} else {
							"".to_string()
						}
					)
					.as_str(),
				);
			}
		}

		match listing.status {
			ServerListingStatus::Pending => {
				item.set_inner_html("<img class='Spinner'/>");
			}
			ServerListingStatus::Unreachable => {
				item.set_inner_html(
					format!(
						"<div class='server-header'><p>{}</p>{}<hr class='server-separator'></div>IP: {}<br>Unreachable",
						listing.game,
						if listing.name.is_empty() {
							"".to_string()
						} else {
							format!("<h2>{}</h2>", listing.name)
						},
						listing.addr
					)
					.as_str(),
				);
			}
			ServerListingStatus::Reachable => {
				client_entry.first_update = false;

				item.set_inner_html(
				format!(
					"<div class='server-header'>{}{}</p><h2>{}</h2><hr class='server-separator'></div>{}<br><a href=\"steam://connect/{}\" onclick=\"event.stopPropagation()\">Connect</a><div class='details' id='{}expanded'></div>",
					if let Some(icon_name) = &listing.icon_name { format!("<p><img class='bg' src='{}' width='32'/>", icon_name) } else { "<p>".to_string() },
					listing.game,
					listing.name,
					if let Some(ref info) = listing.info {
						format!(
							"Map: {}<br>Players: {}",
							if info.map.is_empty() {
								"Unknown"
							} else {
								info.map.as_str()
							},
							if info.active == 0 && info.max == 0 {
								"? / ?".to_string()
							} else {
								format!("{} / {}", info.active, info.max)
							}
						)
					} else {
						"".to_string()
					},
					listing.addr,
					listing.id
				)
				.as_str(),
			);
			}
		}

		SERVER_PLAYERS.lock().await.remove(&listing.id);
	})
}
