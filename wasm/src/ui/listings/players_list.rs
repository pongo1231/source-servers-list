use crate::{
	handler::MFnResult,
	ui::{handler::UIMsgHandler, listings::SERVER_PLAYERS, msg::UIMsg},
};
use ref_thread_local::RefThreadLocal;
use web_sys::window;

inventory::submit! {
	UIMsgHandler {
		handler
	}
}
fn handler(msg: UIMsg) -> MFnResult<'static> {
	Box::pin(async {
		let UIMsg::UpdatePlayers(id, player_names) = msg else {
			return;
		};

		let doc = window().unwrap().document().unwrap();

		SERVER_PLAYERS.borrow_mut().insert(id, player_names.clone());

		let Some(elem) = doc.get_element_by_id(format!("{id}expanded").as_str()) else {
			return;
		};

		_ = elem.class_list().add_1("expanded");
		elem.set_inner_html(
			format!(
				"<div class='inner'><br><hr>Players<hr>{}</div>",
				player_names
					.iter()
					.map(|p| format!("{p}<br>"))
					.collect::<String>()
			)
			.as_str(),
		);
	})
}
