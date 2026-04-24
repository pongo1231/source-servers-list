mod handler;
mod listings;
pub mod msg;
use crate::{
	handler::{InitFunc, MFnResult},
	ui::{
		handler::{UIInitFunc, UIMsgHandler},
		msg::UIMsg,
	},
	ws::handler::WSInitFunc,
};
use gloo::utils::window;
use ref_thread_local::{RefThreadLocal, ref_thread_local};
use tokio::sync::broadcast;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;

ref_thread_local! {
	pub static managed UI_CHANNEL: broadcast::Sender<UIMsg> = broadcast::channel(100).0;
}

inventory::submit! {
	InitFunc {
		handler: init
	}
}
fn init() -> MFnResult<'static> {
	Box::pin(async {
		let mut ui_channel = UI_CHANNEL.borrow().subscribe();
		while let Ok(msg) = ui_channel.recv().await {
			match msg {
				UIMsg::WSInit => {
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
						spawn_local(async {
							(init.handler)().await;
						});
					}
				}
				_ => {
					for handler in inventory::iter::<UIMsgHandler> {
						let msg = msg.clone();
						spawn_local(async {
							(handler.handler)(msg).await;
						});
					}
				}
			}
		}
	})
}

inventory::submit! {
	WSInitFunc {
		handler: ws_init
	}
}
fn ws_init() -> MFnResult<'static> {
	Box::pin(async {
		_ = UI_CHANNEL.borrow_mut().send(UIMsg::WSInit);
	})
}
