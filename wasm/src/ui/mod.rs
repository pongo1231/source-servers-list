mod handler;
mod listings;
pub mod msg;
use crate::ui::{
	handler::{UIInitFunc, UIMsgHandler},
	msg::UIMsg,
};
use gloo::utils::window;
use lazy_static::lazy_static;
use shared::handler::InitFunc;
use tokio::sync::broadcast;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

lazy_static! {
	pub static ref UI_CHANNEL: broadcast::Sender<UIMsg> = broadcast::channel(100).0;
}

inventory::submit! {
	InitFunc {
		init
	}
}
fn init() {
	wasm_bindgen_futures::spawn_local(async {
		let mut ui_channel = UI_CHANNEL.subscribe();
		while let Ok(msg) = ui_channel.recv().await {
			match msg {
				UIMsg::Init => {
					_ = window()
						.document()
						.unwrap()
						.get_element_by_id("Banner")
						.unwrap()
						.dyn_into::<HtmlElement>()
						.unwrap()
						.style()
						.remove_property("display");

					for init in inventory::iter::<UIInitFunc> {
						(init.init)();
					}
				}
				_ => {
					for handler in inventory::iter::<UIMsgHandler> {
						(handler.handler)(&msg);
					}
				}
			}
		}
	});
}
