use ffi;

pub struct Pattern {
	inner: *mut ffi::cairo_pattern_t
}

impl Pattern {
	/// Returns the raw underlying pattern
	pub fn as_raw(&self) -> *mut ffi::cairo_pattern_t {
		self.inner
	}
}

impl Drop for Pattern {
	fn drop(&mut self) {
		unsafe {
			ffi::cairo_pattern_destroy(self.inner);
		}
	}
}