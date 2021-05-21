use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use crate::error::TroveError::InvalidInstruction;

pub enum TroveInstruction {
    /// Starts the trade by creating and populating a trove account and transferring ownership of the
    /// given SOL token account to the PDA
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]`   (Borrower) The account of the person initializing the borrowing
    /// 1. `[writable]` (Trove SOL Account) Trove SOL token account that should be created prior to this instruction and owned by the initializer
    /// 2. `[writable]` (Trove USD Account) The initializer's USD token account for the USD token they will receive should the trade go through
    /// 3. `[writable]` (Trove Data Account) The associated trove account, it will hold all necessary info about the trade.
    /// 4. `[writable]` (Temp Data Account) The associated trove account, it will hold all necessary info about the trade.
    /// 5. `[]`         The rent sysvar
    /// 6. `[]`         The token program
    InitTrove {
        /// The amount of SOL deposited
        amountSol: u64,
        /// The amount of USD borrowed
        amountUsd: u64,
    },
}

impl TroveInstruction {
    /// Unpacks a byte buffer into a [TroveInstruction](enum.TroveInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitTrove {
                amountSol: Self::unpack_amount_sol(rest)?,
                amountUsd: Self::unpack_amount_usd(rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount_sol(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(0..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
    fn unpack_amount_usd(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(8..16)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}
