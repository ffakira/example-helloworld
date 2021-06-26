#![cfg(not(feature = "no-entrypoint"))]

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::{
    error::{ HelloWorldError, PrintHelloWorldError},
    processor::Processor,
};

entrypoint!(process_instruction);

pub fn process_instruction<'a>(
    program_id: &Pubkey, 
    accounts: &[AccountInfo<'a>], 
    instruction_data: &[u8]) -> ProgramResult {
    if let Err(error) = Processor::process(program_id, accounts, instruction_data) {
        error.print::<HelloWorldError>();
        return Err(error);
    }
    Ok(())
}
