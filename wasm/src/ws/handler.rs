use crate::handler::MFnResult;
use shared::WSServerMsg;

inventory::collect!(WSInitFunc);
pub struct WSInitFunc {
	pub handler: fn() -> MFnResult<'static>,
}

inventory::collect!(WSMsgHandler);
pub(super) struct WSMsgHandler {
	pub handler: fn(WSServerMsg) -> MFnResult<'static>,
}
