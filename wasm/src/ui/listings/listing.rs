use gloo::events::EventListener;
use ref_thread_local::RefThreadLocal;
use shared::{ServerListingStatus, WSClientMsg};
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, window};
use crate::{handler::MFnResult, notif, ui::{UI_CHANNEL, handler::UIMsgHandler, listings::{ClientServerEntry, SERVER_ENTRIES, SERVER_PLAYERS}, msg::UIMsg}, ws::WS_CHANNEL};

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

	let doc = window().unwrap().document().unwrap();

	let mut server_items = SERVER_ENTRIES.borrow_mut();
	let servers_container = doc.get_element_by_id("ServersContainer").unwrap();

	let client_entry = match server_items.get_mut(&listing.id) {
		Some(item) => item,
		None => {
			let item = doc
				.create_element("div")
				.unwrap()
				.unchecked_into::<HtmlElement>();
			_ = servers_container.append_child(&item);
			item.set_class_name("ServerContainer");

			server_items.insert(
				listing.id,
				ClientServerEntry {
					html_element: item.clone(),
					first_update: true,
				},
			);

			{
				let doc = doc.clone();
				let id = listing.id;
				 EventListener::new(
						&item,
						"click",
						move |_| {
							let Some(detailed_item) =
								doc.get_element_by_id(format!("{}expanded", id).as_str())
							else {
								return;
							};

							if detailed_item.class_list().contains("expanded") {
								_ = detailed_item.class_list().remove_1("expanded");
							} else {
								let server_players = SERVER_PLAYERS.borrow();
								let Some(players) = server_players.get(&id) else {
									_ = WS_CHANNEL.borrow().send(WSClientMsg::ReqPlayers(id));
									_ = UI_CHANNEL.borrow().send(UIMsg::UpdatePlayers(id, Vec::from(["<img class='Spinner'/>".to_string()])));
									return;
								};

								_ = UI_CHANNEL.borrow().send(UIMsg::UpdatePlayers(id, players.clone()));
							}
						}).forget();
			}

			server_items.get_mut(&listing.id).unwrap()
		}
	};

	let item = &client_entry.html_element;

	if !client_entry.first_update {
		_ = item.class_list().remove_1("flash-highlight");
		_ = item.client_width();
		_ = item.class_list().add_1("flash-highlight");

		if listing.status == ServerListingStatus::Reachable {
			notif::show(
				"Server Update",
				format!(
					"{}\n{}{}{}",
					listing.game,
					listing
						.name
						.split("|")
						.nth(1)
						.unwrap_or(listing.name.as_str())
						.trim(),
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
					"{}{}<br>IP: {}<br>Unreachable",
					listing.game,
					if listing.name.is_empty() {
						"".to_string()
					} else {
						format!("<h2>{}</h2><br>", listing.name)
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
					"{}{}</p><h2>{}</h2>{}<br><a href=\"steam://connect/{}\" onclick=\"event.stopPropagation()\">Connect</a><div class='details' id='{}expanded'></div>",
					if let Some(icon_name) = &listing.icon_name { format!("<p><img class='bg' src='{}' width='32'/>", icon_name) } else { "".to_string() },
					listing.game,
					listing.name.split("|").nth(1).unwrap_or(listing.name.as_str()).trim(),
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

	SERVER_PLAYERS.borrow_mut().remove(&listing.id);
})
}