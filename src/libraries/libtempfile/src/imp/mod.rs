#[cfg(any(unix, target_os = "libertyos"))]
mod unix;

#[cfg(any(unix, target_os = "libertyos"))]
pub use self::unix::*;

#[cfg(windows)]
mod win;

#[cfg(windows)]
pub use self::win::*;
