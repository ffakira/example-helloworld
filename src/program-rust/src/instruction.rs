use std::convert::TryInto;

use solana_program::{
    program_error::ProgramError,
};

#[derive(Clone, Debug, PartialEq)]
pub enum HelloWorldInstruction {
    Increment {
        amount: u32,
    },

    Decrement {
        amount: u32,
    },
}

impl HelloWorldInstruction {
    pub fn unpack(instruction: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = instruction
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match tag {
            0 | 1 => {
                let amount = rest
                .get(..2)
                .and_then(|slice| slice.try_into().ok())
                .map(u32::from_le_bytes)
                .ok_or(ProgramError::InvalidInstructionData)?;

                match tag {
                    0 => Self::Increment { amount },
                    1 => Self::Decrement { amount },
                }
            },
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
