use adder;
// use assert_matches::*;
// use solana_program::{
//     instruction::{AccountMeta, Instruction},
//     system_program,
//     account_info::{next_account_info, AccountInfo},
//     msg, pubkey::Pubkey,
//     system_instruction,
// };
// use solana_validator::test_validator::*;
// use solana_program_test::*;
// use solana_sdk::{signature::Signer, transaction::Transaction};


// #[test]
// fn test_validator_transaction() {
//     solana_logger::setup_with_default("solana_program_runtime=debug");
//     let program_id = Pubkey::new_unique();

//     let (test_validator, payer) = TestValidatorGenesis::default()
//         .add_program("/target/deploy/adder", program_id)
//         .start();
//     let rpc_client = test_validator.get_rpc_client();
//     solana_logger::setup_with_default("solana_runtime::message=debug");

//     let blockhash = rpc_client.get_latest_blockhash().unwrap();

//     let (pda, _bump_seed) = Pubkey::find_program_address(
//         &[payer.pubkey().as_ref()],
//         &program_id,
//     );

//     let mut transaction = Transaction::new_with_payer(
//         &[Instruction {
//             program_id,
//             accounts: vec![
//                 AccountMeta::new(payer.pubkey(), true),
//                 AccountMeta::new(pda, false),
//                 AccountMeta::new_readonly(system_program::id(), false)
//             ],
//             data: vec![1, 2, 3],
//         }],
//         Some(&payer.pubkey()),
//     );
//     transaction.sign(&[&payer], blockhash);

//     assert_matches!(rpc_client.send_and_confirm_transaction(&transaction), Ok(_));
// }