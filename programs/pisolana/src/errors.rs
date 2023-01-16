use crate::*;

#[error_code]
pub enum CustomError {
    #[msg("Invalid Digits Block Provided")]
    InvalidDigitsBlockProvided,
}
