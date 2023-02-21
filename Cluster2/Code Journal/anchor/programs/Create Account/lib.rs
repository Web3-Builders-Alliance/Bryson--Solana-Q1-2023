use anchor_lang::prelude::*;

use instructions::*;

pub mod instructions;
pub mod state;

// Program ID
declare_id!("FFKtnYFyzPj1qFjE9epkrfYHJwZMdh8CvJrB6XsKeFVz");

// this is the program module where our business logic is created
#[program]
pub mod anchor_program_example {
    use super::*;

    // this is the endpoint that takes Context type as its first argument.
    // this context argument allows us to access accounts through ctx.account
    // we can also pass in other data as arguments as well including
    // name(string), house_number( u8), street (string), city(string)

    pub fn create_address_info(
        ctx: Context<CreateAddressInfo>,
        name: String,
        house_number: u8,
        street: String,
        city: String,
    ) -> Result<()> {
        

        // here we access the instruction create crate that is local in our project
        // this crate allows us to create the address with the logic in the 
        //instruction/create
        // the logic in this create_address function to generate a new address and create a new 
        //account      
        instructions::create::create_address_info(
            ctx,
            name,
            house_number,
            street,
            city,
        )
    }
}