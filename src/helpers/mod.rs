pub type StdResult<T, E> = std::result::Result<T, E>;
pub type StdError = dyn std::error::Error;

pub mod encrypt;