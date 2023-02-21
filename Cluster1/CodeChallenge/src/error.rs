use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("NotRentExempt")]
    NotRentExempt,

    #[error("Escrow Time Locked")]
    EscrowTimeLocked,

    #[error("Timeout 1000 Reached")]
    Timeout,
    
    #[error("Expected Amount Mismatch")]
    ExpectedAmountMismatch,

    #[error("AmountOverflow")]
    AmountOverflow,
    #[error("TransferDidNotOccur")]
    TransferDidNotOccur,

}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
} 
