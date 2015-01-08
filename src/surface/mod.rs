//! Provides cairo drawing surfaces
use ffi;
use result::{CairoResult, from_raw};
use std::ffi::CString;

pub use self::image::ImageSurface;

mod image;

/// Represents a cairo surface
pub trait Surface {
	/// Return a raw pointer to the underlying surface
	fn as_raw(&self) -> *mut ffi::cairo_surface_t;
	fn get_reference_count(&self) -> uint {
		unsafe {
			ffi::cairo_surface_get_reference_count(self.as_raw()) as uint
		}
	}
	fn write_to_png(&self, filename: &str) -> CairoResult<()> {
		unsafe {
			let filename = CString::from_slice(filename.as_bytes());
			from_raw((), ffi::cairo_surface_write_to_png(self.as_raw(), filename.as_ptr()))
		}
	}
	fn flush(&self) {
		unsafe {
			ffi::cairo_surface_flush(self.as_raw());
		}
	}
}
