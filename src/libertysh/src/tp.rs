use small;
use smallvec::SmallVec;
pub use types_rs::{array, types::*};

pub use crate::shell::flow_control::Function;
pub type Args = SmallVec<[small::Stringl; 4]>;

#[macro_export]
macro_rules! args
[
	( $($x:expr), *) => ({
		let mut _arr = $crate::types::Args::new();
		$(_arr.push($x.into());)*
		_arr
	})
];
