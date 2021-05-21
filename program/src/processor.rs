use crate::error::TroveError;
use crate::instruction::TroveInstruction;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    log::sol_log_compute_units,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        sol_log_compute_units();

        msg!(
            "process: {}: {} accounts, data={:?}",
            program_id,
            accounts.len(),
            instruction_data
        );
        sol_log_compute_units();

        let instruction = TroveInstruction::unpack(instruction_data)?;
        sol_log_compute_units();

        match instruction {
            TroveInstruction::InitTrove {
                amountSol,
                amountUsd,
            } => {
                sol_log_compute_units();
                msg!("Instruction: InitTrove");
                Self::process_init_trove(accounts, amountSol, amountUsd, program_id)
            }
        }

        // let borrower_account = util::accounts(accounts)?;

        // msg!("Borrower account {:?}", borrower_account);

        // let metadata_address_seed = format!("zro_{}_trove", borrower_account.key);
        // msg!("Seed Str is {:?}", metadata_address_seed);
        // // let metadata_address_seed = metadata_address_seed.as_bytes();
        // let pda =
        //     // Pubkey::create_with_seed(borrower_account.key, &metadata_address_seed, program_id)?;
        // Pubkey::find_program_address(&[
        //     b"zro",
        //     &borrower_account.key.to_bytes(),
        //     b"trove"
        //     ], program_id);

        // msg!("Seed is {:?}", metadata_address_seed);
        // msg!("Generated PDA {:?}", pda);

        // Ok(())
    }

    fn process_init_trove(
        accounts: &[AccountInfo],
        amountSol: u64,
        amountUsd: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        sol_log_compute_units();
        let account_info_iter = &mut accounts.iter();

        sol_log_compute_units();
        let borrower_account = next_account_info(account_info_iter)?;
        let trove_sol_account = next_account_info(account_info_iter)?;
        let trove_usd_account = next_account_info(account_info_iter)?;
        let trove_data_account = next_account_info(account_info_iter)?;
        let trove_temp_data_account = next_account_info(account_info_iter)?;
        sol_log_compute_units();

        if !borrower_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        sol_log_compute_units();

        // let x = system_instruction::SystemInstruction::CreateAccountWithSeed();

        if *trove_sol_account.owner != spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }
        sol_log_compute_units();

        msg!(
            "Called Open Trove with {:?} SOL {:?} USD",
            amountSol,
            amountUsd
        );
        sol_log_compute_units();
        msg!(
            "trove_temp_data_account {}",
            trove_temp_data_account.key.to_string()
        );

        Ok(())
    }
}

mod util {
    use super::*;
    pub fn accounts<'a, 'b>(
        accounts: &'b [AccountInfo<'a>],
    ) -> Result<&'b AccountInfo<'a>, ProgramError> {
        let account_info_iter = &mut accounts.iter();
        let borrower_account = next_account_info(account_info_iter)?;
        Ok(borrower_account)
    }
}
