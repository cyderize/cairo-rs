use ffi;

pub struct Pattern {
	inner: *mut ffi::cairo_pattern_t
}