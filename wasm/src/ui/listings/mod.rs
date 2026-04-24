mod listing;
mod players_list;
use crate::{handler::MFnResult, ui::handler::UIInitFunc};
use ref_thread_local::{RefThreadLocal, ref_thread_local};
use std::collections::HashMap;
use web_sys::{HtmlElement, window};

struct ClientServerEntry {
	pub html_element: HtmlElement,
	pub first_update: bool,
}
ref_thread_local! {
	static managed SERVER_ENTRIES: HashMap::<u16, ClientServerEntry> =
		HashMap::new();
	static managed SERVER_PLAYERS: HashMap::<u16, Vec<String>> =
		HashMap::new();
}

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

		SERVER_ENTRIES.borrow_mut().clear();
		SERVER_PLAYERS.borrow_mut().clear();
	})
}
