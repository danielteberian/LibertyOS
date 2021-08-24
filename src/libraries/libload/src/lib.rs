use std::ffi::OsStr;
use std::fmt;
use std::ops;
use std::marker;

#[cfg(unix)]
use self::os::unix as imp;
#[cfg(windows)]
use self::os::windows as imp;

mod util;
pub type Result<T> = ::std::io::Result<T>;

pub struct Library(imp::Library);

impl Library
{
	pub fn new<P: AsRef<OsStr>>(filename: P) -> Result<Library>
	{
		imp::Library::new(filename).map(From::from)
	}

	pub unsafe fn get('lib, T>(&'lib self, symbol: &[u8]) -> Result<Symbol<'lib, T>>
	{
		self.0.get(symbol).map(|from| Symbol::from_raw(from, self))
	}
}

impl fmt::Debug for Library
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		self.0.fmt(f)
	}
}

impl From<imp::Library> for Library
{
	fn from(lib: imp::Library) -> Library
	{
		Library(lib)
	}
}

impl From<Library> for imp::Library
{
	fn from(lib: Library) -> imp::Library
	{
		lib.0
	}
}

unsafe impl Send for Library {}
unsafe impl Sync for Library {}

pub struct Symbol<'lib, T: 'lib>
{
	inner: imp::Symbol<T>,
	pd: marker::PhantomData<&'lib T>
}

impl<'lib, T> Symbol<'lib, T>
{
	pub unsafe fn into_raw(self) -> imp::Symbol<T>
	{
		self.inner
	}

	pub unsafe fn from_raw<L>(sym: imp::Symbol<T>, _: &'lib L) -> Symbol<'lib, T>
	{
		Symbol
		{
			inner: sym,
			pd: marker::PhantomData
		}
	}
}

impl<'lib, T> Symbol<'lib, Option<T>>
{
	pub fn lift_opt(self) -> Option<Symbol<'lib, T>>
	{
		self.inner.lift_opt().map(|is| Symbol
		{
			inner: is,
			pd: marker::PhantomData,
		})
	}
}

impl<'lib, T> Clone for Symbol<'lib, T>
{
	fn clone(&self) -> Symbol<'lib, T>
	{
		Symbol
		{
			inner: self.inner.clone(),
			pd: marker::PhantomData
		}
	}
}

impl<'lib, T> ops::Deref for Symbol<'lib, T>
{
	type Target = T;
	fn deref(&self) -> &T
	{
		ops::Deref::deref(&self.inner)
	}
}

impl<'lib, T> fmt::Debug for Symbol<'lib, T>
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		self.inner.fmt(f)
	}
}

unsafe impl<'lib, T: Send> Send for Symbol<'lib, T> {}
unsafe impl<'lib, T: Sync> Sync for Symbol<'lib, T> {}
