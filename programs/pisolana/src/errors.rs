use crate::*;

#[error_code]
pub enum CustomError {
    #[msg("Invalid Hex Block Provided")]
    InvalidHexBlockProvided,
    #[msg("Number of Hex provided has to be between 1 and 8")]
    InvalidNumberOfHexProvided,
    #[msg("number_of_hex provided will overflow the Hex Block. Please provide a smaller number")]
    HexBlockOverflow,
}
