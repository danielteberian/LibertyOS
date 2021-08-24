use crate::FileTime;
use libc::{time_t, timespec};
use std::fs;
use std::os::unix::prelude::*;

cfg_if::cfg_if!
{
	if #[cfg(target_os = "linux")]
	{
		mod utime;
		mod linux;
		pub use self::linux::*;
	}
	else if #[cfg(target_os = "android")]
	{
		mod android;
		pub use self::android::*;
	}
	else if #[cfg(target_os = "macos", 
		      target_os = "darwin"))]
	{
		mod utime;
		mod macos;
		pub use self::macos::*;
	}
	else if #[cfg(any(target_os = "solaris",
			  target_os = "illumos",
			  target_os = "emscripten",
			  target_os = "freebsd",
			  target_os = "netbsd",
			  target_os = "openbsd",
			  target_os = "haiku"))]
	{
		mod utimensat;
		pub use self::utimensat::*;
	}
	else
	{
		mod utime;
		pub use self::utime::*;
	}
}

#[allow(dead_code)]
fn to_tmspec(ft: &Option<FileTime>) -> timespec
{
	cfg_if::cfg_if!
	{
		if #[cfg(any(target_os = "macos"
			     target_os = "darwin
