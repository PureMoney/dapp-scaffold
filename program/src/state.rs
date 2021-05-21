use borsh::{BorshDeserialize, BorshSerialize};

use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct WorldState {
    pub is_initialized: bool,
    pub base_rate: f64,
    pub tvl: f64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Trove {
    pub debtUSD: u32,
    pub collateralSOL: u32,
    pub openDate: u32,
}
