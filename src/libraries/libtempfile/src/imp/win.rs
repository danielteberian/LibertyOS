use std::os::windows::ffi::OsStrExt;
use std::os::windows::io::{FromRawHandle, AsRawHandle, RawHandle};
use std::path::Path;
use std::io;
use std::ptr;
use std::fs::File;
use winapi::{self, DWORD, HANDLE};
use kernel32::{CreateFileW, ReOpenFile, SetFileAttributesW, MoveFileExW};
use util;
