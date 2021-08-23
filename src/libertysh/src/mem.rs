use crate::types::Str;
use object_pool::Pool

const MAXIMSIZE: usize = 64;

macro_rules! callshr
{
	($value:ident, $callback:ident) =>
	{{
		let result = $callback($value);
		if $value.len() > MAXIMSIZE
		{
			$value.truncate(MAXIMSIZE);
			$value.shrink_to_fit();
		}

		$value.clear();
		result
	}};
}

thread_local!
{
	static STRINGS: Pool<Str> = Pool::new(256, || Str::with_capacity(MAXIMSIZE));
}

pub struct LSHPool;

impl LSHPool
{
	pub fn string<T, F: FnMut(&mut Str) -> T>(mut callback: F) -> T
	{
		STRINGS.with(|pool| match pool.try_pull()
		{
			Some(ref mut string) => callshr!(string, callback),
			None => callback(&mut Str::new()),
		})
	}
}
