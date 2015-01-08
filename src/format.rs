use ffi;
use common::RawConversion;

/// Represents a pixel format
#[derive(Copy, FromPrimitive)]
pub enum Format {
	/// Invalid format
	Invalid = -1,
	/// 32-bit ARGB pixels
	AlphaRedGreenBlue32 = 0,
	/// 24-bit RGB pixels where the first 8 bits are unused
	RedGreenBlue24,
	/// 8-bit alpha pixels
	Alpha8,
	/// 1-bit alpha pixels
	Alpha1, 
	/// 16-bit RGB with 5-bit red, 6-bit green, 5-bit blue
	RedGreenBlue16
}
raw_conversion!(Format, ffi::cairo_format_t);