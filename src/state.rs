///into state.rs
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

//deposit nfts
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Stake {
    pub token_program: Pubkey,
    pub sender_account: Pubkey,
    pub receiver_account: Pubkey,
    pub mint_address: Pubkey,
    pub amount: u64,
}
