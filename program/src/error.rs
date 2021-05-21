use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum TroveError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    /// Not Rent Exempt
    #[error("Not Rent Exempt")]
    NotRentExempt,
    /// ExpectedAmountMismatch
    #[error("ExpectedAmountMismatch")]
    ExpectedAmountMismatch,
    /// AmountOverflow
    #[error("AmountOverflow")]
    AmountOverflow,
    /// AmountOverflow
    #[error("AccountParsingError")]
    AccountParsingError,
}

impl From<TroveError> for ProgramError {
    fn from(e: TroveError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
