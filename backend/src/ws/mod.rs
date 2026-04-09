mod entries;
use crate::handler::{RouteCollector, WSInitFunc, WSMsgHandler};
use futures_util::StreamExt;
use lazy_static::lazy_static;
use rocket::{Shutdown, tokio::sync::broadcast};
use shared::{WSClientMsg, WSServerMsg, stream::WSStream};

lazy_static! {
	pub static ref WS_CHANNEL: broadcast::Sender<WSServerMsg> = broadcast::channel(100).0;
}

inventory::submit! {
	RouteCollector {
		handler: || { rocket::routes![route_ws] }
	}
}
#[rocket::get("/ws")]
fn route_ws(mut shutdown: Shutdown, ws: rocket_ws::WebSocket) -> rocket_ws::Channel<'static> {
	let mut ws_channel = WS_CHANNEL.subscribe();

	ws.channel(move |ws_stream| {
		Box::pin(async move {
			let mut ws_stream = WSStream { inner: ws_stream };

			for init in inventory::iter::<WSInitFunc> {
				(init.init)(&mut ws_stream).await;
			}

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
							(handler.handler)(&mut ws_stream, &msg).await;
						}
					}
					Ok(msg) = ws_channel.recv() => {
						_ = ws_stream.send(msg).await;
					}
				}
			}

			Ok(())
		})
	})
}
