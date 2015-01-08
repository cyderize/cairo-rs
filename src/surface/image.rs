use ffi;
use libc;
use format::Format;
use surface::Surface;
use common::RawConversion;

/// Represents a cairo image surface
pub struct ImageSurface {
	inner: *mut ffi::cairo_surface_t
}

impl ImageSurface {
	/// Creates a new ImageSurface
	pub fn new(format: Format, width: int, height: int) -> ImageSurface {
		ImageSurface {
			inner: unsafe { ffi::cairo_image_surface_create(
				format.as_raw(), 
				width as libc::c_int, 
				height as libc::c_int
			)}
		}
	}
	/// Creates a new ImageSurface for the provided pixel data
	pub fn for_data(data: &mut [u8], format: Format, width: int, height: int, stride: int) -> ImageSurface {
		ImageSurface {
			inner: unsafe { ffi::cairo_image_surface_create_for_data(
				data.as_mut_ptr() as *mut libc::c_uchar, 
				format.as_raw(), 
				width as libc::c_int, 
				height as libc::c_int, 
				stride as libc::c_int
			)}
		}
	}
}

impl Surface for ImageSurface {
	fn as_raw(&self) -> *mut ffi::cairo_surface_t {
		self.inner
	}
}

impl Clone for ImageSurface {
	fn clone(&self) -> ImageSurface {
		ImageSurface {
			inner: unsafe {
				ffi::cairo_surface_reference(self.inner)
			}
		}
	}
}

impl Drop for ImageSurface {
	fn drop(&mut self) {
		unsafe {
			ffi::cairo_surface_destroy(self.as_raw());
		}
	}
}
