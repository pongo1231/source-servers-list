mod entries;
mod handler;
use crate::{
	ui::{UI_CHANNEL, msg::UIMsg},
	ws::handler::{WSInitFunc, WSMsgHandler},
};
use futures_util::StreamExt;
use gloo_console::*;
use lazy_static::lazy_static;
use shared::{WSClientMsg, WSServerMsg, handler::InitFunc, stream::WSStream};
use tokio::sync::broadcast;
use web_sys::window;

lazy_static! {
	pub static ref WS_CHANNEL: broadcast::Sender::<WSClientMsg> = broadcast::channel(100).0;
}

inventory::submit! {
	InitFunc {
		init
	}
}
fn init() {
	wasm_bindgen_futures::spawn_local(async move {
		let win = window().unwrap();
		let fqdn = format!(
			"{}//{}:{}",
			win.location().protocol().unwrap(),
			win.location().hostname().unwrap(),
			win.location().port().unwrap()
		);

		loop {
			let Ok(ws_stream) = tokio_tungstenite_wasm::connect(format!("{fqdn}/ws")).await else {
				error!("Connection to websocket failed! Retrying...");
				continue;
			};

			let mut ws_stream = WSStream { inner: ws_stream };

			for init in inventory::iter::<WSInitFunc> {
				(init.init)().await;
			}

			_ = UI_CHANNEL.send(UIMsg::Init);

			_ = ws_stream.send(WSClientMsg::ReqEntries).await;

			let mut ws_channel = WS_CHANNEL.subscribe();
			loop {
				tokio::select! {
					msg = ws_stream.next() => {
						let Some(msg) = msg else {
							break;
						};

						let Ok(msg) = msg else {
							continue;
						};

						debug!(msg.to_string());

						let Ok(msg) = serde_json::from_str::<WSServerMsg>(&msg.to_string()) else {
							continue;
						};

						wasm_bindgen_futures::spawn_local(async move {
							for handler in inventory::iter::<WSMsgHandler> {
								(handler.handler)(&msg).await;
							}
						});
					}
					Ok(msg) = ws_channel.recv() => {
						_ = ws_stream.send(msg).await;
					}
				}
			}

			log!("Disconnected from websocket. Reconnecting...");
		}
	});
}
