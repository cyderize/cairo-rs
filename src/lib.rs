extern crate "cairo-sys" as ffi;
extern crate libc;

use self::surface::Surface;
use self::pattern::Pattern;
use self::common::{RawConversion, Antialias, LineCap, LineJoin, FillRule};

pub mod common;
pub mod version;
pub mod surface;
pub mod pattern;
pub mod format;
pub mod result;

/// Represents a cairo context
pub struct Context {
	inner: *mut ffi::cairo_t
}

impl Context {
	/// Create a new cairo context from the given surface
	pub fn new<S: Surface>(surface: &S) -> Context {
		Context {
			inner: unsafe { ffi::cairo_create(surface.as_raw()) }
		}
	}
	pub fn get_reference_count(&self) -> uint {
		unsafe {
			ffi::cairo_get_reference_count(self.inner) as uint
		}
	}
	pub fn set_source<P: Pattern>(&self, pattern: P) {
		unsafe {
			ffi::cairo_set_source(
				self.inner,
				pattern.as_raw()
			)
		}
	}
	pub fn set_source_rgb(&self, red: f64, green: f64, blue: f64) {
		unsafe {
			ffi::cairo_set_source_rgb(
				self.inner,
				red as libc::c_double,
				green as libc::c_double,
				blue as libc::c_double
			)
		}
	}
	pub fn set_source_rgba(&self, red: f64, green: f64, blue: f64, alpha: f64) {
		unsafe {
			ffi::cairo_set_source_rgba(
				self.inner,
				red as libc::c_double,
				green as libc::c_double,
				blue as libc::c_double,
				alpha as libc::c_double
			)
		}
	}
	pub fn set_tolerance(&self, tolerance: f64) {
		unsafe {
			ffi::cairo_set_tolerance(self.inner, tolerance as libc::c_double);
		}
	}
	pub fn set_antialias(&self, antialias: Antialias) {
		unsafe {
			ffi::cairo_set_antialias(self.inner, antialias.as_raw());
		}
	}
	pub fn set_fill_rule(&self, fill_rule: FillRule) {
		unsafe {
			ffi::cairo_set_fill_rule(self.inner, fill_rule.as_raw());
		}
	}
	pub fn set_line_width(&self, width: f64) {
		unsafe {
			ffi::cairo_set_line_width(self.inner, width as libc::c_double);
		}
	}
	pub fn set_line_cap(&self, line_cap: LineCap) {
		unsafe {
			ffi::cairo_set_line_cap(self.inner, line_cap.as_raw());
		}
	}
	pub fn set_line_join(&self, line_join: LineJoin) {
		unsafe {
			ffi::cairo_set_line_join(self.inner, line_join.as_raw());
		}
	}
	pub fn set_dash(&self, dashes: &[f64], offset: f64) {
		let vec: Vec<libc::c_double> = dashes.iter()
			.map(|&x| x as libc::c_double)
			.collect();
		let len = vec.len();
		unsafe {
			ffi::cairo_set_dash(self.inner, vec.as_ptr(), len as libc::c_int, offset as libc::c_double);
		}
	}
	pub fn set_miter_limit(&self, limit: f64) {
		unsafe {
			ffi::cairo_set_miter_limit(self.inner, limit as libc::c_double);
		}
	}
}

impl Clone for Context {
	fn clone(&self) -> Context {
		Context {
			inner: unsafe { ffi::cairo_reference(self.inner) }
		}
	}
}

impl Drop for Context {
	fn drop(&mut self) {
		unsafe {
			ffi::cairo_destroy(self.inner)
		}
	}
}