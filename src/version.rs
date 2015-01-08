//! Utility functions for dealing with cairo versions

use ffi;
use std::ffi::c_str_to_bytes;
use std::str::from_utf8;

/// Returns the cairo version as an integer
pub fn version() -> int {
	unsafe {
		ffi::cairo_version() as int
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