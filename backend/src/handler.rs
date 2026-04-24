use rocket::Route;
use std::pin::Pin;

pub type MFnResult<'a> = Pin<Box<dyn Future<Output = ()> + 'a + Send>>;

inventory::collect!(InitFunc);
pub struct InitFunc {
	pub handler: fn() -> MFnResult<'static>,
}

inventory::collect!(RouteCollector);
pub struct RouteCollector {
	pub handler: fn() -> Vec<Route>,
}
