mod entries;
mod handler;
use crate::{
	handler::RouteCollector,
	ws::handler::{WSInitFunc, WSMsgHandler},
};
use futures_util::StreamExt;
use lazy_static::lazy_static;
use rocket::{
	Shutdown,
	tokio::{spawn, sync::broadcast},
};
use shared::{WSClientMsg, WSServerMsg, stream::WSStream};

lazy_static! {
	pub static ref WS_GLOBAL_CHANNEL: broadcast::Sender<WSServerMsg> = broadcast::channel(100).0;
}

inventory::submit! {
	RouteCollector {
		handler: || { rocket::routes![route_ws] }
	}
}
#[rocket::get("/ws")]
fn route_ws(mut shutdown: Shutdown, ws: rocket_ws::WebSocket) -> rocket_ws::Channel<'static> {
	ws.channel(move |ws_stream| {
		Box::pin(async move {
			let mut ws_global_channel_rx = WS_GLOBAL_CHANNEL.subscribe();

			let mut ws_stream = WSStream { inner: ws_stream };

			let ws_channel = broadcast::channel::<WSServerMsg>(100).0;

			for init in inventory::iter::<WSInitFunc> {
				let ws_channel = ws_channel.clone();
				spawn(async {
					(init.init)(ws_channel).await;
				});
			}

			let mut ws_channel_rx = ws_channel.subscribe();

			loop {
				rocket::tokio::select! {
					_ = &mut shutdown => {
						break;
					}
					msg = ws_stream.next() => {
						let Some(msg) = msg else {
							break;
						};

						let Ok(msg) = msg else {
							continue;
						};

						let Ok(msg) = serde_json::from_str::<WSClientMsg>(&msg.to_string()) else {
							continue;
						};

						for handler in inventory::iter::<WSMsgHandler> {
							let ws_channel = ws_channel.clone();
							let msg = msg.clone();
							spawn(async {
								(handler.handler)(ws_channel, msg).await;
							});
						}
					}
					Ok(msg) = ws_channel_rx.recv() => {
						_ = ws_stream.send(msg).await;
					}
					Ok(msg) = ws_global_channel_rx.recv() => {
						_ = ws_stream.send(msg).await;
					}
				}
			}

			Ok(())
		})
	})
}
