use ffi;

pub enum Format {
	Invalid,
	ARGB32, 
	RGB24, 
	A8, 
	A1, 
	RGB16_565
}

impl Format {
	pub fn as_raw(&self) -> ffi::cairo_format_t {
		match *self {
			Format::Invalid => ffi::cairo_format_t::CAIRO_FORMAT_INVALID,
			Format::ARGB32 => ffi::cairo_format_t::CAIRO_FORMAT_ARGB32,
			Format::RGB24 => ffi::cairo_format_t::CAIRO_FORMAT_RGB24,
			Format::A8 => ffi::cairo_format_t::CAIRO_FORMAT_A8,
			Format::A1 => ffi::cairo_format_t::CAIRO_FORMAT_A1,
			Format::RGB16_565 => ffi::cairo_format_t::CAIRO_FORMAT_RGB16_565,
		}
	}
}
