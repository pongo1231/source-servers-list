use shared::{WSServerMsg, stream::WSStream};
use std::pin::Pin;
use tokio_tungstenite_wasm::WebSocketStream;

pub(super) type WSFnResult<'a> = Pin<Box<dyn Future<Output = ()> + 'a>>;

inventory::collect!(WSInitFunc);
pub(super) struct WSInitFunc {
	pub init: for<'a> fn(&'a mut WSStream<WebSocketStream>) -> WSFnResult<'a>,
}

inventory::collect!(WSMsgHandler);
pub(super) struct WSMsgHandler {
	pub handler: for<'a> fn(&'a mut WSStream<WebSocketStream>, &'a WSServerMsg) -> WSFnResult<'a>,
}
