use crate::{
	handler::MFnResult,
	ui::{UI_CHANNEL, msg::UIMsg},
	ws::handler::WSMsgHandler,
};
use ref_thread_local::RefThreadLocal;
use shared::WSServerMsg;

inventory::submit! {
	WSMsgHandler {
		handler
	}
}
fn handler(msg: WSServerMsg) -> MFnResult<'static> {
	Box::pin(async move {
		match msg {
			WSServerMsg::ResEntries(entries) => {
				for listing in entries {
					_ = UI_CHANNEL.borrow_mut().send(UIMsg::UpdateListing(listing));
				}
			}
			WSServerMsg::ResPlayers(id, player_names) => {
				_ = UI_CHANNEL
					.borrow_mut()
					.send(UIMsg::UpdatePlayers(id, player_names));
			}
			_ => {}
		}
	})
}
