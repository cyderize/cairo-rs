extern crate "cairo-sys" as ffi;
extern crate libc;

use std::ptr;
use std::ffi::CString;

use self::surface::Surface;
use self::pattern::Pattern;
use self::common::{RawConversion, Antialias, LineCap, LineJoin, FillRule, FontSlant, FontWeight};

pub mod common;
pub mod version;
pub mod surface;
pub mod pattern;
pub mod format;
pub mod result;

/// Represents a cairo context
pub struct Context {
	inner: *mut ffi::cairo_t
}

impl Context {
	/// Create a new cairo context from the given surface
	pub fn new<S: Surface>(surface: &S) -> Context {
		Context {
			inner: unsafe { ffi::cairo_create(surface.as_raw()) }
		}
	}
	pub fn get_reference_count(&self) -> usize {
		unsafe {
			ffi::cairo_get_reference_count(self.inner) as usize
		}
	}
	pub fn scale(&self, sx: f64, sy: f64) {
		unsafe {
			ffi::cairo_scale(self.inner, sx, sy);
		}
	}
	pub fn set_source<P: Pattern>(&self, pattern: P) {
		unsafe {
			ffi::cairo_set_source(
				self.inner,
				pattern.as_raw()
			);
		}
	}
	pub fn set_source_rgb(&self, red: f64, green: f64, blue: f64) {
		unsafe {
			ffi::cairo_set_source_rgb(
				self.inner,
				red as libc::c_double,
				green as libc::c_double,
				blue as libc::c_double
			);
		}
	}
	pub fn set_source_rgba(&self, red: f64, green: f64, blue: f64, alpha: f64) {
		unsafe {
			ffi::cairo_set_source_rgba(
				self.inner,
				red as libc::c_double,
				green as libc::c_double,
				blue as libc::c_double,
				alpha as libc::c_double
			);
		}
	}
	pub fn set_tolerance(&self, tolerance: f64) {
		unsafe {
			ffi::cairo_set_tolerance(self.inner, tolerance as libc::c_double);
		}
	}
	pub fn set_antialias(&self, antialias: Antialias) {
		unsafe {
			ffi::cairo_set_antialias(self.inner, antialias.as_raw());
		}
	}
	pub fn set_fill_rule(&self, fill_rule: FillRule) {
		unsafe {
			ffi::cairo_set_fill_rule(self.inner, fill_rule.as_raw());
		}
	}
	pub fn set_line_width(&self, width: f64) {
		unsafe {
			ffi::cairo_set_line_width(self.inner, width as libc::c_double);
		}
	}
	pub fn set_line_cap(&self, line_cap: LineCap) {
		unsafe {
			ffi::cairo_set_line_cap(self.inner, line_cap.as_raw());
		}
	}
	pub fn set_line_join(&self, line_join: LineJoin) {
		unsafe {
			ffi::cairo_set_line_join(self.inner, line_join.as_raw());
		}
	}
	pub fn set_dash(&self, dashes: &[f64], offset: f64) {
		let vec: Vec<libc::c_double> = dashes.iter()
			.map(|&x| x as libc::c_double)
			.collect();
		let len = vec.len();
		unsafe {
			ffi::cairo_set_dash(self.inner, vec.as_ptr(), len as libc::c_int, offset as libc::c_double);
		}
	}
	pub fn set_miter_limit(&self, limit: f64) {
		unsafe {
			ffi::cairo_set_miter_limit(self.inner, limit as libc::c_double);
		}
	}

	// Paths
	pub fn new_path(&self) {
		unsafe {
			ffi::cairo_new_path(self.inner);
		}
	}

	pub fn new_sub_path(&self) {
		unsafe {
			ffi::cairo_new_sub_path(self.inner);
		}
	}

	pub fn close_path(&self) {
		unsafe {
			ffi::cairo_close_path(self.inner);
		}
	}

	pub fn arc(&self, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64) {
		unsafe {
			ffi::cairo_arc(self.inner, xc as libc::c_double, yc as libc::c_double, radius as libc::c_double, angle1 as libc::c_double, angle2 as libc::c_double);
		}
	}

	pub fn arc_negative(&self, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64) {
		unsafe {
			ffi::cairo_arc_negative(self.inner, xc as libc::c_double, yc as libc::c_double, radius as libc::c_double, angle1 as libc::c_double, angle2 as libc::c_double);
		}
	}

	pub fn line_to(&self, x: f64, y: f64) {
		unsafe {
			ffi::cairo_line_to(self.inner, x as libc::c_double, y as libc::c_double);
		}
	}

	pub fn move_to(&self, x: f64, y: f64) {
		unsafe {
			ffi::cairo_move_to(self.inner, x as libc::c_double, y as libc::c_double);
		}
	}

	pub fn rectangle(&self, x: f64, y: f64, width: f64, height: f64) {
		unsafe {
			ffi::cairo_rectangle(self.inner, x as libc::c_double, y as libc::c_double, width as libc::c_double, height as libc::c_double);
		}
	}

	// Drawing
	pub fn fill(&self) {
		unsafe {
			ffi::cairo_fill(self.inner);
		}
	}

	pub fn fill_preserve(&self) {
		unsafe {
			ffi::cairo_fill_preserve(self.inner);
		}
	}

	pub fn stroke(&self) {
		unsafe {
			ffi::cairo_fill(self.inner);
		}
	}

	pub fn stroke_preserve(&self) {
		unsafe {
			ffi::cairo_fill_preserve(self.inner);
		}
	}

	// Simple Text
	pub fn select_font_face(&self, family: &str, slant: FontSlant, weight: FontWeight) {
		let buf = CString::from_slice(family.as_bytes()).as_ptr();
		unsafe {
			ffi::cairo_select_font_face(self.inner, buf, slant.as_raw(), weight.as_raw());
		}
	}

	pub fn set_font_size(&self, size: f64) {
		unsafe {
			ffi::cairo_set_font_size(self.inner, size as libc::c_double);
		}
	}

	pub fn font_extents(&self) -> ffi::cairo_font_extents_t {
		let mut extents: ffi::cairo_font_extents_t = unsafe { std::mem::uninitialized::<ffi::cairo_font_extents_t>() };
		unsafe {
			ffi::cairo_font_extents(self.inner, &mut extents);
		}
		return extents
	}

	pub fn text_extents(&self, utf8: &str) -> ffi::cairo_text_extents_t {
		let mut extents: ffi::cairo_text_extents_t = unsafe { std::mem::uninitialized::<ffi::cairo_text_extents_t>() };
		let buf = CString::from_slice(utf8.as_bytes()).as_ptr();
		unsafe {
			ffi::cairo_text_extents(self.inner, buf, &mut extents);
		}
		return extents
	}

	pub fn show_text(&self, utf8: &str) {
		let buf = CString::from_slice(utf8.as_bytes()).as_ptr();
		unsafe {
			ffi::cairo_show_text(self.inner, buf);
		}
	}
}

impl Clone for Context {
	fn clone(&self) -> Context {
		Context {
			inner: unsafe { ffi::cairo_reference(self.inner) }
		}
	}
}

impl Drop for Context {
	fn drop(&mut self) {
		unsafe {
			ffi::cairo_destroy(self.inner);
		}
	}
}