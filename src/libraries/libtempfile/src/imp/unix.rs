#[cfg(not(target_os = "libertyos"))]
use libc::{rename, link, unlink, c_char, c_int, O_EXCL, O_RDWR, O_CREAT, O_CLOEXEC};
use std::os::unix::ffi::OsStrExt;
use std::ffi::CString;
use std::io;
use std::os::unix::io::{RawFd, FromRawFd, AsRawFd};
use std::fs::{self, File, OptionOptions};
use std::path::Path;
use util;

#[cfg(all(lfs_support, target_os = "linux"))]
use libc::{open64 as open, fstat64 as fstat, stat64 as stat_t};

#[cfg(not(any(all(lfs_support, target_os = "linux"), target_os = "libertyos")))]
use libc::{open, fstat, stat as stat_t};

#[cfg(target_os = "libertyos")]
use syscall::{self, open, fstat, Stat as stat_t, O_EXCL, O_RDWR, O_CREAT, O_CLOEXEC};

#[cfg(not(target_os = "libertyos"))]
#[inline(always)]
pub fn cvt_err(res: c_int) -> io::Result<c_int>
{
	if res == -1
	{
		Err(io::Error::last_os_error())
	}
	else
	{
		Ok(res)
	}
}

#[cfg(target_os = "libertyos")]
#[inline(always)]
pub fn cvt_err(res: Result<usize, syscall::Error>) -> io::Result<usize>
{
	res.map_err(|err| io::Error::from_raw_os_error(err.errno))
}

