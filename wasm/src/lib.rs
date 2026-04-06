mod notif;
mod ui;
mod ws;
use shared::handler::InitFunc;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub async fn init_page() {
	#[cfg(debug_assertions)]
	console_error_panic_hook::set_once();

	for init in inventory::iter::<InitFunc> {
		(init.init)();
	}
}
