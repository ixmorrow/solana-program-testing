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

    msg!("Initializer pubkey: {}", initializer.key);
    if !initializer.is_signer {
        msg!("Account is missing signature: {}", initializer.key);
        return Err(ProgramError::MissingRequiredSignature)
    }

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref()],
        program_id,
    );

    msg!("PDA pubkey: {}", pda);
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

    let mut test_data = try_from_slice_unchecked::<TestState>(&test_acct.data.borrow()).unwrap();
    test_data.num = 1;
    msg!("PDA data: {}", test_data.num);

    test_data.serialize(&mut &mut test_acct.data.borrow_mut()[..])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        assert_matches::*,
        solana_program::{
            instruction::{AccountMeta, Instruction},
            system_program
        },
        solana_program_test::*,
        solana_sdk::{signature::Signer, transaction::Transaction},
    };

    #[tokio::test]
    async fn it_works() {
        let program_id = Pubkey::new_unique();

        let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
            "adder",
            program_id,
            processor!(process_instruction),
        )
        .start()
        .await;

        let (pda, _bump_seed) = Pubkey::find_program_address(
            &[payer.pubkey().as_ref()],
            &program_id,
        );

        let mut transaction = Transaction::new_with_payer(
            &[Instruction {
                program_id,
                accounts: vec![
                    AccountMeta::new(payer.pubkey(), true),
                    AccountMeta::new(pda, false),
                    AccountMeta::new_readonly(system_program::id(), false)
                ],
                data: vec![1, 2, 3],
            }],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);


        assert_matches!(banks_client.process_transaction(transaction).await, Ok(_));
    }
}
