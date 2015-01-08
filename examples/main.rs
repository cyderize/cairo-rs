extern crate "cairo-sys" as ffi;

fn main() {
	let version = unsafe { ffi::cairo_version() };
	println!("Cairo version number {}", version);
}