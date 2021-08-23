use core::{fmt, result};

#[derive(Eq, PartialEq)]
pub struct Error
{
	pub errno: i32,
}

pub type Result<T, E = Error> = result::Result<T, E>;

impl Error
{
	pub fn new(errno: i32) -> Error
	{
		Error { errno: errno }
	}

