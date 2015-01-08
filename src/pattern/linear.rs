use ffi;
use libc;
use pattern::Pattern;

/// A linear gradient pattern
pub struct LinearPattern {
	inner: *mut ffi::cairo_pattern_t
}

impl LinearPattern {
	pub fn new(x0: f64, y0: f64, x1: f64, y1: f64) -> LinearPattern {
		LinearPattern {
			inner: unsafe {
				ffi::cairo_pattern_create_linear(
					x0 as libc::c_double,
					y0 as libc::c_double,
					x1 as libc::c_double,
					y1 as libc::c_double
				)
			}
		}
	}
}

impl Pattern for LinearPattern {
	fn as_raw(&self) -> *mut ffi::cairo_pattern_t {
		self.inner
	}
}

impl Clone for LinearPattern {
	fn clone(&self) -> LinearPattern {
		LinearPattern {
			inner: unsafe {
				ffi::cairo_pattern_reference(self.inner)
			}
		}
	}
}

impl Drop for LinearPattern {
	fn drop(&mut self) {
		unsafe {
			ffi::cairo_pattern_destroy(self.inner);
		}
	}
}