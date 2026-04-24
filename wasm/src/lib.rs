mod handler;
mod notif;
mod ui;
mod ws;
use crate::handler::InitFunc;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
pub async fn init_page() {
	#[cfg(debug_assertions)]
	console_error_panic_hook::set_once();

	for init in inventory::iter::<InitFunc> {
		spawn_local(async {
			(init.handler)().await;
		});
	}
}
