#![macro_use]
use ffi;

pub trait RawConversion<T> {
	fn as_raw(&self) -> T;
	fn from_raw(T) -> Self;
}

macro_rules! raw_conversion {
    ($inp: ty, $raw: ty) => (
        impl RawConversion<$raw> for $inp {
            fn as_raw(&self) -> $raw {
				::std::num::FromPrimitive::from_u64(*self as u64).unwrap()
			}
			fn from_raw(raw: $raw) -> $inp {
				::std::num::FromPrimitive::from_u64(raw as u64).unwrap()
			}
        }
    );
}

#[derive(Copy, FromPrimitive)]
pub enum Operator {
	Clear,

	Source,
	Over,
	In,
	Out,
	Atop,

	Dest,
	DestOver,
	DestIn,
	DestOut,
	DestAtop,

	Xor,
	Add,
	Saturate,

	Multiply,
	Screen,
	Overlay,
	Darken,
	Lighten,
	ColorDodge,
	ColorBurn,
	HardLight,
	SoftLight,
	Difference,
	Exclusion,
	HslHue,
	HslSaturation,
	HslColor,
	HslLuminosity
}
raw_conversion!(Operator, ffi::cairo_operator_t);

#[derive(Copy, FromPrimitive)]
pub enum Antialias {
	Default,
	None,
	Gray,
	SubPixel
}
raw_conversion!(Antialias, ffi::cairo_antialias_t);

#[derive(Copy, FromPrimitive)]
pub enum FillRule {
	Winding,
	EvenOdd
}
raw_conversion!(FillRule, ffi::cairo_fill_rule_t);

#[derive(Copy, FromPrimitive)]
pub enum LineCap {
	Butt,
	Round,
	Square
}
raw_conversion!(LineCap, ffi::cairo_line_cap_t);

#[derive(Copy, FromPrimitive)]
pub enum LineJoin {
	Miter,
	Round,
	Bevel
}
raw_conversion!(LineJoin, ffi::cairo_line_join_t);

#[derive(Copy)]
pub struct Rectangle {
	pub x: f64, 
	pub y: f64, 
	pub width: f64,
	pub height: f64,
}

#[derive(Copy, FromPrimitive)]
pub enum TextClusterFlags {
	None,
	Backward
}
raw_conversion!(TextClusterFlags, ffi::cairo_text_cluster_flags_t);

#[derive(Copy, FromPrimitive)]
pub enum FontSlant {
	Normal,
	Italic,
	Oblique
}
raw_conversion!(FontSlant, ffi::cairo_font_slant_t);

#[derive(Copy, FromPrimitive)]
pub enum FontWeight {
	Normal, 
	Bold
}
raw_conversion!(FontWeight, ffi::cairo_font_weight_t);

#[derive(Copy, FromPrimitive)]
pub enum SubPixelOrder {
	Default,
	RedGreenBlue,
	BlueGreenRed,
	VerticalRedGreenBlue,
	VerticalBlueGreenRed,
}
raw_conversion!(SubPixelOrder, ffi::cairo_subpixel_order_t);

#[derive(Copy, FromPrimitive)]
pub enum HintStyle {
	Default,
	None,
	Slight,
	Medium,
	Full
}
raw_conversion!(HintStyle, ffi::cairo_hint_style_t);

#[derive(Copy, FromPrimitive)]
pub enum HintMetrics {
	Default, 
	Off, 
	On
}
raw_conversion!(HintMetrics, ffi::cairo_hint_metrics_t);

#[derive(Copy, FromPrimitive)]
pub enum FontType {
	Toy,
	FreeType,
	Win32,
	Quartz,
	User
}
raw_conversion!(FontType, ffi::cairo_font_type_t);

#[derive(Copy, FromPrimitive)]
pub enum PathDataType {
	MoveTo,
	LineTo,
	CurveTo,
	ClosePath
}
raw_conversion!(PathDataType, ffi::cairo_path_data_type_t);

#[derive(Copy, FromPrimitive)]
pub enum DeviceType {
	Drm,
	Gl,
	Script,
	Xcb,
	Xlib,
	Xml
}
raw_conversion!(DeviceType, ffi::cairo_device_type_t);