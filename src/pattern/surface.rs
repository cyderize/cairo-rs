use ffi;
use surface::Surface;
use pattern::Pattern;

/// A surface pattern
pub struct SurfacePattern {
	inner: *mut ffi::cairo_pattern_t
}

impl SurfacePattern {
	pub fn new<S: Surface>(surface: S) -> SurfacePattern {
		SurfacePattern {
			inner: unsafe { ffi::cairo_pattern_create_for_surface(surface.as_raw()) }
		}
	}
}

impl Pattern for SurfacePattern {
	fn as_raw(&self) -> *mut ffi::cairo_pattern_t {
		self.inner
	}
}

impl Clone for SurfacePattern {
	fn clone(&self) -> SurfacePattern {
		SurfacePattern {
			inner: unsafe {
				ffi::cairo_pattern_reference(self.inner)
			}
		}
	}
}

impl Drop for SurfacePattern {
	fn drop(&mut self) {
		unsafe {
			ffi::cairo_pattern_destroy(self.inner);
		}
	}
}