use shared::WSServerMsg;
use std::pin::Pin;

pub(super) type WSFnResult<'a> = Pin<Box<dyn Future<Output = ()> + 'a + Send>>;

inventory::collect!(WSInitFunc);
pub(super) struct WSInitFunc {
	pub init: for<'a> fn() -> WSFnResult<'static>,
}

inventory::collect!(WSMsgHandler);
pub(super) struct WSMsgHandler {
	pub handler: for<'a> fn(&'a WSServerMsg) -> WSFnResult<'a>,
}
