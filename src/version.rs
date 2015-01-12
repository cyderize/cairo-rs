//! Utility functions for dealing with cairo versions

use ffi;
use std::ffi::c_str_to_bytes;
use std::str::from_utf8;

/// Returns the cairo version as an integer
pub fn version() -> isize {
	unsafe {
		ffi::cairo_version() as isize
	}
}

/// Returns the cairo version as a String
pub fn version_string() -> String {
	unsafe {
		let version = ffi::cairo_version_string();
		let bytes = c_str_to_bytes(&version);
		from_utf8(bytes).unwrap().to_string()
	}
}

#[test]
fn test_version() {
	println!("cairo version: {}", version());
	assert!(version() >= 11400);
}
