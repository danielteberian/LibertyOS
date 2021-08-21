#[derive(Debug)]
pub struct Wide(pub u16);
#[derive(Debug)]
pub struct High(pub u16);

//Configuration for various operating systems.
#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use self::unix::*;

#[cfg(win)]
mod win;
#[cfg(win)]
pub use self::win::*;

