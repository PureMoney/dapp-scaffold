use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::{Pubkey, PUBKEY_BYTES},
};

use crate::math::Decimal;

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

use crate::state_utils::{PROGRAM_VERSION, UNINITIALIZED_VERSION};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct WorldState {
    pub is_initialized: bool,
    pub base_rate: f64,
    pub tvl: f64,
}

/// Lending market obligation state
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Trove {
    /// Version of the struct
    pub version: u8,
    /// Owner authority which can borrow usd
    pub owner: Pubkey,
    pub deposited_sol: Decimal,
    pub borrowed_usd: Decimal,
}

impl Trove {
    /// Create a new trove
    pub fn new(params: InitTroveParams) -> Self {
        let mut trove = Self::default();
        Self::init(&mut trove, params);
        trove
    }

    /// Initialize a trove
    pub fn init(&mut self, params: InitTroveParams) {
        self.version = PROGRAM_VERSION;
        self.owner = params.owner;
        self.deposited_sol = params.deposited_sol;
        self.borrowed_usd = params.borrowed_usd;
    }
}

/// Initialize an obligation
pub struct InitTroveParams {
    /// Owner authority which can borrow usd
    pub owner: Pubkey,
    /// Deposited collateral for the trove
    pub deposited_sol: Decimal,
    /// Borrowed usd for the trove
    pub borrowed_usd: Decimal,
}

impl Sealed for Trove {}

impl IsInitialized for Trove {
    fn is_initialized(&self) -> bool {
        self.version != UNINITIALIZED_VERSION
    }
}

// type     -> bytes
// Pubkey   -> 32
// u8       -> 1
// u64      -> 8
// Decimal  -> 16

const U8_BYTES: usize = 1;
const DECIMAL_BYTES: usize = 16;
const TROVE_LEN: usize = 65; // 1 + 32 + 16 + 16

impl Pack for Trove {
    const LEN: usize = TROVE_LEN;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let output = array_mut_ref![dst, 0, TROVE_LEN];
        #[allow(clippy::ptr_offset_with_cast)]
        let (version, owner, deposited_sol, borrowed_usd) =
            mut_array_refs![output, U8_BYTES, PUBKEY_BYTES, DECIMAL_BYTES, DECIMAL_BYTES];

        *version = self.version.to_le_bytes();
        owner.copy_from_slice(self.owner.as_ref());
        utils::pack_decimal(self.deposited_sol, deposited_sol);
        utils::pack_decimal(self.borrowed_usd, borrowed_usd);
    }

    /// Unpacks a byte buffer into an [ObligationInfo](struct.ObligationInfo.html).
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let input = array_ref![src, 0, TROVE_LEN];
        #[allow(clippy::ptr_offset_with_cast)]
        let (version, owner, deposited_sol, borrowed_usd) =
            array_refs![input, U8_BYTES, PUBKEY_BYTES, DECIMAL_BYTES, DECIMAL_BYTES];

        let version = u8::from_le_bytes(*version);
        if version > PROGRAM_VERSION {
            msg!("Trove version does not match lending program version");
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(Self {
            version,
            owner: Pubkey::new_from_array(*owner),
            deposited_sol: utils::unpack_decimal(deposited_sol),
            borrowed_usd: utils::unpack_decimal(borrowed_usd),
        })
    }
}

mod utils {
    use super::*;
    pub fn pack_decimal(decimal: Decimal, dst: &mut [u8; 16]) {
        *dst = decimal
            .to_scaled_val()
            .expect("Decimal cannot be packed")
            .to_le_bytes();
    }

    pub fn unpack_decimal(src: &[u8; 16]) -> Decimal {
        Decimal::from_scaled_val(u128::from_le_bytes(*src))
    }

    pub fn pack_bool(boolean: bool, dst: &mut [u8; 1]) {
        *dst = (boolean as u8).to_le_bytes()
    }
}
