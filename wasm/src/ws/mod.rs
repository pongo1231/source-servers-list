mod entries;
pub mod handler;
use crate::{
	handler::{InitFunc, MFnResult},
	ws::handler::{WSInitFunc, WSMsgHandler},
};
use futures_util::StreamExt;
use gloo_console::*;
use ref_thread_local::{RefThreadLocal, ref_thread_local};
use shared::{WSClientMsg, WSServerMsg, stream::WSStream};
use tokio::sync::broadcast;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;

ref_thread_local! {
	pub static managed WS_CHANNEL: broadcast::Sender::<WSClientMsg> = broadcast::channel(100).0;
}

inventory::submit! {
	InitFunc {
		handler: init
	}
}
fn init() -> MFnResult<'static> {
	Box::pin(async {
		wasm_bindgen_futures::spawn_local(async move {
			let win = window().unwrap();
			let fqdn = format!(
				"{}//{}:{}",
				win.location().protocol().unwrap(),
				win.location().hostname().unwrap(),
				win.location().port().unwrap()
			);

			loop {
				let Ok(ws_stream) = tokio_tungstenite_wasm::connect(format!("{fqdn}/ws")).await
				else {
					error!("Connection to websocket failed! Retrying...");
					continue;
				};

				let mut ws_stream = WSStream { inner: ws_stream };

				for init in inventory::iter::<WSInitFunc> {
					spawn_local(async {
						(init.handler)().await;
					});
				}

				_ = ws_stream.send(WSClientMsg::ReqEntries).await;

				let mut ws_channel = WS_CHANNEL.borrow().subscribe();
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

							for handler in inventory::iter::<WSMsgHandler> {
								let msg = msg.clone();
								spawn_local(async move {
									(handler.handler)(msg).await;
								});
							}
						}
						Ok(msg) = ws_channel.recv() => {
							_ = ws_stream.send(msg).await;
						}
					}
				}

				log!("Disconnected from websocket. Reconnecting...");
			}
		});
	})
}
