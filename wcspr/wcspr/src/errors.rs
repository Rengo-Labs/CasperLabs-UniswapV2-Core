
use casper_types::{ApiError};
/// Enum for FailureCode, It represents codes for different smart contract errors.
#[repr(u16)]
pub enum FailureCode {
    /// 65,536 for (UniswapV2: OVERFLOW)
    Zero = 0,
    /// 65,537 for (UniswapV2: UNDERFLOW)
    One,
}

#[repr(u16)]
pub enum Error{
    User(u16)
}
impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        let user_error = match error {
            Error::User(user_error) => user_error
        };
        ApiError::User(user_error)
    }
}
