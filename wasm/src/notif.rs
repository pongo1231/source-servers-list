use shared::handler::InitFunc;
use web_sys::{Notification, NotificationOptions};

inventory::submit! {
	InitFunc {
		init
	}
}

fn init() {
	_ = Notification::request_permission();
}

pub fn show(title: &str, body: &str) {
	let opts = NotificationOptions::new();
	opts.set_icon("favicon.ico");
	opts.set_body(body);

	Notification::new_with_options(title, &opts).unwrap();
}
