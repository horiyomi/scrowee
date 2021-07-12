use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {

  /// Invalid instruction error
  #[error("Invalid Instruction")]
  InvalidInstruction,

  #[error("Not rent exempt")]
  NotRentExempt,

  #[error("Expected amount mismatched")]
  ExpectedAmountMismatch,

  #[error("Amount overflow")]
  AmountOverflow,
}

impl From<EscrowError> for ProgramError {
  fn from(e: EscrowError) -> Self {
    ProgramError::Custom(e as u32)
  }
}