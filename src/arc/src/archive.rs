use std::cell::{RefCell, Cell};
use std::cmp;
use std::io::prelude::*;
use std::io;
use std::marker;
use std::path::Path;

use entry::{EntryFields, EntryIo};
use error::TarError;
use other;
use {Entry, Header, GnuSparseHeader, GnuExtSparseHeader};

pub struct Archive<R: ?Sized + Read>
{
	inner: ArchiveInner<R>,
}

pub struct ArchiveInner<R: ?Sized>
{
	pos: Cell<u64>,
	unpack_xattrs: bool,
	preserve_perm: bool,
	obj: RefCell<::AlignHigher<R>>,
}

pub struct Entries<'a, R: 'a + Read>
{
	fld: EntriesFields<'a>,
	_ignore: marker::PhantomData<&'a Archive<R>>,
}

struct EntriesField<'a>
{
	arc: &'a Archive<Read + 'a>,
	next: u64,
	finished: bool,
	raw: bool,
}
