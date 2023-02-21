use solana_program::{ instruction::{AccountMeta, Instruction},
    program_error::ProgramError, pubkey::Pubkey};
use std::convert::TryInto;

use crate::error::EscrowError::InvalidInstruction;


pub enum EscrowInstruction {
    InitEscrow { amount: u64 },
    Exchange {amount: u64},
    Cancel  {},
    ResetTimeLock{},

}

impl EscrowInstruction {
    /// Unpacks a byte buffer into a [EscrowInstruction](enum.EscrowInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitEscrow {
                amount: Self::unpack_amount(rest)?,
            },
            1 => Self::Exchange {
                amount: Self::unpack_amount(rest)?
            },
            2 => Self::Cancel {},

            3 => Self::ResetTimeLock {},

            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }

    fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(std::mem::size_of::<Self>());
        match &*self {
            Self::InitEscrow { amount } => {
                buf.push(0);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::Exchange { amount } => {
                buf.push(1);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::ResetTimeLock {} => {
                buf.push(2);
            }
            Self::Cancel {} => {
                buf.push(3);
            }
        }
        buf
    }
}




pub fn init_escrow(
    program_id: &Pubkey,
    initiator: &Pubkey,
    pda_temp_token_acct: &Pubkey,
    init_token_acct: &Pubkey,
    escrow_account: &Pubkey,
    token_program: &Pubkey,
    amount: u64,
) -> Result<Instruction, ProgramError> {
    let data = EscrowInstruction::InitEscrow { amount }.pack();
    let accounts = vec![
        AccountMeta::new(*initiator, true),
        AccountMeta::new(*pda_temp_token_acct, false),
        AccountMeta::new_readonly(*init_token_acct, false),
        AccountMeta::new(*escrow_account, false),
        AccountMeta::new_readonly(*token_program, false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

pub fn exchange(
    program_id: &Pubkey,
    taker: &Pubkey,
    taker_token_account: &Pubkey,  
    taker_token_account2: &Pubkey, 
    temp_token_account: &Pubkey,   
    initializer_token_account: &Pubkey,
    initializer_main_account: &Pubkey,
    escrow_account: &Pubkey,
    token_program: &Pubkey,
    amount: u64,
) -> Result<Instruction, ProgramError> {
    let data = EscrowInstruction::Exchange { amount }.pack();
    let accounts = vec![
        AccountMeta::new(*taker, true),
        AccountMeta::new(*taker_token_account, false),
        AccountMeta::new(*taker_token_account2, false),
        AccountMeta::new(*temp_token_account, false),
        AccountMeta::new(*initializer_token_account, false),
        AccountMeta::new(*initializer_main_account, false),
        AccountMeta::new(*escrow_account, false),
        AccountMeta::new_readonly(*token_program, false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}



pub fn process_reset_time_lock(
    program_id: &Pubkey,
    initiator: &Pubkey,
    escrow_account: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let data = EscrowInstruction::ResetTimeLock {}.pack();
    let accounts = vec![
        AccountMeta::new(*initiator, true),
        AccountMeta::new(*escrow_account, false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

pub fn process_cancel(
    program_id: &Pubkey,
    initiator: &Pubkey,
    temp_token_account: &Pubkey,
    initializer_token_account: &Pubkey,
    escrow_account: &Pubkey,
    token_program: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let data = EscrowInstruction::Cancel {}.pack();
    let accounts = vec![
        AccountMeta::new(*initiator, true),
        AccountMeta::new(*temp_token_account, false),
        AccountMeta::new_readonly(*initializer_token_account, false),
        AccountMeta::new(*escrow_account, false),
        AccountMeta::new_readonly(*token_program, false),
    ];
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

