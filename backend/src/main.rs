#![feature(time_systemtime_limits)]

mod env;
mod handler;
mod rcon;
mod ws;
use crate::handler::{InitFunc, RouteCollector};
use rocket::{Build, Rocket, Route, response::Responder, tokio::spawn};

#[derive(rust_embed::RustEmbed)]
#[folder = "../dist"]
struct Asset;

struct EmbeddedFile {
	bytes: Vec<u8>,
	content_type: rocket::http::ContentType,
}

impl<'a> Responder<'a, 'static> for EmbeddedFile {
	fn respond_to(self, _: &'a rocket::Request<'_>) -> rocket::response::Result<'static> {
		rocket::Response::build()
			.header(self.content_type)
			.sized_body(self.bytes.len(), std::io::Cursor::new(self.bytes))
			.ok()
	}
}

#[rocket::launch]
async fn rocket() -> Rocket<Build> {
	for init in inventory::iter::<InitFunc> {
		spawn(async {
			(init.init)().await;
		});
	}

	rocket::build()
		.configure(rocket::Config {
			port: 8999,
			..Default::default()
		})
		.mount(
			"/",
			rocket::routes![route_index]
				.into_iter()
				.chain(
					inventory::iter::<RouteCollector>
						.into_iter()
						.flat_map(|r| (r.handler)()),
				)
				.collect::<Vec<Route>>(),
		)
}

#[rocket::get("/<path..>")]
fn route_index(path: std::path::PathBuf) -> Option<EmbeddedFile> {
	let path_str = path.to_string_lossy();
	let mut path_str = if path_str.is_empty() {
		"index.html".to_string()
	} else {
		path_str.to_string()
	};

	let file: Option<rust_embed::EmbeddedFile> = match Asset::get(&path_str) {
		Some(f) => Some(f),
		None => {
			path_str = format!("{path_str}/index.html");
			Asset::get(&path_str)
		}
	};
	file.as_ref()?;
	let file = file.unwrap();
	let mime = mime_guess::from_path(&path_str).first_or_octet_stream();
	let ct = rocket::http::ContentType::parse_flexible(mime.essence_str())
		.unwrap_or(rocket::http::ContentType::Binary);

	Some(EmbeddedFile {
		bytes: file.data.into_owned(),
		content_type: ct,
	})
}
