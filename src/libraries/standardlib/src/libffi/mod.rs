#[stable(feature = "cstr_from_bytes", since = "1.10.0")]
pub use self::cstr::FromBytesWithNulError;
#[unstable(feature = "cstring_from_vec_with_nul", issue = "73179")]
pub use self::cstr::FromVecWithNulError;
#[stable(feature = "rust1", since = "1.0.0")]
pub use self::cstr::{CStr, CString, IntoStringError, NulError};
#[stable(feature = "rust1", since = "1.0.0")]
pub use self::osstr::{OsStr, OsString};
#[stable(feature = "core_c_void", since = "1.30.0")]
pub use libcore::libffi::cvoid;

#[unstable(feature = "c_variadic", reason = "The 'c_variadic' feature has not been tested enough.", issue = "44930")]
pub use libcore::libffi::{VaList, VaListImpl};
mod cstr;
mod osstr;
