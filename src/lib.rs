const VERSION: &'static str = env!("CARGO_PKG_VERSION");

NAME: Option<String> = None;

#[no_mangle]
pub extern fn begin(String name) {
	NAME = Some(name);
}

#[no_mangle]
pub extern fn version() -> &'static str {
	VERSION
}

#[test]
fn print_version() {
	println!("version: {}", version());
}

