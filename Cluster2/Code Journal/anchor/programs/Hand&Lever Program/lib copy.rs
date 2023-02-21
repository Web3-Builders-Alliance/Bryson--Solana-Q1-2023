
// pull in the anchor crate
use anchor_lang::prelude::*;



// this is the hard codes program ID (will have to change once deployed) 
declare_id!("EnjN3cm7xYqYHNUZbQfhJYj5S5RBrSU9tc5aHwQ6LqvT");

// this is the program module where we write the business logic
#[program]
pub mod lever {
    use super::*;

    // this is our endpoint that takes in a 'Context' type as its first argument
    // this context argument allows us to access accounts through ctx.account
    // Context is in essence a container that holds the account data
    // this context allows us to get access to the program id, the accounts, and the
    //custom instruction data.
    // this function is an instruction 
    pub fn initialize(_ctx: Context<InitializeLever>) -> Result<()> {
        // no logic here, it seems that we just want to take in a account to intialize

        Ok(())
    }


    // this is another instruction  that sets the power status. 
    // another argument name is passed in because we want to know
    // who pulled the power switch
    pub fn switch_power(ctx: Context<SetPowerStatus>, name: String) -> Result<()> {
    
    // setting power to be a mutable account
        let power = &mut ctx.accounts.power;
    // if power is on we will get a boolean
        power.is_on = !power.is_on;
    // message telling us that the user is interacting with the switch
        msg!("{} is pulling the power switch!", &name);
    
    // here we match to see if power is on. if it is true we display a message
    // power is on. If power is not on the power is now off. 
        match power.is_on {
            true => msg!("The power is now on."),
            false => msg!("The power is now off!"),
        };

        Ok(())
    }
}

// this is where we validate the incoming accounts
// here the accounts inherit the account trait
#[derive(Accounts)]
                            // 'info is a lifetime variable (accountinfo class that contain types)
pub struct InitializeLever<'info> {

    // Create the account, payer is to be the user interacting with switch, space allocation
    #[account(init, payer = user, space = 8 + 8)]
    //this is the power account 
    pub power: Account<'info, PowerStatus>,
    //we set this account to mutable

    #[account(mut)]
    // this is the user interacting with the account, we expect the user to be a signer
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetPowerStatus<'info> {
    // we set account to mutable so we can change data, specifically the power data
    // power is on or is off. We can change this
    #[account(mut)]
    pub power: Account<'info, PowerStatus>,
}

// this is the PowerStatus account
#[account]
pub struct PowerStatus {
    pub is_on: bool,
}