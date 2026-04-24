use crate::handler::{InitFunc, MFnResult};
use web_sys::{Notification, NotificationOptions};

inventory::submit! {
	InitFunc {
		handler: init
	}
}
fn init() -> MFnResult<'static> {
	Box::pin(async {
		_ = Notification::request_permission();
	})
}

pub fn show(title: &str, body: &str) {
	let opts = NotificationOptions::new();
	opts.set_icon("/favicon.ico");
	opts.set_body(body);

	Notification::new_with_options(title, &opts).unwrap();
}
