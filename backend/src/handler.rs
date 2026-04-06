use rocket::Route;
use rocket_ws::stream::DuplexStream;
use shared::{WSClientMsg, stream::WSStream};
use std::pin::Pin;

pub type WSFnResult<'a> = Pin<Box<dyn Future<Output = ()> + 'a + Send>>;

inventory::collect!(WSInitFunc);
pub struct WSInitFunc {
	pub init: for<'a> fn(&'a mut WSStream<DuplexStream>) -> WSFnResult<'a>,
}

inventory::collect!(WSMsgHandler);
pub struct WSMsgHandler {
	pub handler: for<'a> fn(&'a mut WSStream<DuplexStream>, &'a WSClientMsg) -> WSFnResult<'a>,
}

inventory::collect!(RouteCollector);
pub struct RouteCollector {
	pub handler: fn() -> Vec<Route>,
}
