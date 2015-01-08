use ffi;
use libc;
use pattern::Pattern;

pub struct RadialPattern {
	inner: *mut ffi::cairo_pattern_t
}

/// A radial gradient pattern
impl RadialPattern {
	pub fn new(cx0: f64, cy0: f64, radius0: f64, cx1: f64, cy1: f64, radius1: f64) -> RadialPattern {
		RadialPattern {
			inner: unsafe {
				ffi::cairo_pattern_create_radial(
					cx0 as libc::c_double,
					cy0 as libc::c_double,
					radius0 as libc::c_double,
					cx1 as libc::c_double,
					cy1 as libc::c_double,
					radius1 as libc::c_double
				)
			}
		}
	}
}

impl Pattern for RadialPattern {
	fn as_raw(&self) -> *mut ffi::cairo_pattern_t {
		self.inner
	}
}

impl Clone for RadialPattern {
	fn clone(&self) -> RadialPattern {
		RadialPattern {
			inner: unsafe {
				ffi::cairo_pattern_reference(self.inner)
			}
		}
	}
}

impl Drop for RadialPattern {
	fn drop(&mut self) {
		unsafe {
			ffi::cairo_pattern_destroy(self.inner);
		}
	}
}