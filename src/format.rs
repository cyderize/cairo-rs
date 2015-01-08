use ffi;

/// Represents a pixel format
#[derive(Copy)]
pub enum Format {
	/// Invalid format
	Invalid,
	/// 32-bit ARGB pixels
	AlphaRedGreenBlue32,
	/// 24-bit RGB pixels where the first 8 bits are unused
	RedGreenBlue24,
	/// 8-bit alpha pixels
	Alpha8,
	/// 1-bit alpha pixels
	Alpha1, 
	/// 16-bit RGB with 5-bit red, 6-bit green, 5-bit blue
	RedGreenBlue16
}

impl Format {
	/// Returns the raw equivalent of this format
	pub fn as_raw(&self) -> ffi::cairo_format_t {
		match *self {
			Format::Invalid => ffi::cairo_format_t::CAIRO_FORMAT_INVALID,
			Format::AlphaRedGreenBlue32 => ffi::cairo_format_t::CAIRO_FORMAT_ARGB32,
			Format::RedGreenBlue24 => ffi::cairo_format_t::CAIRO_FORMAT_RGB24,
			Format::Alpha8 => ffi::cairo_format_t::CAIRO_FORMAT_A8,
			Format::Alpha1 => ffi::cairo_format_t::CAIRO_FORMAT_A1,
			Format::RedGreenBlue16 => ffi::cairo_format_t::CAIRO_FORMAT_RGB16_565,
		}
	}
}
