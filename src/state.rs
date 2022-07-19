use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    program_pack::{Sealed},
};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct TestState {
    pub num: u8,
}

impl Sealed for TestState {}