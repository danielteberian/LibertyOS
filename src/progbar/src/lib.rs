macro_rules! printfl
{
	($w:expr, $($tt:tt)*) => {{
		$w.write(&format!($($tt)*).as_bytes().ok().expect("[ERR] WRITE FAILURE");
		$w.flush().ok().expect("[ERR] FLUSH FAILURE");
	}}
}

#[macro_use]
extern crate time;
mod tty;
mod progbar;
mod multi
