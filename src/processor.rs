#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(deprecated)]
#![allow(unused_variables)]
#![allow(unused_mut)]

//! Program state processor

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    msg,
    system_program,
    sysvar::{clock::Clock, rent::Rent, Sysvar},
};

use crate::state::Stake;
use crate::utils::{assert_keys_equal, get_master_address_and_bump_seed};
use spl_associated_token_account::get_associated_token_address;

use crate::{
    instruction::{ ProcessDeposit,  TokenInstruction},

};

use std::convert::TryInto;

pub struct Processor {}

impl Processor {
    pub fn process_deposit(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        //// function to deposit any token
        let account_info_iter = &mut accounts.iter();

        let sender_account = next_account_info(account_info_iter)?;
        let receiver_account = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;
        let mint_address = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;
        let rent_account = next_account_info(account_info_iter)?;
        let receiver_associate_info = next_account_info(account_info_iter)?;
        let token_associate_info = next_account_info(account_info_iter)?;
        let token_associate_address = next_account_info(account_info_iter)?;

        let stake_data = next_account_info(account_info_iter)?;

        if !sender_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let receiver_token = get_associated_token_address(receiver_account.key, mint_address.key);

        if spl_token::id() != *token_program.key && receiver_token != *receiver_associate_info.key {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if receiver_associate_info.data_is_empty() {
            invoke(
                &spl_associated_token_account::create_associated_token_account(
                    sender_account.key,
                    receiver_account.key,
                    mint_address.key,
                ),
                &[
                    sender_account.clone(),
                    receiver_account.clone(),
                    token_program.clone(),
                    mint_address.clone(),
                    system_program.clone(),
                    rent_account.clone(),
                    receiver_associate_info.clone(),
                    token_associate_info.clone(),
                    token_associate_address.clone(),
                ],
            )?;
        }

        invoke(
            &spl_token::instruction::transfer(
                token_program.key,
                token_associate_address.key,
                receiver_account.key,
                sender_account.key,
                &[sender_account.key],
                amount,
            )?,
            &[
                token_program.clone(),
                token_associate_address.clone(),
                receiver_associate_info.clone(),
                sender_account.clone(),
                system_program.clone(),
            ],
        )?;

        let mut stake = Stake::try_from_slice(&stake_data.data.borrow())?;

        stake.token_program = *token_program.key;
        stake.sender_account = *sender_account.key;
        stake.receiver_account = *receiver_account.key;
        stake.mint_address = *mint_address.key;
        stake.amount = amount;

        stake.serialize(&mut &mut stake_data.data.borrow_mut()[..])?;
        Ok(())
    }

    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        let instruction = TokenInstruction::unpack(input)?;
        match instruction {
            TokenInstruction::ProcessDeposit(ProcessDeposit { amount }) => {
                msg!("Instruction: Deposit token");
                Self::process_deposit(program_id, accounts, amount)
            }
        }
    }
}
