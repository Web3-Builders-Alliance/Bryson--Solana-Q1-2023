// pulling in crates

use borsh::{ BorshDeserialize, BorshSerialize };
use solana_program::{
    account_info::{
        next_account_info, AccountInfo
    },
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);


// create a  function - process_instruction, pass in arguments seen below, then use match to
//determine if we want to get the power status or to set the power status on or off
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    match PowerStatus::try_from_slice(&instruction_data) {
        Ok(power_status) => return initialize(program_id, accounts, power_status),
        Err(_) => {},
    }

    match SetPowerStatus::try_from_slice(&instruction_data) {
        Ok(set_power_status) => return switch_power(accounts, set_power_status.name),
        Err(_) => {},
    }
//Error handling
    Err(ProgramError::InvalidInstructionData)
}


// pub function initialize will take in program_id of the hand program, accounts expected,
// and the power status

pub fn initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    power_status: PowerStatus,
) -> ProgramResult {

    // set iterator to account_iter variable
    let accounts_iter = &mut accounts.iter();

    // set power, user, and system program accounts to iterate. Accounts we expect.
    let power = next_account_info(accounts_iter)?;
    let user = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // set the size of the accounts power_status using try_to_vec and getting its length in bytes ".len()"
    let account_span = (power_status.try_to_vec()?).len();

    // get the necessary amount of lamports to keep account rent free utilizing the size of the account 
    //and calculating the necessary amount of lamports from account size
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    // lets call system_instruction (from the solana_program crate) to create these accounts
    // 1) user account 
    // 2) power account
    // 3) lamports_required
    // 4) space allocation 
    // 5) program_id (owner)

    invoke(
        &system_instruction::create_account(
            &user.key,
            &power.key,
            lamports_required,
            account_span as u64,
            program_id,
        ),
        // copy values from user, power, and system program
        &[
            user.clone(), power.clone(), system_program.clone()
        ]
    )?;
    // now to serialize the power_status power.data   
    power_status.serialize(&mut &mut power.data.borrow_mut()[..])?;

    Ok(())
}


// function switch_power that takes in accounts expected and the name argument that is a string
pub fn switch_power(
    accounts: &[AccountInfo],
    name: String,
) -> ProgramResult {
// set acount_iter
    let accounts_iter = &mut accounts.iter();

    // power account (iterater and pop to next)
    let power = next_account_info(accounts_iter)?;
    
    // we set mutability to the power_status to change its data in the account
    let mut power_status = PowerStatus::try_from_slice(&power.data.borrow())?;
    power_status.is_on = !power_status.is_on;

    // we are serializing power_status now writing new data to account power.
    power_status.serialize(&mut &mut power.data.borrow_mut()[..])?;

    //message to tell us if power is on or off
    msg!("{} is pulling the power switch!", &name);

    //matching power_status.is_on from the struct PowerStatus
    match power_status.is_on {
        true => msg!("The power is now on."),
        false => msg!("The power is now off!"),
    };

    Ok(())
}


//state of accounts with the data structures in structs
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct SetPowerStatus {
    pub name: String,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct PowerStatus {
    pub is_on: bool,
}

// What are the concepts (borrowing, ownership, vectors etc)
// We are using functions and we are using match functions (acting as processor for program),
// We are also working with variables(let)
// AccountsInfo is an array of AccountInfo fields
// we also see mutability(&mut) and borrowing(borrow_mut)
//

//What is the organization?
//pull in crates
//create a function process_instruction that we can pass into entrypoint
//use match functions to determine what instructions to pass
// function initialize that passes in 3 args(program_id, accounts, powerstatus),
// then we have the expected accounts we iterate through.
// we determine size of powerstatus account and rent needed
// then we invoke system program / system instructions to create account

//What is the contract doing? What is the mechanism? 
// The contract is a hand and lever program. We will be invoking the lever prorgam
// and switching the power on and off



//How could it be better? More efficient? Safer?
// To be honest Im not sure yet. We could add more parameters to turning on the power and make sure
// we check if there is enough power to switch on, or set time paramiters so the switch could
// only be turned on at certain hours etc. I do not know enough yet to suggest betetr changes, but soon enough I will.