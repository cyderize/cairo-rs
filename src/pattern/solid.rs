use ffi;
use libc;
use pattern::Pattern;
use common::RawConversion;

/// A solid pattern
pub struct SolidPattern {
	inner: *mut ffi::cairo_pattern_t
}

impl SolidPattern {
	pub fn new_rgb(red: f64, green: f64, blue: f64) -> SolidPattern {
		SolidPattern {
			inner: unsafe {
				ffi::cairo_pattern_create_rgb(
					red as libc::c_double,
					green as libc::c_double,
					blue as libc::c_double
				)
			}
		}
	}
	
	pub fn new_rgba(red: f64, green: f64, blue: f64, alpha: f64) -> SolidPattern {
		SolidPattern {
			inner: unsafe {
				ffi::cairo_pattern_create_rgba(
					red as libc::c_double,
					green as libc::c_double,
					blue as libc::c_double,
					alpha as libc::c_double
				)
			}
		}
	}
}

impl Pattern for SolidPattern { }

impl RawConversion<*mut ffi::cairo_pattern_t> for SolidPattern {
	fn as_raw(&self) -> *mut ffi::cairo_pattern_t {
		self.inner
	}
	fn from_raw(raw: *mut ffi::cairo_pattern_t) -> SolidPattern {
		SolidPattern {
			inner: raw
		}
	}
}

impl Clone for SolidPattern {
	fn clone(&self) -> SolidPattern {
		SolidPattern {
			inner: unsafe {
				ffi::cairo_pattern_reference(self.inner)
			}
		}
	}
}

impl Drop for SolidPattern {
	fn drop(&mut self) {
		unsafe {
			ffi::cairo_pattern_destroy(self.inner);
		}
	}
}