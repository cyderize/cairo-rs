#![allow(non_camel_case_types)]
#![allow(missing_copy_implementations)]

extern crate libc;

use libc::{c_int, c_char, c_double, c_void, c_uchar, c_uint, c_ulong};
use std::mem;

pub type cairo_bool_t = c_int;

#[repr(C)] pub struct cairo_t;
#[repr(C)] pub struct cairo_surface_t;
#[repr(C)] pub struct cairo_device_t;

#[repr(C)]
pub struct cairo_matrix_t {
	pub xx: c_double, 
	pub yx: c_double,
	pub xy: c_double, 
	pub yy: c_double,
	pub x0: c_double, 
	pub y0: c_double,
}

#[repr(C)] pub struct cairo_pattern_t;

pub type cairo_destroy_func_t = extern fn(*mut c_void);

#[repr(C)]
pub struct cairo_user_data_key_t {
	pub unused: c_int,
}

#[repr(C)]
#[derive(FromPrimitive)]
pub enum cairo_status_t {
	CAIRO_STATUS_SUCCESS = 0,

	CAIRO_STATUS_NO_MEMORY,
	CAIRO_STATUS_INVALID_RESTORE,
	CAIRO_STATUS_INVALID_POP_GROUP,
	CAIRO_STATUS_NO_CURRENT_POINT,
	CAIRO_STATUS_INVALID_MATRIX,
	CAIRO_STATUS_INVALID_STATUS,
	CAIRO_STATUS_NULL_POINTER,
	CAIRO_STATUS_INVALID_STRING,
	CAIRO_STATUS_INVALID_PATH_DATA,
	CAIRO_STATUS_READ_ERROR,
	CAIRO_STATUS_WRITE_ERROR,
	CAIRO_STATUS_SURFACE_FINISHED,
	CAIRO_STATUS_SURFACE_TYPE_MISMATCH,
	CAIRO_STATUS_PATTERN_TYPE_MISMATCH,
	CAIRO_STATUS_INVALID_CONTENT,
	CAIRO_STATUS_INVALID_FORMAT,
	CAIRO_STATUS_INVALID_VISUAL,
	CAIRO_STATUS_FILE_NOT_FOUND,
	CAIRO_STATUS_INVALID_DASH,
	CAIRO_STATUS_INVALID_DSC_COMMENT,
	CAIRO_STATUS_INVALID_INDEX,
	CAIRO_STATUS_CLIP_NOT_REPRESENTABLE,
	CAIRO_STATUS_TEMP_FILE_ERROR,
	CAIRO_STATUS_INVALID_STRIDE,
	CAIRO_STATUS_FONT_TYPE_MISMATCH,
	CAIRO_STATUS_USER_FONT_IMMUTABLE,
	CAIRO_STATUS_USER_FONT_ERROR,
	CAIRO_STATUS_NEGATIVE_COUNT,
	CAIRO_STATUS_INVALID_CLUSTERS,
	CAIRO_STATUS_INVALID_SLANT,
	CAIRO_STATUS_INVALID_WEIGHT,
	CAIRO_STATUS_INVALID_SIZE,
	CAIRO_STATUS_USER_FONT_NOT_IMPLEMENTED,
	CAIRO_STATUS_DEVICE_TYPE_MISMATCH,
	CAIRO_STATUS_DEVICE_ERROR,

	CAIRO_STATUS_LAST_STATUS
}

#[repr(C)]
pub enum cairo_content_t {
	CAIRO_CONTENT_COLOR = 0x1000,
	CAIRO_CONTENT_ALPHA = 0x2000,
	CAIRO_CONTENT_COLOR_ALPHA = 0x3000
}

pub type cairo_write_func_t = extern fn(closure: *mut c_void, data: *const c_uchar, length: c_uint) -> cairo_status_t;
pub type cairo_read_func_t = extern fn(closure: *mut c_void, data: *mut c_uchar, length: c_uint) -> cairo_status_t;

#[repr(C)]
#[derive(FromPrimitive)]
pub enum cairo_operator_t {
	CAIRO_OPERATOR_CLEAR,

	CAIRO_OPERATOR_SOURCE,
	CAIRO_OPERATOR_OVER,
	CAIRO_OPERATOR_IN,
	CAIRO_OPERATOR_OUT,
	CAIRO_OPERATOR_ATOP,

	CAIRO_OPERATOR_DEST,
	CAIRO_OPERATOR_DEST_OVER,
	CAIRO_OPERATOR_DEST_IN,
	CAIRO_OPERATOR_DEST_OUT,
	CAIRO_OPERATOR_DEST_ATOP,

	CAIRO_OPERATOR_XOR,
	CAIRO_OPERATOR_ADD,
	CAIRO_OPERATOR_SATURATE,

	CAIRO_OPERATOR_MULTIPLY,
	CAIRO_OPERATOR_SCREEN,
	CAIRO_OPERATOR_OVERLAY,
	CAIRO_OPERATOR_DARKEN,
	CAIRO_OPERATOR_LIGHTEN,
	CAIRO_OPERATOR_COLOR_DODGE,
	CAIRO_OPERATOR_COLOR_BURN,
	CAIRO_OPERATOR_HARD_LIGHT,
	CAIRO_OPERATOR_SOFT_LIGHT,
	CAIRO_OPERATOR_DIFFERENCE,
	CAIRO_OPERATOR_EXCLUSION,
	CAIRO_OPERATOR_HSL_HUE,
	CAIRO_OPERATOR_HSL_SATURATION,
	CAIRO_OPERATOR_HSL_COLOR,
	CAIRO_OPERATOR_HSL_LUMINOSITY
}

#[repr(C)]
#[derive(FromPrimitive)]
pub enum cairo_antialias_t {
	CAIRO_ANTIALIAS_DEFAULT,
	CAIRO_ANTIALIAS_NONE,
	CAIRO_ANTIALIAS_GRAY,
	CAIRO_ANTIALIAS_SUBPIXEL
}

#[repr(C)]
#[derive(FromPrimitive)]
pub enum cairo_fill_rule_t {
	CAIRO_FILL_RULE_WINDING,
	CAIRO_FILL_RULE_EVEN_ODD
}

#[repr(C)]
#[derive(FromPrimitive)]
pub enum cairo_line_cap_t {
	CAIRO_LINE_CAP_BUTT,
	CAIRO_LINE_CAP_ROUND,
	CAIRO_LINE_CAP_SQUARE
}

#[repr(C)]
#[derive(FromPrimitive)]
pub enum cairo_line_join_t {
	CAIRO_LINE_JOIN_MITER,
	CAIRO_LINE_JOIN_ROUND,
	CAIRO_LINE_JOIN_BEVEL
}

#[repr(C)]
pub struct cairo_rectangle_t {
	pub x: c_double, 
	pub y: c_double, 
	pub width: c_double,
	pub height: c_double,
}

#[repr(C)]
pub struct cairo_rectangle_list_t {
	pub status: cairo_status_t,
	pub rectangles: *mut cairo_rectangle_t,
	pub num_rectangles: c_int,
}

#[repr(C)] pub struct cairo_scaled_font_t;
#[repr(C)] pub struct cairo_font_face_t;

#[repr(C)]
pub struct cairo_glyph_t {
	pub index: c_ulong,
	pub x: c_double,
	pub y: c_double,
}

#[repr(C)]
pub struct cairo_text_cluster_t {
	pub num_bytes: c_int,
	pub num_glyphs: c_int,
}

#[repr(C)]
#[derive(FromPrimitive)]
pub enum cairo_text_cluster_flags_t {
	/// Needed for Rust to compile this
	CAIRO_TEXT_CLUSTER_FLAG_NONE = 0x00000000,
	CAIRO_TEXT_CLUSTER_FLAG_BACKWARD = 0x00000001
}

#[repr(C)]
pub struct cairo_text_extents_t {
	pub x_bearing: c_double,
	pub y_bearing: c_double,
	pub width: c_double,
	pub height: c_double,
	pub x_advance: c_double,
	pub y_advance: c_double,
}

#[repr(C)]
pub struct cairo_font_extents_t {
	pub ascent: c_double,
	pub descent: c_double,
	pub height: c_double,
	pub max_x_advance: c_double,
	pub max_y_advance: c_double,
}

#[repr(C)]
#[derive(FromPrimitive)]
pub enum cairo_font_slant_t {
	CAIRO_FONT_SLANT_NORMAL, 
	CAIRO_FONT_SLANT_ITALIC, 
	CAIRO_FONT_SLANT_OBLIQUE
}

#[repr(C)]
#[derive(FromPrimitive)]
pub enum cairo_font_weight_t {
	CAIRO_FONT_WEIGHT_NORMAL, 
	CAIRO_FONT_WEIGHT_BOLD
}

#[repr(C)]
#[derive(FromPrimitive)]
pub enum cairo_subpixel_order_t {
	CAIRO_SUBPIXEL_ORDER_DEFAULT, 
	CAIRO_SUBPIXEL_ORDER_RGB, 
	CAIRO_SUBPIXEL_ORDER_BGR, 
	CAIRO_SUBPIXEL_ORDER_VRGB, 
	CAIRO_SUBPIXEL_ORDER_VBGR
}

#[repr(C)]
#[derive(FromPrimitive)]
pub enum cairo_hint_style_t {
	CAIRO_HINT_STYLE_DEFAULT, 
	CAIRO_HINT_STYLE_NONE, 
	CAIRO_HINT_STYLE_SLIGHT, 
	CAIRO_HINT_STYLE_MEDIUM, 
	CAIRO_HINT_STYLE_FULL
}

#[repr(C)]
#[derive(FromPrimitive)]
pub enum cairo_hint_metrics_t {
	CAIRO_HINT_METRICS_DEFAULT, 
	CAIRO_HINT_METRICS_OFF, 
	CAIRO_HINT_METRICS_ON
}

#[repr(C)] pub struct cairo_font_options_t;

#[repr(C)]
#[derive(FromPrimitive)]
pub enum cairo_font_type_t {
	CAIRO_FONT_TYPE_TOY, 
	CAIRO_FONT_TYPE_FT, 
	CAIRO_FONT_TYPE_WIN32, 
	CAIRO_FONT_TYPE_QUARTZ, 
	CAIRO_FONT_TYPE_USER
}

pub type cairo_user_scaled_font_init_func_t = extern fn(scaled_font: *mut cairo_scaled_font_t, cr: *mut cairo_t, extents: *mut cairo_font_extents_t);
pub type cairo_user_scaled_font_render_glyph_func_t = extern fn(scaled_font: *mut cairo_scaled_font_t, glyph: c_ulong, cr: *mut cairo_t, extents: *mut cairo_text_extents_t) -> cairo_status_t;
pub type cairo_user_scaled_font_text_to_glyphs_func_t = extern fn(scaled_font: *mut cairo_scaled_font_t, utf8: *const c_char, utf8_len: c_int, glyphs: *mut *mut cairo_glyph_t, num_glyphs: *mut c_int, clusters: *mut *mut cairo_text_cluster_t, num_clusters: *mut c_int, cluster_flags: *mut cairo_text_cluster_flags_t) -> cairo_status_t;
pub type cairo_user_scaled_font_unicode_to_glyph_func_t = extern fn(scaled_font: *mut cairo_scaled_font_t, unicode: c_ulong, glyph_index: *mut c_ulong) -> cairo_status_t;

#[repr(C)]
#[derive(FromPrimitive)]
pub enum cairo_path_data_type_t {
	CAIRO_PATH_MOVE_TO, 
	CAIRO_PATH_LINE_TO, 
	CAIRO_PATH_CURVE_TO, 
	CAIRO_PATH_CLOSE_PATH
}

/// Represents either a header or a point
///
/// To use this, create a cairo_path_data_header_t or cairo_path_data_point_t and call
/// the into_data() method. To get the header or point, call the into_header() or into_point()
/// methods on the cairo_path_data_t.
#[repr(C)]
pub struct cairo_path_data_t {
	x: c_double,
	y: c_double,	
}

impl cairo_path_data_t {
	pub unsafe fn into_header(self) -> cairo_path_data_header_t {
		mem::transmute(self)
	}
	pub unsafe fn into_point(self) -> cairo_path_data_point_t {
		mem::transmute(self)
	}
}

#[repr(C)]
pub struct cairo_path_data_header_t {
	pub kind: cairo_path_data_type_t,
	pub length: c_int,
	
	unused: c_double,
}

impl cairo_path_data_header_t {
	pub unsafe fn into_data(self) -> cairo_path_data_t {
		mem::transmute(self)
	}
}

#[repr(C)]
pub struct cairo_path_data_point_t {
	pub x: c_double,
	pub y: c_double,
}

impl cairo_path_data_point_t {
	pub unsafe fn into_data(self) -> cairo_path_data_t {
		mem::transmute(self)
	}
}

#[repr(C)]
pub struct cairo_path_t {
	pub status: cairo_status_t,
	pub data: *mut cairo_path_data_t,
	pub num_data: c_int,
}

#[repr(C)]
#[derive(FromPrimitive)]
pub enum cairo_device_type_t {
	CAIRO_DEVICE_TYPE_DRM, 
	CAIRO_DEVICE_TYPE_GL, 
	CAIRO_DEVICE_TYPE_SCRIPT, 
	CAIRO_DEVICE_TYPE_XCB, 
	CAIRO_DEVICE_TYPE_XLIB, 
	CAIRO_DEVICE_TYPE_XML
}

#[repr(C)]
#[derive(FromPrimitive)]
pub enum cairo_surface_type_t {
	CAIRO_SURFACE_TYPE_IMAGE,
	CAIRO_SURFACE_TYPE_PDF,
	CAIRO_SURFACE_TYPE_PS,
	CAIRO_SURFACE_TYPE_XLIB,
	CAIRO_SURFACE_TYPE_XCB,
	CAIRO_SURFACE_TYPE_GLITZ,
	CAIRO_SURFACE_TYPE_QUARTZ,
	CAIRO_SURFACE_TYPE_WIN32,
	CAIRO_SURFACE_TYPE_BEOS,
	CAIRO_SURFACE_TYPE_DIRECTFB,
	CAIRO_SURFACE_TYPE_SVG,
	CAIRO_SURFACE_TYPE_OS2,
	CAIRO_SURFACE_TYPE_WIN32_PRINTING,
	CAIRO_SURFACE_TYPE_QUARTZ_IMAGE,
	CAIRO_SURFACE_TYPE_SCRIPT,
	CAIRO_SURFACE_TYPE_QT,
	CAIRO_SURFACE_TYPE_RECORDING,
	CAIRO_SURFACE_TYPE_VG,
	CAIRO_SURFACE_TYPE_GL,
	CAIRO_SURFACE_TYPE_DRM,
	CAIRO_SURFACE_TYPE_TEE,
	CAIRO_SURFACE_TYPE_XML,
	CAIRO_SURFACE_TYPE_SKIA,
	CAIRO_SURFACE_TYPE_SUBSURFACE
}

#[repr(C)]
#[derive(FromPrimitive)]
pub enum cairo_format_t {
	CAIRO_FORMAT_INVALID = -1, 
	CAIRO_FORMAT_ARGB32 = 0, 
	CAIRO_FORMAT_RGB24 = 1, 
	CAIRO_FORMAT_A8 = 2, 
	CAIRO_FORMAT_A1 = 3, 
	CAIRO_FORMAT_RGB16_565 = 4
}

#[repr(C)]
#[derive(FromPrimitive)]
pub enum cairo_pattern_type_t {
	CAIRO_PATTERN_TYPE_SOLID, 
	CAIRO_PATTERN_TYPE_SURFACE, 
	CAIRO_PATTERN_TYPE_LINEAR, 
	CAIRO_PATTERN_TYPE_RADIAL
}

#[repr(C)]
#[derive(FromPrimitive)]
pub enum cairo_extend_t {
	CAIRO_EXTEND_NONE, 
	CAIRO_EXTEND_REPEAT, 
	CAIRO_EXTEND_REFLECT, 
	CAIRO_EXTEND_PAD
}

#[repr(C)]
#[derive(FromPrimitive)]
pub enum cairo_filter_t {
	CAIRO_FILTER_FAST, 
	CAIRO_FILTER_GOOD, 
	CAIRO_FILTER_BEST, 
	CAIRO_FILTER_NEAREST, 
	CAIRO_FILTER_BILINEAR, 
	CAIRO_FILTER_GAUSSIAN
}

#[repr(C)] pub struct cairo_region_t;

#[repr(C)]
pub struct cairo_rectangle_int_t {
	pub x: c_int, 
	pub y: c_int,
	pub width: c_int, 
	pub height: c_int,
}

#[repr(C)]
#[derive(FromPrimitive)]
pub enum cairo_region_overlap_t {
	CAIRO_REGION_OVERLAP_IN, 
	CAIRO_REGION_OVERLAP_OUT, 
	CAIRO_REGION_OVERLAP_PART
}

extern {
	pub fn cairo_version() -> c_int;
	pub fn cairo_version_string() -> *const c_char;
	pub fn cairo_create(target: *mut cairo_surface_t) -> *mut cairo_t;
	pub fn cairo_reference(cr: *mut cairo_t) -> *mut cairo_t;
	pub fn cairo_destroy(cr: *mut cairo_t);
	pub fn cairo_get_reference_count(cr: *mut cairo_t) -> c_uint;
	pub fn cairo_get_user_data(cr: *mut cairo_t, key: *const cairo_user_data_key_t) -> *mut c_void;
	pub fn cairo_set_user_data(cr: *mut cairo_t, key: *const cairo_user_data_key_t, user_data: *mut c_void, destroy: cairo_destroy_func_t) -> cairo_status_t;
	pub fn cairo_save(cr: *mut cairo_t);
	pub fn cairo_restore(cr: *mut cairo_t);
	pub fn cairo_push_group(cr: *mut cairo_t);
	pub fn cairo_push_group_with_content(cr: *mut cairo_t, content: cairo_content_t);
	pub fn cairo_pop_group(cr: *mut cairo_t) -> *mut cairo_pattern_t;
	pub fn cairo_pop_group_to_source(cr: &mut cairo_t);
	pub fn cairo_set_operator(cr: *mut cairo_t, op: cairo_operator_t);
	pub fn cairo_set_source(cr: *mut cairo_t, source: *mut cairo_pattern_t);
	pub fn cairo_set_source_rgb(cr: *mut cairo_t, red: c_double, green: c_double, blue: c_double);
	pub fn cairo_set_source_rgba(cr: *mut cairo_t, red: c_double, green: c_double, blue: c_double, alpha: c_double);
	pub fn cairo_set_source_surface(cr: *mut cairo_t, surface: *mut cairo_surface_t, x: c_double, y: c_double);
	pub fn cairo_set_tolerance(cr: *mut cairo_t, tolerance: c_double);
	pub fn cairo_set_antialias(cr: *mut cairo_t, antialias: cairo_antialias_t);
	pub fn cairo_set_fill_rule(cr: *mut cairo_t, fill_rule: cairo_fill_rule_t);
	pub fn cairo_set_line_width(cr: *mut cairo_t, width: c_double);
	pub fn cairo_set_line_cap(cr: *mut cairo_t, line_cap: cairo_line_cap_t);
	pub fn cairo_set_line_join(cr: *mut cairo_t, line_join: cairo_line_join_t);
	pub fn cairo_set_dash(cr: *mut cairo_t, dashes: *const c_double, num_dashes: c_int, offset: c_double);
	pub fn cairo_set_miter_limit(cr: *mut cairo_t, limit: c_double);
	pub fn cairo_translate(cr: *mut cairo_t, tx: c_double, ty: c_double) -> c_void;
	pub fn cairo_scale(cr: *mut cairo_t, sx: c_double, sy: c_double) -> c_void;
	pub fn cairo_rotate(cr: *mut cairo_t, angle: c_double) -> c_void;
	pub fn cairo_transform(cr: *mut cairo_t, matrix: *const cairo_matrix_t) -> c_void;
	pub fn cairo_set_matrix(cr: *mut cairo_t, matrix: *const cairo_matrix_t) -> c_void;
	pub fn cairo_identity_matrix(cr: *mut cairo_t) -> c_void;
	pub fn cairo_user_to_device(cr: *mut cairo_t, x: *mut c_double, y: *mut c_double) -> c_void;
	pub fn cairo_user_to_device_distance(cr: *mut cairo_t, dx: *mut c_double, dy: *mut c_double) -> c_void;
	pub fn cairo_device_to_user(cr: *mut cairo_t, x: *mut c_double, y: *mut c_double) -> c_void;
	pub fn cairo_device_to_user_distance(cr: *mut cairo_t, dx: *mut c_double, dy: *mut c_double) -> c_void;
	pub fn cairo_new_path(cr: *mut cairo_t) -> c_void;
	pub fn cairo_move_to(cr: *mut cairo_t, x: c_double, y: c_double) -> c_void;
	pub fn cairo_new_sub_path(cr: *mut cairo_t) -> c_void;
	pub fn cairo_line_to(cr: *mut cairo_t, x: c_double, y: c_double) -> c_void;
	pub fn cairo_curve_to(cr: *mut cairo_t, x1: c_double, y1: c_double, x2: c_double, y2: c_double, x3: c_double, y3: c_double) -> c_void;
	pub fn cairo_arc(cr: *mut cairo_t, xc: c_double, yc: c_double, radius: c_double, angle1: c_double, angle2: c_double) -> c_void;
	pub fn cairo_arc_negative(cr: *mut cairo_t, xc: c_double, yc: c_double, radius: c_double, angle1: c_double, angle2: c_double) -> c_void;
	pub fn cairo_rel_move_to(cr: *mut cairo_t, dx: c_double, dy: c_double) -> c_void;
	pub fn cairo_rel_line_to(cr: *mut cairo_t, dx: c_double, dy: c_double) -> c_void;
	pub fn cairo_rel_curve_to(cr: *mut cairo_t, dx1: c_double, dy1: c_double, dx2: c_double, dy2: c_double, dx3: c_double, dy3: c_double) -> c_void;
	pub fn cairo_rectangle(cr: *mut cairo_t, x: c_double, y: c_double, width: c_double, height: c_double) -> c_void;
	pub fn cairo_close_path(cr: *mut cairo_t) -> c_void;
	pub fn cairo_path_extents(cr: *mut cairo_t, x1: *mut c_double, y1: *mut c_double, x2: *mut c_double, y2: *mut c_double) -> c_void;
	pub fn cairo_pac_int(cr: *mut cairo_t) -> c_void;
	pub fn cairo_pac_int_with_alpha(cr: *mut cairo_t, alpha: c_double) -> c_void;
	pub fn cairo_mask(cr: *mut cairo_t, pattern: *mut cairo_pattern_t) -> c_void;
	pub fn cairo_mask_surface(cr: *mut cairo_t, surface: *mut cairo_surface_t, surface_x: c_double, surface_y: c_double) -> c_void;
	pub fn cairo_stroke(cr: *mut cairo_t) -> c_void;
	pub fn cairo_stroke_preserve(cr: *mut cairo_t) -> c_void;
	pub fn cairo_fill(cr: *mut cairo_t) -> c_void;
	pub fn cairo_fill_preserve(cr: *mut cairo_t) -> c_void;
	pub fn cairo_copy_page(cr: *mut cairo_t) -> c_void;
	pub fn cairo_show_page(cr: *mut cairo_t) -> c_void;
	pub fn cairo_in_stroke(cr: *mut cairo_t, x: c_double, y: c_double) -> cairo_bool_t;
	pub fn cairo_in_fill(cr: *mut cairo_t, x: c_double, y: c_double) -> cairo_bool_t;
	pub fn cairo_in_clip(cr: *mut cairo_t, x: c_double, y: c_double) -> cairo_bool_t;
	pub fn cairo_stroke_extents(cr: *mut cairo_t, x1: *mut c_double, y1: *mut c_double, x2: *mut c_double, y2: *mut c_double) -> c_void;
	pub fn cairo_fill_extents(cr: *mut cairo_t, x1: *mut c_double, y1: *mut c_double, x2: *mut c_double, y2: *mut c_double) -> c_void;
	pub fn cairo_reset_clip(cr: *mut cairo_t) -> c_void;
	pub fn cairo_clip(cr: *mut cairo_t) -> c_void;
	pub fn cairo_clip_preserve(cr: *mut cairo_t) -> c_void;
	pub fn cairo_clip_extents(cr: *mut cairo_t, x1: *mut c_double, y1: *mut c_double, x2: *mut c_double, y2: *mut c_double) -> c_void;
	pub fn cairo_copy_clip_rectangle_list(cr: *mut cairo_t) -> *mut cairo_rectangle_list_t;
	pub fn cairo_rectangle_list_destroy(rectangle_list: *mut cairo_rectangle_list_t) -> c_void;
	pub fn cairo_glyph_allocate(num_glyphs: c_int) -> *mut cairo_glyph_t;
	pub fn cairo_glyph_gree(glyphs: *mut cairo_glyph_t);
	pub fn cairo_text_cluster_allocate(num_clusters: c_int) -> *mut cairo_text_cluster_t;
	pub fn cairo_text_cluster_free(clusters: *mut cairo_text_cluster_t) -> c_void;
	pub fn cairo_font_options_create() -> *mut cairo_font_options_t;
	pub fn cairo_font_options_copy(original: *const cairo_font_options_t) -> *mut cairo_font_options_t;
	pub fn cairo_font_options_destroy(options: *mut cairo_font_options_t) -> c_void;
	pub fn cairo_font_options_status(options: *mut cairo_font_options_t) -> cairo_status_t;
	pub fn cairo_font_options_merge(options: *mut cairo_font_options_t, other: *const cairo_font_options_t) -> c_void;
	pub fn cairo_font_options_equal(options: *const cairo_font_options_t, other: *const cairo_font_options_t) -> cairo_bool_t;
	pub fn cairo_font_options_hash(options: *const cairo_font_options_t) -> c_ulong;
	pub fn cairo_font_options_set_antialias(options: *mut cairo_font_options_t, antialias: cairo_antialias_t) -> c_void;
	pub fn cairo_font_options_get_antialias(options: *const cairo_font_options_t) -> cairo_antialias_t;
	pub fn cairo_font_options_set_subpixel_order(options: *mut cairo_font_options_t, subpixel_order: cairo_subpixel_order_t) -> c_void;
	pub fn cairo_font_options_get_subpixel_order(options: *const cairo_font_options_t) -> cairo_subpixel_order_t;
	pub fn cairo_font_options_set_hint_style(options: *mut cairo_font_options_t, hint_style: cairo_hint_style_t) -> c_void;
	pub fn cairo_font_options_get_hint_style(options: *const cairo_font_options_t) -> cairo_hint_style_t;
	pub fn cairo_font_options_set_hint_metrics(options: *mut cairo_font_options_t, hint_metrics: cairo_hint_metrics_t) -> c_void;
	pub fn cairo_font_options_get_hint_metrics(options: *const cairo_font_options_t) -> cairo_hint_metrics_t;
	pub fn cairo_select_font_face(cr: *mut cairo_t, family: *const c_char, slant: cairo_font_slant_t, weight: cairo_font_weight_t) -> c_void;
	pub fn cairo_set_font_size(cr: *mut cairo_t, size: c_double) -> c_void;
	pub fn cairo_set_font_matrix(cr: *mut cairo_t, matrix: *const cairo_matrix_t) -> c_void;
	pub fn cairo_get_font_matrix(cr: *mut cairo_t, matrix: *mut cairo_matrix_t) -> c_void;
	pub fn cairo_set_font_options(cr: *mut cairo_t, options: *const cairo_font_options_t) -> c_void;
	pub fn cairo_get_font_options(cr: *mut cairo_t, options: *mut cairo_font_options_t) -> c_void;
	pub fn cairo_set_font_face(cr: *mut cairo_t, font_face: *mut cairo_font_face_t) -> c_void;
	pub fn cairo_get_font_face(cr: *mut cairo_t) -> *mut cairo_font_face_t;
	pub fn cairo_set_scaled_font(cr: *mut cairo_t, scaled_font: *const cairo_scaled_font_t) -> c_void;
	pub fn cairo_get_scaled_font(cr: *mut cairo_t) -> *mut cairo_scaled_font_t;
	pub fn cairo_show_text(cr: *mut cairo_t, utf8: *const c_char) -> c_void;
	pub fn cairo_show_glyphs(cr: *mut cairo_t, glyphs: *const cairo_glyph_t, num_glyphs: c_int) -> c_void;
	pub fn cairo_show_text_glyphs(cr: *mut cairo_t, utf8: *const c_char, utf8_len: c_int, glyphs: *const cairo_glyph_t, num_glyphs: c_int, clusters: *const cairo_text_cluster_t, num_clusters: c_int, cluster_flags: cairo_text_cluster_flags_t) -> c_void;
	pub fn cairo_text_path(cr: *mut cairo_t, utf8: *const c_char) -> c_void;
	pub fn cairo_glyph_path(cr: *mut cairo_t, glyphs: *const cairo_glyph_t, num_glyphs: c_int) -> c_void;
	pub fn cairo_text_extents(cr: *mut cairo_t, utf8: *const c_char, extents: *mut cairo_text_extents_t) -> c_void;
	pub fn cairo_glyph_extents(cr: *mut cairo_t, glyphs: *const cairo_glyph_t, num_glyphs: c_int, extents: *mut cairo_text_extents_t) -> c_void;
	pub fn cairo_font_extents(cr: *mut cairo_t, extents: *mut cairo_font_extents_t) -> c_void;
	pub fn cairo_font_face_reference(font_face: *mut cairo_font_face_t) -> *mut cairo_font_face_t;
	pub fn cairo_font_face_destroy(font_face: *mut cairo_font_face_t) -> c_void;
	pub fn cairo_font_face_get_reference_count(font_face: *mut cairo_font_face_t) -> c_uint;
	pub fn cairo_font_face_status(font_face: *mut cairo_font_face_t) -> cairo_status_t;
	pub fn cairo_font_face_get_type(font_face: *mut cairo_font_face_t) -> cairo_font_type_t;
	pub fn cairo_font_face_get_user_data(font_face: *mut cairo_font_face_t, key: *const cairo_user_data_key_t) -> *mut c_void;
	pub fn cairo_font_face_set_user_data(font_face: *mut cairo_font_face_t, key: *const cairo_user_data_key_t, user_data: *mut c_void, destroy: cairo_destroy_func_t) -> cairo_status_t;
	pub fn cairo_scaled_font_create(font_face: *mut cairo_font_face_t, font_matrix: *const cairo_matrix_t, ctm: *const cairo_matrix_t, options: *const cairo_font_options_t) -> *mut cairo_scaled_font_t;
	pub fn cairo_scaled_font_reference(scaled_font: *mut cairo_scaled_font_t) -> *mut cairo_scaled_font_t;
	pub fn cairo_scaled_font_destroy(scaled_font: *mut cairo_scaled_font_t) -> c_void;
	pub fn cairo_scaled_font_get_reference_count(scaled_font: *mut cairo_scaled_font_t) -> c_uint;
	pub fn cairo_scaled_font_status(scaled_font: *mut cairo_scaled_font_t) -> cairo_status_t;
	pub fn cairo_scaled_font_get_type(scaled_font: *mut cairo_scaled_font_t) -> cairo_font_type_t;
	pub fn cairo_scaled_font_get_user_data(scaled_font: *mut cairo_scaled_font_t, key: *const cairo_user_data_key_t) -> *mut c_void;
	pub fn cairo_scaled_font_set_user_data(scaled_font: *mut cairo_scaled_font_t, key: *const cairo_user_data_key_t, user_data: *mut c_void, destroy: cairo_destroy_func_t) -> cairo_status_t;
	pub fn cairo_scaled_font_extents(scaled_font: *mut cairo_scaled_font_t, extents: *mut cairo_font_extents_t) -> c_void;
	pub fn cairo_scaled_font_text_extents(scaled_font: *mut cairo_scaled_font_t, utf8: *const c_char, extents: *mut cairo_text_extents_t) -> c_void;
	pub fn cairo_scaled_font_glyph_extents(scaled_font: *mut cairo_scaled_font_t, glyphs: *const cairo_glyph_t, num_glyphs: c_int, extents: *mut cairo_text_extents_t) -> c_void;
	pub fn cairo_scaled_font_text_to_glyphs(scaled_font: *mut cairo_scaled_font_t, x: c_double, y: c_double, utf8: *const c_char, utf8_len: c_int, glyphs: *mut *mut cairo_glyph_t, num_glyphs: *mut c_int, clusters: *mut *mut cairo_text_cluster_t, num_clusters: *mut c_int, cluster_flags: *mut cairo_text_cluster_flags_t) -> cairo_status_t;
	pub fn cairo_scaled_font_get_font_face(scaled_font: *mut cairo_scaled_font_t) -> *mut cairo_font_face_t;
	pub fn cairo_scaled_font_get_font_matrix(scaled_font: *mut cairo_scaled_font_t, font_matrix: *mut cairo_matrix_t) -> c_void;
	pub fn cairo_scaled_font_get_ctm(scaled_font: *mut cairo_scaled_font_t, ctm: *mut cairo_matrix_t) -> c_void;
	pub fn cairo_scaled_font_get_scale_matrix(scaled_font: *mut cairo_scaled_font_t, scale_matrix: *mut cairo_matrix_t) -> c_void;
	pub fn cairo_scaled_font_get_font_options(scaled_font: *mut cairo_scaled_font_t, options: *mut cairo_font_options_t) -> c_void;
	pub fn cairo_toy_font_face_create(family: *const c_char, slant: cairo_font_slant_t, weight: cairo_font_weight_t) -> *mut cairo_font_face_t;
	pub fn cairo_toy_font_face_get_family(font_face: *mut cairo_font_face_t) -> *const c_char;
	pub fn cairo_toy_font_face_get_slant(font_face: *mut cairo_font_face_t) -> cairo_font_slant_t;
	pub fn cairo_toy_font_face_get_weight(font_face: *mut cairo_font_face_t) -> cairo_font_weight_t;
	pub fn cairo_user_font_face_create() -> *mut cairo_font_face_t;
	pub fn cairo_user_font_face_set_init_func(font_face: *mut cairo_font_face_t, init_func: cairo_user_scaled_font_init_func_t) -> c_void;
	pub fn cairo_user_font_face_set_render_glyph_func(font_face: *mut cairo_font_face_t, render_glyph_func: cairo_user_scaled_font_render_glyph_func_t) -> c_void;
	pub fn cairo_user_font_face_set_text_to_glyphs_func(font_face: *mut cairo_font_face_t, text_to_glyphs_func: cairo_user_scaled_font_text_to_glyphs_func_t) -> c_void;
	pub fn cairo_user_font_face_set_unicode_to_glyph_func(font_face: *mut cairo_font_face_t, unicode_to_glyph_func: cairo_user_scaled_font_unicode_to_glyph_func_t) -> c_void;
	pub fn cairo_user_font_face_get_init_func(font_face: *mut cairo_font_face_t) -> cairo_user_scaled_font_init_func_t;
	pub fn cairo_user_font_face_get_render_glyph_func(font_face: *mut cairo_font_face_t) -> cairo_user_scaled_font_render_glyph_func_t;
	pub fn cairo_user_font_face_get_text_to_glyphs_func(font_face: *mut cairo_font_face_t) -> cairo_user_scaled_font_text_to_glyphs_func_t;
	pub fn cairo_user_font_face_get_unicode_to_glyph_func(font_face: *mut cairo_font_face_t) -> cairo_user_scaled_font_unicode_to_glyph_func_t;
	pub fn cairo_get_operator(cr: *mut cairo_t) -> cairo_operator_t;
	pub fn cairo_get_source(cr: *mut cairo_t) -> *mut cairo_pattern_t;
	pub fn cairo_get_tolerance(cr: *mut cairo_t) -> c_double;
	pub fn cairo_get_antialias(cr: *mut cairo_t) -> cairo_antialias_t;
	pub fn cairo_has_current_poc_int(cr: *mut cairo_t) -> cairo_bool_t;
	pub fn cairo_get_current_poc_int(cr: *mut cairo_t, x: *mut c_double, y: *mut c_double) -> c_void;
	pub fn cairo_get_fill_rule(cr: *mut cairo_t) -> cairo_fill_rule_t;
	pub fn cairo_get_line_width(cr: *mut cairo_t) -> c_double;
	pub fn cairo_get_line_cap(cr: *mut cairo_t) -> cairo_line_cap_t;
	pub fn cairo_get_line_join(cr: *mut cairo_t) -> cairo_line_join_t;
	pub fn cairo_get_miter_limit(cr: *mut cairo_t) -> c_double;
	pub fn cairo_get_dash_count(cr: *mut cairo_t) -> c_int;
	pub fn cairo_get_dash(cr: *mut cairo_t, dashes: *mut c_double, offset: *mut c_double) -> c_void;
	pub fn cairo_get_matrix(cr: *mut cairo_t, matrix: *mut cairo_matrix_t) -> c_void;
	pub fn cairo_get_target(cr: *mut cairo_t) -> *mut cairo_surface_t;
	pub fn cairo_get_group_target(cr: *mut cairo_t) -> *mut cairo_surface_t;
	pub fn cairo_copy_path(cr: *mut cairo_t) -> *mut cairo_path_t;
	pub fn cairo_copy_path_flat(cr: *mut cairo_t) -> *mut cairo_path_t;
	pub fn cairo_append_path(cr: *mut cairo_t, path: *const cairo_path_t) -> c_void;
	pub fn cairo_path_destroy(path: *mut cairo_path_t) -> c_void;
	pub fn cairo_status(cr: *mut cairo_t) -> cairo_status_t;
	pub fn cairo_status_to_string(status: cairo_status_t) -> *const c_char;
	pub fn cairo_device_reference(device: *mut cairo_device_t) -> *mut cairo_device_t;
	pub fn cairo_device_get_type(device: *mut cairo_device_t) -> cairo_device_type_t;
	pub fn cairo_device_status(device: *mut cairo_device_t) -> cairo_status_t;
	pub fn cairo_device_acquire(device: *mut cairo_device_t) -> cairo_status_t;
	pub fn cairo_device_release(device: *mut cairo_device_t) -> c_void;
	pub fn cairo_device_flush(device: *mut cairo_device_t) -> c_void;
	pub fn cairo_device_finish(device: *mut cairo_device_t) -> c_void;
	pub fn cairo_device_destroy(device: *mut cairo_device_t) -> c_void;
	pub fn cairo_device_get_reference_count(device: *mut cairo_device_t) -> c_uint;
	pub fn cairo_device_get_user_data(device: *mut cairo_device_t, key: *const cairo_user_data_key_t) -> *mut c_void;
	pub fn cairo_device_set_user_data(device: *mut cairo_device_t, key: *const cairo_user_data_key_t, user_data: *mut c_void, destroy: cairo_destroy_func_t) -> cairo_status_t;
	pub fn cairo_surface_create_similar(other: *mut cairo_surface_t, content: cairo_content_t, width: c_int, height: c_int) -> *mut cairo_surface_t;
	pub fn cairo_surface_create_for_rectangle(target: *mut cairo_surface_t, x: c_double, y: c_double, width: c_double, height: c_double) -> *mut cairo_surface_t;
	pub fn cairo_surface_reference(surface: *mut cairo_surface_t) -> *mut cairo_surface_t;
	pub fn cairo_surface_finish(surface: *mut cairo_surface_t) -> c_void;
	pub fn cairo_surface_destroy(surface: *mut cairo_surface_t) -> c_void;
	pub fn cairo_surface_get_device(surface: *mut cairo_surface_t) -> *mut cairo_device_t;
	pub fn cairo_surface_get_reference_count(surface: *mut cairo_surface_t) -> c_uint;
	pub fn cairo_surface_status(surface: *mut cairo_surface_t) -> cairo_status_t;
	pub fn cairo_surface_get_type(surface: *mut cairo_surface_t) -> cairo_surface_type_t;
	pub fn cairo_surface_get_content(surface: *mut cairo_surface_t) -> cairo_content_t;
	pub fn cairo_surface_write_to_png(surface: *mut cairo_surface_t, filename: *const c_char) -> cairo_status_t;
	pub fn cairo_surface_write_to_png_stream(surface: *mut cairo_surface_t, write_func: cairo_write_func_t, closure: *mut c_void) -> cairo_status_t;
	pub fn cairo_surface_get_user_data(surface: *mut cairo_surface_t, key: *const cairo_user_data_key_t) -> *mut c_void;
	pub fn cairo_surface_set_user_data(surface: *mut cairo_surface_t, key: *const cairo_user_data_key_t, user_data: *mut c_void, destroy: cairo_destroy_func_t) -> cairo_status_t;
	pub fn cairo_surface_get_mime_data(surface: *mut cairo_surface_t, mime_type: *const c_char, data: *const c_uchar, length: *mut c_ulong) -> c_void;
	pub fn cairo_surface_set_mime_data(surface: *mut cairo_surface_t, mime_type: *const c_char, data: *const c_uchar, length: c_ulong, destroy: cairo_destroy_func_t, closure: *mut c_void) -> cairo_status_t;
	pub fn cairo_surface_get_font_options(surface: *mut cairo_surface_t, options: *mut cairo_font_options_t) -> c_void;
	pub fn cairo_surface_flush(surface: *mut cairo_surface_t) -> c_void;
	pub fn cairo_surface_mark_dirty(surface: *mut cairo_surface_t) -> c_void;
	pub fn cairo_surface_mark_dirty_rectangle(surface: *mut cairo_surface_t, x: c_int, y: c_int, width: c_int, height: c_int) -> c_void;
	pub fn cairo_surface_set_device_offset(surface: *mut cairo_surface_t, x_offset: c_double, y_offset: c_double) -> c_void;
	pub fn cairo_surface_get_device_offset(surface: *mut cairo_surface_t, x_offset: *mut c_double, y_offset: *mut c_double) -> c_void;
	pub fn cairo_surface_set_fallback_resolution(surface: *mut cairo_surface_t, x_pixels_per_inch: c_double, y_pixels_per_inch: c_double) -> c_void;
	pub fn cairo_surface_get_fallback_resolution(surface: *mut cairo_surface_t, x_pixels_per_inch: *mut c_double, y_pixels_per_inch: *mut c_double) -> c_void;
	pub fn cairo_surface_copy_page(surface: *mut cairo_surface_t) -> c_void;
	pub fn cairo_surface_show_page(surface: *mut cairo_surface_t) -> c_void;
	pub fn cairo_surface_has_show_text_glyphs(surface: *mut cairo_surface_t) -> cairo_bool_t;
	pub fn cairo_image_surface_create(format: cairo_format_t, width: c_int, height: c_int) -> *mut cairo_surface_t;
	pub fn cairo_format_stride_for_width(format: cairo_format_t, width: c_int) -> c_int;
	pub fn cairo_image_surface_create_for_data(data: *mut c_uchar, format: cairo_format_t, width: c_int, height: c_int, stride: c_int) -> *mut cairo_surface_t;
	pub fn cairo_image_surface_get_data(surface: *mut cairo_surface_t) -> *mut c_uchar;
	pub fn cairo_image_surface_get_format(surface: *mut cairo_surface_t) -> cairo_format_t;
	pub fn cairo_image_surface_get_width(surface: *mut cairo_surface_t) -> c_int;
	pub fn cairo_image_surface_get_height(surface: *mut cairo_surface_t) -> c_int;
	pub fn cairo_image_surface_get_stride(surface: *mut cairo_surface_t) -> c_int;
	pub fn cairo_image_surface_create_from_png(filename: *const c_char) -> *mut cairo_surface_t;
	pub fn cairo_image_surface_create_from_png_stream(read_func: cairo_read_func_t, closure: *mut c_void) -> *mut cairo_surface_t;
	pub fn cairo_recording_surface_create(content: cairo_content_t, extents: *const cairo_rectangle_t) -> *mut cairo_surface_t;
	pub fn cairo_recording_surface_ink_extents(surface: *mut cairo_surface_t, x0: *mut c_double, y0: *mut c_double, width: *mut c_double, height: *mut c_double) -> c_void;
	pub fn cairo_pattern_create_rgb(red: c_double, green: c_double, blue: c_double) -> *mut cairo_pattern_t;
	pub fn cairo_pattern_create_rgba(red: c_double, green: c_double, blue: c_double, alpha: c_double) -> *mut cairo_pattern_t;
	pub fn cairo_pattern_create_for_surface(surface: *mut cairo_surface_t) -> *mut cairo_pattern_t;
	pub fn cairo_pattern_create_linear(x0: c_double, y0: c_double, x1: c_double, y1: c_double) -> *mut cairo_pattern_t;
	pub fn cairo_pattern_create_radial(cx0: c_double, cy0: c_double, radius0: c_double, cx1: c_double, cy1: c_double, radius1: c_double) -> *mut cairo_pattern_t;
	pub fn cairo_pattern_reference(pattern: *mut cairo_pattern_t) -> *mut cairo_pattern_t;
	pub fn cairo_pattern_destroy(pattern: *mut cairo_pattern_t) -> c_void;
	pub fn cairo_pattern_get_reference_count(pattern: *mut cairo_pattern_t) -> c_uint;
	pub fn cairo_pattern_status(pattern: *mut cairo_pattern_t) -> cairo_status_t;
	pub fn cairo_pattern_get_user_data(pattern: *mut cairo_pattern_t, key: *const cairo_user_data_key_t) -> *mut c_void;
	pub fn cairo_pattern_set_user_data(pattern: *mut cairo_pattern_t, key: *const cairo_user_data_key_t, user_data: *mut c_void, destroy: cairo_destroy_func_t) -> cairo_status_t;
	pub fn cairo_pattern_get_type(pattern: *mut cairo_pattern_t) -> cairo_pattern_type_t;
	pub fn cairo_pattern_add_color_stop_rgb(pattern: *mut cairo_pattern_t, offset: c_double, red: c_double, green: c_double, blue: c_double) -> c_void;
	pub fn cairo_pattern_add_color_stop_rgba(pattern: *mut cairo_pattern_t, offset: c_double, red: c_double, green: c_double, blue: c_double, alpha: c_double) -> c_void;
	pub fn cairo_pattern_set_matrix(pattern: *mut cairo_pattern_t, matrix: *const cairo_matrix_t) -> c_void;
	pub fn cairo_pattern_get_matrix(pattern: *mut cairo_pattern_t, matrix: *mut cairo_matrix_t) -> c_void;
	pub fn cairo_pattern_set_extend(pattern: *mut cairo_pattern_t, extend: cairo_extend_t) -> c_void;
	pub fn cairo_pattern_get_extend(pattern: *mut cairo_pattern_t) -> cairo_extend_t;
	pub fn cairo_pattern_set_filter(pattern: *mut cairo_pattern_t, filter: cairo_filter_t) -> c_void;
	pub fn cairo_pattern_get_filter(pattern: *mut cairo_pattern_t) -> cairo_filter_t;
	pub fn cairo_pattern_get_rgba(pattern: *mut cairo_pattern_t, red: *mut c_double, green: *mut c_double, blue: *mut c_double, alpha: *mut c_double) -> cairo_status_t;
	pub fn cairo_pattern_get_surface(pattern: *mut cairo_pattern_t, surface: *mut *mut cairo_surface_t) -> cairo_status_t;
	pub fn cairo_pattern_get_color_stop_rgba(pattern: *mut cairo_pattern_t, index: c_int, offset: *mut c_double, red: *mut c_double, green: *mut c_double, blue: *mut c_double, alpha: *mut c_double) -> cairo_status_t;
	pub fn cairo_pattern_get_color_stop_count(pattern: *mut cairo_pattern_t, count: *mut c_int) -> cairo_status_t;
	pub fn cairo_pattern_get_linear_poc_ints(pattern: *mut cairo_pattern_t, x0: *mut c_double, y0: *mut c_double, x1: *mut c_double, y1: *mut c_double) -> cairo_status_t;
	pub fn cairo_pattern_get_radial_circles(pattern: *mut cairo_pattern_t, x0: *mut c_double, y0: *mut c_double, r0: *mut c_double, x1: *mut c_double, y1: *mut c_double, r1: *mut c_double) -> cairo_status_t;
	pub fn cairo_matrix_init(matrix: *mut cairo_matrix_t, xx: c_double, yx: c_double, xy: c_double, yy: c_double, x0: c_double, y0: c_double) -> c_void;
	pub fn cairo_matrix_init_identity(matrix: *mut cairo_matrix_t) -> c_void;
	pub fn cairo_matrix_init_translate(matrix: *mut cairo_matrix_t, tx: c_double, ty: c_double) -> c_void;
	pub fn cairo_matrix_init_scale(matrix: *mut cairo_matrix_t, sx: c_double, sy: c_double) -> c_void;
	pub fn cairo_matrix_init_rotate(matrix: *mut cairo_matrix_t, radians: c_double) -> c_void;
	pub fn cairo_matrix_translate(matrix: *mut cairo_matrix_t, tx: c_double, ty: c_double) -> c_void;
	pub fn cairo_matrix_scale(matrix: *mut cairo_matrix_t, sx: c_double, sy: c_double) -> c_void;
	pub fn cairo_matrix_rotate(matrix: *mut cairo_matrix_t, radians: c_double) -> c_void;
	pub fn cairo_matrix_invert(matrix: *mut cairo_matrix_t) -> cairo_status_t;
	pub fn cairo_matrix_multiply(result: *mut cairo_matrix_t, a: *const cairo_matrix_t, b: *const cairo_matrix_t) -> c_void;
	pub fn cairo_matrix_transform_distance(matrix: *const cairo_matrix_t, dx: *mut c_double, dy: *mut c_double) -> c_void;
	pub fn cairo_matrix_transform_poc_int(matrix: *const cairo_matrix_t, x: *mut c_double, y: *mut c_double) -> c_void;
	pub fn cairo_region_create() -> *mut cairo_region_t;
	pub fn cairo_region_create_rectangle(rectangle: *const cairo_rectangle_int_t) -> *mut cairo_region_t;
	pub fn cairo_region_create_rectangles(rects: *const cairo_rectangle_int_t, count: c_int) -> *mut cairo_region_t;
	pub fn cairo_region_copy(original: *const cairo_region_t) -> *mut cairo_region_t;
	pub fn cairo_region_reference(region: *mut cairo_region_t) -> *mut cairo_region_t;
	pub fn cairo_region_destroy(region: *mut cairo_region_t) -> c_void;
	pub fn cairo_region_equal(a: *const cairo_region_t, b: *const cairo_region_t) -> cairo_bool_t;
	pub fn cairo_region_status(region: *const cairo_region_t) -> cairo_status_t;
	pub fn cairo_region_get_extents(region: *const cairo_region_t, extents: *mut cairo_rectangle_int_t) -> c_void;
	pub fn cairo_region_num_rectangles(region: *const cairo_region_t) -> c_int;
	pub fn cairo_region_get_rectangle(region: *const cairo_region_t, nth: c_int, rectangle: *mut cairo_rectangle_int_t) -> c_void;
	pub fn cairo_region_is_empty(region: *const cairo_region_t) -> cairo_bool_t;
	pub fn cairo_region_contains_rectangle(region: *const cairo_region_t, rectangle: *const cairo_rectangle_int_t) -> cairo_region_overlap_t;
	pub fn cairo_region_contains_poc_int(region: *const cairo_region_t, x: c_int, y: c_int) -> cairo_bool_t;
	pub fn cairo_region_translate(region: *mut cairo_region_t, dx: c_int, dy: c_int) -> c_void;
	pub fn cairo_region_subtract(dst: *mut cairo_region_t, other: *const cairo_region_t) -> cairo_status_t;
	pub fn cairo_region_subtract_rectangle(dst: *mut cairo_region_t, rectangle: *const cairo_rectangle_int_t) -> cairo_status_t;
	pub fn cairo_region_c_intersect(dst: *mut cairo_region_t, other: *const cairo_region_t) -> cairo_status_t;
	pub fn cairo_region_c_intersect_rectangle(dst: *mut cairo_region_t, rectangle: *const cairo_rectangle_int_t) -> cairo_status_t;
	pub fn cairo_region_union(dst: *mut cairo_region_t, other: *const cairo_region_t) -> cairo_status_t;
	pub fn cairo_region_union_rectangle(dst: *mut cairo_region_t, rectangle: *const cairo_rectangle_int_t) -> cairo_status_t;
	pub fn cairo_region_xor(dst: *mut cairo_region_t, other: *const cairo_region_t) -> cairo_status_t;
	pub fn cairo_region_xor_rectangle(dst: *mut cairo_region_t, rectangle: *const cairo_rectangle_int_t) -> cairo_status_t;
	pub fn cairo_debug_reset_static_data() -> c_void;
}
