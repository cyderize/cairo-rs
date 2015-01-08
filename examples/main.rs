extern crate "cairo-sys" as ffi;

use std::c_str::CString;

fn main() {
	let cstring = unsafe { CString::new(ffi::cairo_version_string(), false) };
	println!("Cairo {}", cstring);
}