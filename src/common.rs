use ffi;
use std::num::FromPrimitive;

pub trait RawConversion<T> {
	fn as_raw(&self) -> T;
	fn from_raw(T) -> Self;
}

macro_rules! raw_conversion {
    ($inp: ty, $raw: ty) => (
        impl RawConversion<$raw> for $inp {
            fn as_raw(&self) -> $raw {
				FromPrimitive::from_u64(*self as u64).unwrap()
			}
			fn from_raw(raw: $raw) -> $inp {
				FromPrimitive::from_u64(raw as u64).unwrap()
			}
        }
    );
}

raw_conversion!(Operator, ffi::cairo_operator_t);
raw_conversion!(Antialias, ffi::cairo_antialias_t);
raw_conversion!(FillRule, ffi::cairo_fill_rule_t);
raw_conversion!(LineCap, ffi::cairo_line_cap_t);
raw_conversion!(LineJoin, ffi::cairo_line_join_t);

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

#[derive(Copy, FromPrimitive)]
pub enum Antialias {
	Default,
	None,
	Gray,
	SubPixel
}

#[derive(Copy, FromPrimitive)]
pub enum FillRule {
	Winding,
	EvenOdd
}

#[derive(Copy, FromPrimitive)]
pub enum LineCap {
	Butt,
	Round,
	Square
}

#[derive(Copy, FromPrimitive)]
pub enum LineJoin {
	Miter,
	Round,
	Bevel
}

#[derive(Copy)]
pub struct Rectangle {
	pub x: f64, 
	pub y: f64, 
	pub width: f64,
	pub height: f64,
}
