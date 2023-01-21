use crate::*;

#[error_code]
pub enum CustomError {
    #[msg("Invalid Hex Block Provided.")]
    InvalidHexBlockProvided,
    #[msg("Number of Hex provided has to be between 1 and 10.")]
    InvalidNumberOfHexProvided,
    #[msg("number_of_hex provided will overflow the Hex Block. Please provide a smaller number.")]
    HexBlockOverflow,
    #[msg("Pi Already Minted. You cannot update a minted Pi.")]
    PiAlreadyMinted,
    #[msg("Next Block has to be initialized.")]
    NextBlockNotInitialized,
    #[msg("Hex Block Already Initialized.")]
    HexBlockAlreadyInitialized,
    #[msg("You have to close all hex block accounts before closing the pi account.")]
    CannotClosePiBeforeHexBlock,
}
