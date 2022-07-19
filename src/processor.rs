use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    borsh::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    msg, pubkey::Pubkey,
    program_error::ProgramError,
    sysvar::{rent::Rent, Sysvar},
    system_instruction,
    program::invoke_signed,
};
use borsh::BorshSerialize;
use crate::state::TestState;

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!(
        "process_instruction: {}: {} accounts, data={:?}",
        program_id,
        accounts.len(),
        instruction_data
    );

    let account_info_iter = &mut accounts.iter();
    let initializer = next_account_info(account_info_iter)?;
    let test_acct = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref()],
        program_id,
    );
    if pda != *test_acct.key {
        msg!("Invalid seeds for PDA");
        return Err(ProgramError::InvalidArgument);
    }

    let account_len: usize = 1000;

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            test_acct.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[
            initializer.clone(),
            test_acct.clone(),
            system_program.clone(),
        ],
        &[&[
            initializer.key.as_ref(),
            &[bump_seed],
        ]],
    )?;

    msg!("Initializer pubkey: {}", initializer.key);
    if !initializer.is_signer {
        msg!("Account is missing signature: {}", initializer.key);
        return Err(ProgramError::MissingRequiredSignature)
    }

    let mut test_data = try_from_slice_unchecked::<TestState>(&test_acct.data.borrow()).unwrap();
    test_data.num = 1;

    test_data.serialize(&mut &mut test_acct.data.borrow_mut()[..])?;
    Ok(())
}
