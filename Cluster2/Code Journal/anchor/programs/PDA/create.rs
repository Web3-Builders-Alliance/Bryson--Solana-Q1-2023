use anchor_lang::prelude::*;


// pull in the local state/PageVisits crates
use crate::state::PageVisits;


  // this is our endpoint that takes in a 'Context' type as its first argument
    // this context argument allows us to access accounts through ctx.account
    // Context is in essence a container that holds the account data
    // this context allows us to get access to the program id, the accounts, and the
    //custom instruction data.
    // this function is an instruction 
pub fn create_page_visits(
    ctx: Context<CreatePageVisits>
) -> Result<()> {
    // here we are setting a new PDA to every page visit.
    ctx.accounts.page_visits.set_inner(
        PageVisits::new(
            0,
            *ctx.bumps.get(PageVisits::SEED_PREFIX).expect("Bump not found."),
        )
    );
    Ok(())
}

// here we want to validate the accounts coming int
#[derive(Accounts)]
pub struct CreatePageVisits<'info> {
    // here the page visits account we check that its intialized, there is space allocated
    // that the payer is the Signer, and that the PDA is correct for that user because
    // we take in the users key as a reference as well as the seed(s).
    #[account(
        init,
        space = PageVisits::ACCOUNT_SPACE,
        payer = payer,
        seeds = [
            PageVisits::SEED_PREFIX.as_bytes().as_ref(),
            user.key().as_ref(),
        ],
        bump,
    )]
    // checking that the page visits accounts AccountInfo is correct with the Account wrapper
    page_visits: Account<'info, PageVisits>,
    // thecking that the user is owned by the system program
    user: SystemAccount<'info>,
    // the Signers account must be mutable
    #[account(mut)]
    payer: Signer<'info>,

    // no checks on system program, but we must expect the system program to be owner 
    system_program: Program<'info, System>,
}