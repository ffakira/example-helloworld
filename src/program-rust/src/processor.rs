use crate::instruction::HelloWorldInstruction;
use crate::error::HelloWorldError;
use crate::state::Counter;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
};

pub struct Processor {}

impl Processor {
    pub fn process(
        program_id: &Pubkey, 
        accounts: &[AccountInfo], 
        instruction_data: &[u8]
    ) -> ProgramResult {
        let instruction = HelloWorldInstruction::unpack(instruction_data)?;

        match instruction {
            HelloWorldInstruction::Increment { amount } => {
                msg!("[helloworld_instruction -> increment]: function called");

                let accounts_iter = &mut accounts.iter();
                let account = next_account_info(accounts_iter)?;

                if account.owner != program_id {
                    return Err(HelloWorldError::IncorrectTokenId.into());
                }

                let mut counter_data = Counter::unpack_unchecked(&account.data.borrow())?;

                if !counter_data.is_initialized() {
                    return Err(HelloWorldError::NotInitialized.into());
                }
            }
        }

        Ok(())
    }
}