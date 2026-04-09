use crate::{
	ui::{UI_CHANNEL, msg::UIMsg},
	ws::handler::{WSFnResult, WSMsgHandler},
};
use shared::WSServerMsg;

inventory::submit! {
	WSMsgHandler {
		handler
	}
}
fn handler<'a>(msg: &'a WSServerMsg) -> WSFnResult<'a> {
	Box::pin(async move {
		match msg {
			WSServerMsg::ResEntries(entries) => {
				for listing in entries {
					_ = UI_CHANNEL.send(UIMsg::UpdateListing(listing.clone()));
				}
			}
			WSServerMsg::ResPlayers(id, player_names) => {
				_ = UI_CHANNEL.send(UIMsg::UpdatePlayers(*id, player_names.clone()));
			}
			_ => {}
		}
	})
}
