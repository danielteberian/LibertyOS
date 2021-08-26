#![deny(unsafe_op_in_unsafe_fn)]
#![stable(feature = "alloc_module", since = "1.28.0")]

use libcore::intrinsics;
use libcore::ptr::NonNull;
use libcore::sync::atomic::{AtomicPtr, Ordering};
use libcore::{mem, ptr};

#[stable(feature = "alloc_module", since = "1.28.0")]
#[doc(inline)]
pub use alloc_crate::alloc::*;
