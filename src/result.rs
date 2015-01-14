use std::error::Error;
use std::num::FromPrimitive;
use std::ffi::c_str_to_bytes;
use std::str::from_utf8;
use ffi;

pub type CairoResult<T> = Result<T, CairoError>;

#[derive(Show, Copy, FromPrimitive)]
pub enum CairoError {
	NoMemory,
	InvalidRestore,
	InvalidPopGroup,
	NoCurrentPoint,
	InvalidMatrix,
	InvalidStatus,
	NullPointer,
	InvalidString,
	InvalidPathData,
	ReadError,
	WriteError,
	SurfaceFinished,
	SurfaceTypeMismatch,
	PatternTypeMismatch,
	InvalidContent,
	InvalidFormat,
	InvalidVisual,
	FileNotFound,
	InvalidDash,
	InvalidDscComment,
	InvalidIndex,
	ClipNotRepresentable,
	TempFileError,
	InvalidStride,
	FontTypeMismatch,
	UserFontImmutable,
	UserFontError,
	NegativeCount,
	InvalidClusters,
	InvalidSlant,
	InvalidWeight,
	InvalidSize,
	UserFontNotImplemented,
	DeviceTypeMismatch,
	DeviceError,
	
	Unknown
}

impl CairoError {
	pub fn as_raw(&self) -> ffi::cairo_status_t {
		let num = *self as u64;
		FromPrimitive::from_u64(num + 1).unwrap()
	}
}

pub fn from_raw<T>(ok: T, status: ffi::cairo_status_t) -> CairoResult<T> {
	let num = status as u64;
	if num == 0 {
		return Ok(ok);
	}
	Err(FromPrimitive::from_u64(num - 1).unwrap_or(CairoError::Unknown))
}

impl Error for CairoError {
	fn description(&self) -> &str {
		"A cairo error occured"
	}
	fn detail(&self) -> Option<String> {
		unsafe {
			let status = ffi::cairo_status_to_string(self.as_raw());
			let bytes = c_str_to_bytes(&status);
			Some(from_utf8(bytes).unwrap().to_string())
		}
	}
}