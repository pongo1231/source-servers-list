use crate::ui::msg::UIMsg;

inventory::collect!(UIInitFunc);
pub(super) struct UIInitFunc {
	pub init: fn(),
}

inventory::collect!(UIMsgHandler);
pub(super) struct UIMsgHandler {
	pub handler: fn(&UIMsg),
}
