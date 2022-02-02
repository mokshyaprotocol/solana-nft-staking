//! Instruction types
use crate::errors::TokenError;
use solana_program::program_error::ProgramError;
use std::convert::TryInto;

pub struct ProcessDeposit {
    pub amount: u64,
}


pub enum TokenInstruction {
    ProcessDeposit(ProcessDeposit),
}

impl TokenInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        use TokenError::InvalidInstruction;
        let (&tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag {
            0 => {
                let (amount, _rest) = rest.split_at(8);
                let amount = amount
                    .try_into()
                    .map(u64::from_le_bytes)
                    .or(Err(InvalidInstruction))?;                
                Self::ProcessDeposit(ProcessDeposit {amount})
            }
            _ => return Err(TokenError::InvalidInstruction.into()),
        })
    }
}
