use crate::{handler::MFnResult, ui::msg::UIMsg};

inventory::collect!(UIInitFunc);
pub(super) struct UIInitFunc {
	pub init: fn() -> MFnResult<'static>,
}

inventory::collect!(UIMsgHandler);
pub(super) struct UIMsgHandler {
	pub handler: fn(UIMsg) -> MFnResult<'static>,
}
