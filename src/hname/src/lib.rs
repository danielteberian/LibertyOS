#![cfg_attr(all(feature = "unstable", test), feature(test))]

#[cfg(windows)]
extern crate winutil;

#[cfg(any(unix, target_os = "liberty"))]
extern crate libc;

#[cfg(windows)]
pub fn get_hname() -> Option<String>
{
	winutil::get_computer_name()
}

#[cfg(any(unix, target_os = "liberty"))]
extern "C"
{
	fn gethname(name: *mut libc::c_char, size: libc::size_t) -> libc::c_int;
}

#[cfg(any(unix, target_os = "liberty"))]
use std::ffi::CStr;

#[cfg(any(unix, target_os = "liberty"))
pub fn get_hname() -> Option<String>
{
	let len = 255;
	let mut buf = Vec::<u8>::with_capacity(len);
	let ptr = buf.as_mut_ptr() as *mut libc::c_char;

	unsafe
	{
		if gethname(ptr, len as libc::size_t) != 0
		{
			return None;
		}

		Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
	}
}
