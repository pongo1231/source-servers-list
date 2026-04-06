mod listing;
mod players_list;
use crate::ui::handler::UIInitFunc;
use gloo::events::EventListener;
use lazy_static::lazy_static;
use std::{
	cell::RefCell,
	collections::HashMap,
	sync::{Arc, Mutex},
};
use web_sys::{HtmlElement, window};

struct ClientServerEntry {
	pub html_element: HtmlElement,
	pub first_update: bool,
}
lazy_static! {
	static ref SERVER_ENTRIES: Arc<Mutex<HashMap::<u16, ClientServerEntry>>> =
		Arc::new(Mutex::new(HashMap::new()));
	static ref SERVER_PLAYERS: Arc<Mutex<HashMap::<u16, Vec<String>>>> =
		Arc::new(Mutex::new(HashMap::new()));
}

thread_local! {
	static ELEMENT_LISTENERS: RefCell<Vec<EventListener>> = const { RefCell::new(Vec::new()) }
}

inventory::submit! {
	UIInitFunc {
		init
	}
}
fn init() {
	window()
		.unwrap()
		.document()
		.unwrap()
		.get_element_by_id("ServersContainer")
		.unwrap()
		.set_inner_html("");

	SERVER_ENTRIES.lock().unwrap().clear();
	SERVER_PLAYERS.lock().unwrap().clear();
	ELEMENT_LISTENERS.set(Vec::new());
}
