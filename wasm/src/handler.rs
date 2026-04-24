use std::pin::Pin;

pub type MFnResult<'a> = Pin<Box<dyn Future<Output = ()> + 'a>>;

inventory::collect!(InitFunc);
pub struct InitFunc {
	pub handler: fn() -> MFnResult<'static>,
}
