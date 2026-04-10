use crate::handler::MFnResult;
use rocket::tokio::sync::broadcast;
use shared::{WSClientMsg, WSServerMsg};

inventory::collect!(WSInitFunc);
pub struct WSInitFunc {
	pub init: fn(broadcast::Sender<WSServerMsg>) -> MFnResult<'static>,
}

inventory::collect!(WSMsgHandler);
pub struct WSMsgHandler {
	pub handler: fn(broadcast::Sender<WSServerMsg>, WSClientMsg) -> MFnResult<'static>,
}
