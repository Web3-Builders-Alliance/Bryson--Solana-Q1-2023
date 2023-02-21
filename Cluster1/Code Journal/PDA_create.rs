use borsh::{ BorshDeserialize, BorshSerialize };
use solana_program::{
    account_info::{ AccountInfo, next_account_info },
    entrypoint::ProgramResult, 
    program::{ invoke, invoke_signed },
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    system_program,
    sysvar::Sysvar,
};

use crate::state::PageVisits;

// here we will create a function create_page_visits and pass in program_id, accounts, and page_visits from the state
pub fn create_page_visits(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    page_visits: PageVisits,
) -> ProgramResult {

    // set iterator
    let accounts_iter = &mut accounts.iter();

    // accounts we expect and iterate/pop them to the next account
    let page_visits_account = next_account_info(accounts_iter)?;
    let user = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    //determine size of account page_visits using try_to_vec and getting length in bytes
    let account_span = (page_visits.try_to_vec()?).len();
    // determine amount of rent to keep account page_visits exempt
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    //invoke signed here as are using pda
    invoke_signed(
        &system_instruction::create_account(
            &payer.key,
            &page_visits_account.key,
            lamports_required,
            account_span as u64,
            program_id,
        ),
        &[
            payer.clone(), page_visits_account.clone(), system_program.clone()
        ],

        //Here we are using signers seeds, passing in as byte slice, converting it into a reference,
        //The signer_seeds parameter is a slice of u8 slices where the inner slices represent the seeds 
        //plus the bump seed used to derive (with Pubkey::find_program_address) one of the PDAs within the 
        //account_infos slice of AccountInfos. 

        // this is using the SEED_PREFIX from PageVisits and converting it from a
        // string slice to a byte slice, then coverting it to a reference.
        // then we convert the nusers Pubkey into a reference,
        // then we create the PDA that holds the state of the page visits for that user


        &[&[
            PageVisits::SEED_PREFIX.as_bytes().as_ref(),
            user.key.as_ref(),
            &[page_visits.bump],
        ]]
    )?;

    Ok(())
}



//What are the concepts (borrowing, ownership, vectors etc)?
// There are a lot of concepts in here from functions that take in arguments, and within the functions we
// set multiple variables including iterators, ther expected accounts we iterate through.
// In the processor we use the match function to know what to do with instructions.
// There many places where references(&) are used. 
// We utilize types for example account_span as u64.
//

//What is the organization?
// 1) Pull in necessary creates.
// 2) Create will create page visits and we pass in the expected accounts, setting allocation,
//    determining amount of lamports based on size of account to allocate.
// 3) get signers seeds and bumping it to create a PDA account.


//What is the contract doing? What is the mechanism? 
// It is using incrementing page visits and using a PDA to hold the state of page visists for a user.

//How could it be better? More efficient? Safer?
// I am still unsure as I am just gaining the grasp of these concepts.
