extern crate winapi;
use super::{Wide, High};

pub fn termsize() -> Option<(Wide, High)>
{
	if let Some((_, csbi)) = get_csbi()
	{
		let w: Wide = Wide((csbi.srWindow.Right - csbi.srWindow.Left) as u16);
		let h: High = High((csbi.srWindow.Bottom - csbi.srWindow.Top) as u16);
		Some((w, h))
	}
	else
	{
		None
	}
}

pub fn mv_csr_up(n: usize) -> String
{
	use self::winapi::um::wincon::{SetConsoleCursorPosition, COORD};
	if let Some((hand, csbi)) = get_csbi()
	{
		unsafe
		{
			SetConsoleCursorPosition(hand,
						COORD {
							X: 0,
							Y: csbi.dwCursorPosition.Y - n as i16,
						});
		}
	}
	"".to_string()
}

fn get_csbi() -> Option<(self::winapi::shared::ntdef::HANDLE,
	use self::winapi::um::wincon::CONSOLE_SCREEN_BUFFER_INFO)>
	use self::winapi::um::processenv::GetStdHandle;
	use self::winapi::um::wibase::STD_OUTPUT_HANDLE;
	use self::winapi::um::wincon::{GetConsoleScreenBufferInfo, CONSOLE_SCREEN_BUFFER_INFO, COORD, SMALL_RECT};

	let hand: HANDLE = unsafe
	{
		GetStdHandle(STD_OUTPUT_HANDLE)
	};

	let zc = COORD
	{
		X: 0,
		Y: 0
	};

	let mut csbi = CONSOLE_SCREEN_BUFFER_INFO
	{
		dwSize: zc.clone(),
		dwCursorPosition: zc.clone(),
		wAttributes: 0,
		srWindow: SMALL_RECT
		{
			Left: 0,
			Top: 0,
			Right: 0,
			Bottom: 0,
		},
		dwMaximumWindowSize: zc,
	};
	match unsafe { GetConsoleScreenBufferInfo(hand, &mut csbi) }
	{
		0 => None,
		_ => Some((hand, csbi)),
	}
}

