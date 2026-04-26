mod listing;
mod players_list;
use crate::{handler::MFnResult, ui::handler::UIInitFunc};
use futures_util::lock::Mutex;
use std::{collections::HashMap, sync::LazyLock};
use web_sys::{HtmlElement, window};

struct ClientServerEntry {
	pub html_element: HtmlElement,
	pub first_update: bool,
}
static SERVER_ENTRIES: LazyLock<Mutex<HashMap<u16, ClientServerEntry>>> =
	LazyLock::new(|| Mutex::new(HashMap::new()));
static SERVER_PLAYERS: LazyLock<Mutex<HashMap<u16, Vec<String>>>> =
	LazyLock::new(|| Mutex::new(HashMap::new()));

inventory::submit! {
	UIInitFunc {
		handler: init
	}
}
fn init() -> MFnResult<'static> {
	Box::pin(async {
		window()
			.unwrap()
			.document()
			.unwrap()
			.get_element_by_id("ServersContainer")
			.unwrap()
			.set_inner_html("");

		SERVER_ENTRIES.lock().await.clear();
		SERVER_PLAYERS.lock().await.clear();
	})
}
