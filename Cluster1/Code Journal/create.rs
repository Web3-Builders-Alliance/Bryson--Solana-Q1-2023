
// here we pull in the crates using borsh which is a deserializer and serializer.
// we must deserialize and serialize data accordingly to pass into the program
use borsh::{ BorshDeserialize, BorshSerialize };

// here we pull in the solana_program crates to gain access to the the crate
// for this program we will need to access account_info and also iterate through
// accounts and their info with nxt_account_info
// we will need access to the BPF loaders program entrypoint
// we will be bringing the CPI from program::invoke (invoking a program)
// we need to determine rent allocated
// we will need to bring in instructions and constructors from system program 
// access sysvar for special accounts with dynamically updated data to calculate rent exemptions
use solana_program::{
    account_info::{ AccountInfo, next_account_info },
    entrypoint::ProgramResult, 
    program::invoke,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    system_program,
    sysvar::Sysvar,
};


// This will gain access to our state crate and pull from address_info.rs
// the structs we have impliments under AddressInfo
use crate::state::AddressInfo;

// a public function called create_address_info that takes in three arguments
// 1) program_id - which has a reference to a Public Key
// 2) accounts - which has a reference to an array of AccountInfo. AccountInfo
//               holds multiple fields such as is_signer, is_owner, pubkey of the account
//               lamports in the account, data held in account, executable bool, rent epoch
//               and is_writeable
// 3) AddressInfo - this is the data taken from struct AddressInfo in state
pub fn create_address_info(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    address_info: AddressInfo,
) -> ProgramResult {

//   we will create a variable with let, we make accounts mutible and iterare through the accounts
    let accounts_iter = &mut accounts.iter();
//  we will create a variable with left and make it equalt to a convenience function called 
//  next_account_info that takes in the previous variable account_iter


    let address_info_account = next_account_info(accounts_iter)?;
//  next we will iterate to the payer account
    let payer = next_account_info(accounts_iter)?;
//  next we will get the system_program
    let system_program = next_account_info(accounts_iter)?;


//   we will get the length in bytes
    let account_span = (address_info.try_to_vec()?).len();
//   we will determin minimum amount of lamports needed in rent
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

// here we invoke the system program to create an account where we pass in payers pubkey,
// the address_info_account pubkey, the lamports required to be exempt, the space
// needed for allocation of bytes, and the owner which is the program_id
    invoke(
        &system_instruction::create_account(
            &payer.key,
            &address_info_account.key,
            lamports_required,
            account_span as u64,
            program_id,
        ),
        &[
            // we copy the payer values, the address_info_account values, system_prorgam values
            payer.clone(), address_info_account.clone(), system_program.clone()
        ]
    )?;
    
    address_info.serialize(&mut &mut address_info_account.data.borrow_mut()[..])?;
    Ok(())
}

// What are the concepts (borrowing, ownership, vectors etc)
// We are using variables(let), references, arrays(see AccountInfo), types (see u64), we are invoking system_programs
// to create accounts, we also clone values (see payer.clone etc), we see mutability as well.

//What is the organization?
// 1) Pull in crates
// 2) Create a public function called create_address_info that takes in 3 arguments
//    program_if, accounts, address_info.
// 3) we will set up our iterator to iterate through expected acounts and pop off to
//    next account expected.
// 4) We will set our allocation of account space and also determine rent based on size
// 5) We will invoke system_instructions from the system program and create 
//    address_info_account
// 6) we will clone the payer, address_info_account, and system_program values
// 7) we will then serialize the address_info
// 8) this will either compile or return an error.

//What is the contract doing? What is the mechanism? 
// program will create an account 



//How could it be better? More efficient? Safer?
