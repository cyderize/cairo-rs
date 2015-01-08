extern crate "cairo-sys" as ffi;
extern crate libc;

use self::surface::Surface;
use self::pattern::Pattern;

pub mod version;
pub mod surface;
pub mod pattern;
pub mod format;
pub mod result;

/// Represents a cairo context
pub struct Context {
	cr: *mut ffi::cairo_t
}

impl Context {
	/// Create a new cairo context from the given surface
	pub fn new<S: Surface>(surface: &S) -> Context {
		Context {
			cr: unsafe { ffi::cairo_create(surface.as_raw()) }
		}
	}
	pub fn set_source<P: Pattern>(&self, pattern: P) {
		unsafe {
			ffi::cairo_set_source(
				self.cr,
				pattern.as_raw()
			)
		}
	}
	pub fn set_source_rgb(&self, red: f64, green: f64, blue: f64) {
		unsafe {
			ffi::cairo_set_source_rgb(
				self.cr,
				red as libc::c_double,
				green as libc::c_double,
				blue as libc::c_double
			)
		}
	}
	pub fn set_source_rgba(&self, red: f64, green: f64, blue: f64, alpha: f64) {
		unsafe {
			ffi::cairo_set_source_rgba(
				self.cr,
				red as libc::c_double,
				green as libc::c_double,
				blue as libc::c_double,
				alpha as libc::c_double
			)
		}
	}
	
}

impl Drop for Context {
	fn drop(&mut self) {
		unsafe {
			ffi::cairo_destroy(self.cr)
		}
	}
}