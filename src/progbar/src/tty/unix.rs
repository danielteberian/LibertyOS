extern crate libc;
use super::{Wide, High};

#[cfg(any(target_os = "dragonfly", target_os = "freebsd"))]
fn ioctl_conv<T: Into<libc::c_ulong>>(v: T) -> libc::c_ulong
{
	v.into()
}

#[cfg(not(any(target_os = "dragonfly", target_os = "freebsd")))]
fn ioctl_conv<T: Copy>(v: T) -> T
{
	v
}

pub fn termsize() -> Option<(Wide, High)>
{
	use self::libc::{ioctl, isatty, STDOUT_FILENO, TIOCGWINSZ, winsize};
	let istty: bool = unsafe
	{
		isatty(STDOUT_FILENO) == 1
	};

	if !istty
	{
		return None;
	}
	
	let (row, col) = unsafe
	{
		let mut winsize = winsize
		{
			winsize_r: 0,
			winsize_c: 0,
			winsize_xpxl: 0,
			winsize_ypxl: 0,
		};
		ioctl(STDOUT_FILENO, ioctl_conv(TIOCGWINSZ), &mut winsize);
		let row = if winsize.winsize_r > 0
		{
			winsize.winsize_r
		}
		else
		{
			0
		};
		let col = if winsize.winsize_c > 0
		{
			winsize.winsize_c
		}
		else
		{
			0
		};
		(row as u16, col as u16)
	};

	if row > 0 && col > 0
	{
		Some((Wide(col), High(row)))
	}
	else
	{
		None
	}
}

pub fn mv_csr_up(n: usize) -> String
{
	format!("\x1B[{}A", n)
}
