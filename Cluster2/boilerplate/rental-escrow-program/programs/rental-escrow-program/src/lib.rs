use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_spl::token::{self, CloseAccount, SetAuthority, TokenAccount, Transfer, Token};


// replace this Program ID once deployed.
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod boilerplate_v1 {
    use super::*;

    //initialize some account
    pub fn initialize_rent_escrow(ctx: Context<InitializeRentEscrow>, initializer_metaticket_nft_amount_usdc: u64, renters_metaticket_nft_amount: u64 ) -> Result<()> {
        
        Ok(())
    }

    pub fn exchange_usdc_for_nft(ctx: Context<Exchange> ) -> Result<()> {
        
        Ok(())
    }

    pub fn cancel_rent_escrow(ctx: Context<CancelRentEscrow> ) -> Result<()> {
        
        Ok(())
    }
    
}

#[derive(Accounts)]
pub struct InitializeRentEscrow<'info, > {
   pub initializer: Signer<'info>,
   pub temp_usdc_account: Account<'info, TokenAccount>,
   pub nft_to_receive_account: Account<'info, TokenAccount>,
   pub escrow_account: Account<'info, MetaTicketRentEscrowAccount>,
   pub token_program: Program<'info, Token>,
   pub system_program: Program<'info, System>
}

//initiates the echange ... USDC for Sub-Minted NFT MetaTicket
#[derive(Accounts)]
pub struct Exchange {
    // TODO
}

#[derive(Accounts)]
pub struct CancelRentEscrow {
    // TODO
}





// Program Escrow Account
#[account]
pub struct MetaTicketRentEscrowAccount {


                           // Is this Escrow Initialized? //
//***//------/------/------/------/------/------/------/------/------/------/------  

    pub is_initialized: bool,


                           // ACCOUNTS NEEDED //
//***//------/------/------/------/------/------/------/------/------/------/------  
    // this is the NFT purchasers public key
  pub initializer_key: Pubkey,
    // account created to hold USDC for the metaticket and is assigned to the PDA of the escrow
  pub initializer_temporary_usdc_account: Pubkey,
    // this is the account created to hold the subminted NFT ticket
  pub initializer_receive_metaticket_nft_account: Pubkey,

                           // AMOUNTS //
//***//------/------/------/------/------/------/------/------/------/------/------      
    // this is the amount of usdc the intializer agrees to pay for the NFT
  pub initializer_usdc_amount: u64,
    // this is how many metatickets the user wishes to buy
  pub renters_metaticket_nft_amount: u64,
//***///------/------/------/------/------/------/------/------/------/------/------                
}


const DISCRIMINATOR_LENGTH: usize = 8;
const BOOL_LENGTH: usize = 1;
const PUBLIC_KEY_LENGTH: usize = 32;
const U64_LENGTH: usize = 8;

impl MetaTicketRentEscrowAccount {
    const LEN: usize =  
    DISCRIMINATOR_LENGTH +
    BOOL_LENGTH +
    PUBLIC_KEY_LENGTH +
    U64_LENGTH;
}


impl<'info> From<&mut InitializeRentEscrow<'info>> for CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
    fn from(accounts: &mut InitializeRentEscrow<'info>) -> Self {
        let cpi_accounts = SetAuthority {
            current_authority: todo!(),
            account_or_mint: todo!(),
            
        };
        // let cpi_program = accounts.token_program.to_account_info();
        // CpiContext::new(cpi_program, cpi_accounts)
    }
}
