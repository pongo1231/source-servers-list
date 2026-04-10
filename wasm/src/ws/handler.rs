use crate::handler::MFnResult;
use shared::WSServerMsg;

inventory::collect!(WSInitFunc);
pub(super) struct WSInitFunc {
	pub init: fn() -> MFnResult<'static>,
}

inventory::collect!(WSMsgHandler);
pub(super) struct WSMsgHandler {
	pub handler: fn(WSServerMsg) -> MFnResult<'static>,
}
