use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Counter {
    pub owner: Pubkey,
    pub amount: u32,
    pub initialized: bool,
}

impl Sealed for Counter {}

impl IsInitialized for Counter {
    fn is_initialized(&self) -> bool {
        self.initialized
    }
}

impl Pack for Counter {
    const LEN: usize = 32 + 4 + 1;

    // Unpack data from [u8] to the data struct
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, 37];
        let (owner, amount, initialized) = array_refs![src, 32, 4, 1];

        Ok(Counter {
            owner: Pubkey::new_from_array(*owner),
            amount: u32::from_le_bytes(*amount),
            initialized: match initialized {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            },
        })
    }

    // Pack data from the data struct to [u8]
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 37];
        let (dst_owner, dst_amount, dst_initialized) = mut_array_refs![dst, 32, 4, 1];
        let &Counter {ref owner, amount, initialized} = self;

        dst_owner.copy_from_slice(owner.as_ref());
        *dst_amount = amount.to_le_bytes();
        *dst_initialized = [initialized as u8];
    }
}