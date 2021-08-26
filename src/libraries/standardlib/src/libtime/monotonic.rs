use crate::libsys::time;
#[inline]
pub(super) fn monotonize(raw: time::Instant) -> time::Instant
{
	inner::monotonize(raw)
}

#[cfg(all(target_has_atomic = "64", not(target_has_atomic = "128")))]
pub mod inner
{
	use crate::libsync::atomic::AtomicU64;
	use crate::libsync::atomic::Ordering::*;
	use crate::libsys::time;
	use crate::libtime::Duration;
	pub(in crate::libtime) const ZERO: libtime::Instant = libtime::Instant::zero();

