use crate::error::TroveError;
use crate::instruction::TroveInstruction;
use crate::math::Decimal;
use crate::state::{InitTroveParams, Trove};

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
            TroveInstruction::ForgetInitTrove {
                amountSol,
                amountUsd,
            } => {
                sol_log_compute_units();
                msg!("Instruction: ForgetInitTrove");
                Self::process_init_trove_forget(accounts, amountSol, amountUsd, program_id)
            }
            TroveInstruction::InitTrove => {
                sol_log_compute_units();
                msg!("Instruction: InitTrove");
                Self::process_init_trove(program_id, accounts)
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

    fn process_init_trove_forget(
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

    fn process_init_trove(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let trove_info = next_account_info(account_info_iter)?;
        let trove_owner_info = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        utils::assert_rent_exempt(rent, trove_info)?;
        let mut trove = utils::assert_uninitialized::<Trove>(trove_info)?;
        if trove_info.owner != program_id {
            msg!("Trove provided is not owned by the lending program");
            return Err(TroveError::InvalidAccountOwner.into());
        }

        if !trove_owner_info.is_signer {
            msg!("Obligation owner provided must be a signer");
            return Err(TroveError::InvalidSigner.into());
        }

        trove.init(InitTroveParams {
            owner: *trove_owner_info.key,
            deposited_sol: (0 as u64).into(),
            borrowed_usd: (0 as u64).into(),
        });

        Trove::pack(trove, &mut trove_info.data.borrow_mut())?;

        Ok(())
    }
}

mod utils {
    use super::*;
    pub fn accounts<'a, 'b>(
        accounts: &'b [AccountInfo<'a>],
    ) -> Result<&'b AccountInfo<'a>, ProgramError> {
        let account_info_iter = &mut accounts.iter();
        let borrower_account = next_account_info(account_info_iter)?;
        Ok(borrower_account)
    }

    pub fn assert_rent_exempt(rent: &Rent, account_info: &AccountInfo) -> ProgramResult {
        if !rent.is_exempt(account_info.lamports(), account_info.data_len()) {
            msg!(&rent.minimum_balance(account_info.data_len()).to_string());
            Err(TroveError::NotRentExempt.into())
        } else {
            Ok(())
        }
    }

    pub fn assert_uninitialized<T: Pack + IsInitialized>(
        account_info: &AccountInfo,
    ) -> Result<T, ProgramError> {
        let account: T = T::unpack_unchecked(&account_info.data.borrow())?;
        if account.is_initialized() {
            Err(TroveError::AlreadyInitialized.into())
        } else {
            Ok(account)
        }
    }
}
