use crate::handler::{InitFunc, MFnResult};
use std::sync::OnceLock;

pub static RCON_PASSWORD: OnceLock<String> = OnceLock::new();

inventory::submit! {
	InitFunc {
		handler: init
	}
}
fn init() -> MFnResult<'static> {
	Box::pin(async {
		_ = dotenvy::dotenv();

		_ = RCON_PASSWORD.set(std::env::var("RCON_PASSWORD").unwrap_or_default());
	})
}
