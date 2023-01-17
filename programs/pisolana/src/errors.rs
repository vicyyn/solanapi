use crate::*;

#[error_code]
pub enum CustomError {
    #[msg("Invalid Hex Block Provided")]
    InvalidHexBlockProvided,
    #[msg("Number of Hex provided has to be between 1 and 8")]
    InvalidNumberOfHexProvided,
}
