use std::path::PathBuf;

fn main() {
	println!("cargo::rerun-if-changed=.");
	println!("cargo::rerun-if-changed=../static");

	let target = std::env::var("TARGET").unwrap();
	if target.contains("wasm32") {
		return;
	}

	let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

	let static_dir = PathBuf::from("../static");
	let dist_dir = PathBuf::from("../dist");

	_ = fs_extra::remove_items(&[&dist_dir]);
	_ = fs_extra::copy_items(
		&[&static_dir],
		&dist_dir,
		&fs_extra::dir::CopyOptions {
			copy_inside: true,
			..Default::default()
		},
	);

	let wasm_out_dir = dist_dir.join("pkg");
	println!("cargo:out_dir={}", wasm_out_dir.to_str().unwrap());

	let wasm_target_dir = out_dir.join("wasm-target");

	let mut wasm_pack = std::process::Command::new("wasm-pack");
	wasm_pack
		.envs([("CARGO_TARGET_DIR", wasm_target_dir.to_str().unwrap())])
		.args([
			"build",
			//#[cfg(debug_assertions)]
			//"--dev",
			#[cfg(not(debug_assertions))]
			"--release",
			"--no-opt",
			"--no-pack",
			"--no-typescript",
			"--weak-refs",
			"--target",
			"web",
			"--out-dir",
			wasm_out_dir.to_str().unwrap(),
		]);
	let status = wasm_pack
		.status()
		.expect("failed to run wasm-pack (is it installed and on PATH?)");
	assert!(status.success(), "wasm-pack build failed");

	#[cfg(not(debug_assertions))]
	{
		use std::sync::Arc;
		use swc_common::errors::Emitter;

		struct JsNoopEmitter;
		impl Emitter for JsNoopEmitter {
			fn emit(&mut self, _: &mut swc_common::errors::DiagnosticBuilder) {}
		}
		let js_compiler =
			swc::Compiler::new(swc_common::sync::Lrc::new(swc_common::SourceMap::default()));
		let js_handler =
			swc_common::errors::Handler::with_emitter(false, false, Box::new(JsNoopEmitter));

		for entry in glob::glob(format!("{}/**/*.*", dist_dir.to_str().unwrap()).as_str())
			.expect("Failed to read glob pattern")
		{
			match entry {
				Ok(path) => {
					let extension = path.extension().unwrap_or_default().to_str().unwrap();
					if extension != "css"
						&& extension != "html"
						&& extension != "json"
						&& extension != "js"
					{
						continue;
					}

					let Ok(content) = std::fs::read_to_string(&path) else {
						continue;
					};

					_ = std::fs::write(
						&path,
						match extension {
							"css" => {
								lightningcss::stylesheet::StyleSheet::parse(
									&content,
									lightningcss::stylesheet::ParserOptions::default(),
								)
								.unwrap()
								.to_css(lightningcss::printer::PrinterOptions {
									minify: true,
									targets: lightningcss::targets::Targets::default(),
									..Default::default()
								})
								.unwrap()
								.code
							}
							"html" => minify::html::minify(&content),
							"json" => minify::json::minify(&content),
							"js" => swc_common::GLOBALS.set(&Default::default(), || {
								js_compiler
									.minify(
										js_compiler.cm.new_source_file(
											Arc::new(swc_common::FileName::Custom(
												path.file_name()
													.unwrap()
													.to_str()
													.unwrap()
													.to_string(),
											)),
											content,
										),
										&js_handler,
										&swc::config::JsMinifyOptions::default(),
										swc::JsMinifyExtras::default(),
									)
									.unwrap()
									.code
							}),
							_ => unimplemented!(),
						},
					);
				}
				Err(e) => println!("{:?}", e),
			}
		}
	}
}
