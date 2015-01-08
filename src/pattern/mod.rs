use ffi;
use libc;
use common::RawConversion;

pub use self::solid::SolidPattern;
pub use self::surface::SurfacePattern;
pub use self::linear::LinearPattern;
pub use self::radial::RadialPattern;

mod solid;
mod surface;
mod linear;
mod radial;

/// A trait for cairo patterns
pub trait Pattern: RawConversion<*mut ffi::cairo_pattern_t> {
	fn get_reference_count(&self) -> uint {
		unsafe {
			ffi::cairo_pattern_get_reference_count(self.as_raw()) as uint
		}
	}
	fn add_color_stop_rgb(&self, offset: f64, red: f64, green: f64, blue: f64) {
		unsafe {
			ffi::cairo_pattern_add_color_stop_rgb(
				self.as_raw(),
				offset as libc::c_double,
				red as libc::c_double,
				green as libc::c_double,
				blue as libc::c_double
			);
		}
	}
	fn add_color_stop_rgba(&self, offset: f64, red: f64, green: f64, blue: f64, alpha: f64) {
		unsafe {
			ffi::cairo_pattern_add_color_stop_rgba(
				self.as_raw(),
				offset as libc::c_double,
				red as libc::c_double,
				green as libc::c_double,
				blue as libc::c_double,
				alpha as libc::c_double
			);
		}
	}
}
