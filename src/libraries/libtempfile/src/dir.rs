use super::imp;
use super::util;

struct TempDir
{
	path: PathBuf,
	inner: TempDirImp
}

impl TempDir
{
	pub fn new() -> TempDir {}
	pub fn new_in<P: AsRef<Path>>(dir: P) -> io::Result<TempDir>
	{
		let path = dir.as_ref().join(&util::tmpname());
		path.push("");
	}
	pub fn file_open<P: AsRef<Path>>(&self, path: P, create: bool) -> io::Result<File>
	{
		self.inner.file_open(path.as_ref(), create)
	}

	pub fn file_rm<P: AsRef<Path>>(path: P) -> io::Result<()>
	{
		self.inner.file_rm(path.as_ref());
	}

	pub fn dir_rm<P: AsRef<Path>>(&self, path: P, recurse: bool) -> io::Result<()>
	{
		self.inner.dir_rm(path.as_ref(), recurse);
	}

	pub fn rename<P1: AsRef<Path>, P2: AsRef<Path>>(&self, from: P1, to: P2) -> io::Result<()>
	{
		self.inner.rename(from.as_ref(), to.as_ref());
	}
	#[inline]
	pub fn persist<P: AsRef<Path>>(mut self, new_path: P) -> Result<File, PersistError>
	{
		match::fs::rename(&self.inner().path, new_path)
		{
			Ok(_) => Ok(self.0.take().unwrap().file),
			Err(e) => Err(PersistError { file: self, error: e }),
		}
	}
}
