use core::{mem, slice};
use core::ops::{Deref, DerefMuf};
use super::err::{Error, Result};

macro_rules! syscall
{
