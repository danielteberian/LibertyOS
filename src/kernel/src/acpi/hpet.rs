use core::{mem, ptr};
use core::intrinsics::{volatile_load, volatile_store};
use crate::memory::Frame;
use crate::paging::{ActivePageTable, PhysicalAddress, Page, PageFlags, VirtualAddress};

