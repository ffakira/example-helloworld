use num_derive::FromPrimitive as DeriveFromPrimitive;
use num_traits::FromPrimitive;

use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{ PrintProgramError, ProgramError },
};
use thiserror::Error;

pub use solana_program::program_error::PrintProgramError as PrintHelloWorldError;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum HelloWorldError {
    #[error("Invalid instruction")]
    InvalidInstruction,

    #[error("Invalid owner")]
    InvalidOwner,

    #[error("Data not been initialized")]
    NotInitialized,

    #[error("Incorrect token id")]
    IncorrectTokenId,

    #[error("Unsafe underflow")]
    UnsafeUnderflow,
}

impl From<HelloWorldError> for ProgramError {
    fn from(e: HelloWorldError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for HelloWorldError {
    fn type_of() -> &'static str {
        "HelloWorldError"
    }
}

impl PrintHelloWorldError for HelloWorldError {
    fn print<E>(&self) where 
        E: 'static + std::error::Error + DecodeError<E> PrintHelloWorldError + FromPrimitive {
        match self {
            HelloWorldError::InvalidInstruction => msg!("Error: Invalid instruction"),
            HelloWorldError::InvalidOwner => msg!("Error: invalid owner"),
            HelloWorldError::IncorrectTokenId => msg!("Error: incorrect token id"),
            HelloWorldError::UnsafeUnderflow => msg!("Error: Operation underflow"),
        }
    }
}