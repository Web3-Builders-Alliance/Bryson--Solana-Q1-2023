use anchor_lang::{
    prelude::*,
    system_program::{self}
};

use anchor_spl::token::{ Transfer as SplTransfer, TokenAccount, Mint, Token};
use solana_program::native_token::Sol;



declare_id!("3zGNcvkZXeJhsxc1tcx2wF9WLCUfoquPJbYNymmbVpb2");

#[program]
pub mod deposit_program {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
            let deposit_account = &mut ctx.accounts.deposit_account;
                deposit_account.deposit_authority = *ctx.accounts.deposit_authority.key;
                ctx.accounts.deposit_account.auth_bump = *ctx.bumps.get("pda_auth").unwrap();

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let deposit_account = &mut ctx.accounts.deposit_account;
        let deposit_authority = &ctx.accounts.deposit_authority;
        let system_program = &ctx.accounts.system_program;


        deposit_account.vault_bump = ctx.bumps.get("vault").copied();

        let cpi_accounts = system_program::Transfer {
            from: deposit_authority.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };

        let cpi = CpiContext::new(system_program.to_account_info(), cpi_accounts);

        anchor_lang::system_program::transfer(cpi, amount)?;

     

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let deposit_account = &ctx.accounts.deposit_account;
        let system_program = &ctx.accounts.system_program;
        let pda_auth = &ctx.accounts.pda_auth; 
        let vault = &mut ctx.accounts.vault;

        let cpi_accounts = system_program::Transfer {
            from: vault.to_account_info(),
            to: ctx.accounts.deposit_authority.to_account_info(),
        };

        let seeds = &[
            b"vault",
            pda_auth.to_account_info().key.as_ref(),
            &[deposit_account.vault_bump.unwrap()],
        ];

        let signer = &[&seeds[..]];

        let cpi = CpiContext::new_with_signer(system_program.to_account_info(), cpi_accounts, signer);

        anchor_lang::system_program::transfer(cpi, amount)?;




        Ok(())
    }

    pub fn deposit_spl(ctx: Context<DepositSPL>, amount: u64) -> Result<()> {
        let cpi_accounts = SplTransfer {
            from: ctx.accounts.from_token_account.to_account_info(),
            to: ctx.accounts.to_token_account.to_account_info(),
            authority: ctx.accounts.deposit_authority.to_account_info(),
        };

        let cpi = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);


        anchor_spl::token::transfer(cpi, amount)?;
        Ok(())
    }


    pub fn withdraw_spl(ctx: Context<WithdrawSPL>, amount: u64) -> Result<()> {
        let deposit_account = &mut ctx.accounts.deposit_account;
        let deposit_auth = &ctx.accounts.deposit_auth;
        let from_token_account = &mut ctx.accounts.from_token_account;
       
    
        let cpi_accounts = SplTransfer {
            from: from_token_account.to_account_info(),
            to: deposit_account.to_account_info(),
            authority: deposit_auth.to_account_info() 
        };

        let seeds = &[
            b"auth",
            deposit_account.to_account_info().key.as_ref(),
            &[deposit_account.auth_bump],
        ];

        let signer = &[&seeds[..]];

        let cpi = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            signer,
        );

        anchor_spl::token::transfer(cpi, amount)?;
        
        Ok(())
    }
}

#[derive(Accounts)] // validates incoming accounts with appropriate constraints

pub struct Initialize <'info> { 
#[account(mut)]
    pub deposit_authority: Signer<'info>,
#[account(init, payer = deposit_authority, space = SolAccount::SIZE)]
    pub deposit_account: Account<'info, SolAccount>, 
#[account(seeds = [b"auth", deposit_account.key().as_ref()], bump)]
/// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
#[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)] // validates incoming accounts
    pub struct Deposit <'info> {
        #[account(mut, has_one = deposit_authority)]
        pub deposit_account: Account<'info, SolAccount>,
        #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
        /// CHECK: no need to check this.
        pub pda_auth: UncheckedAccount<'info>,
        #[account(mut, seeds = [b"vault", pda_auth.key().as_ref()], bump)]
        pub vault: SystemAccount<'info>,
        #[account(mut)]
        pub deposit_authority: Signer<'info>,
        pub system_program: Program<'info, System>,
}



#[derive(Accounts)] // validates incoming accounts
pub struct Withdraw <'info> {
    #[account(mut, has_one = deposit_authority)]
    pub deposit_account: Account<'info, SolAccount>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"vault", pda_auth.key().as_ref()], bump)]
    pub vault: SystemAccount<'info>,
    #[account(mut)]
    pub deposit_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositSPL<'info> {
 #[account(mut, has_one = deposit_authority)]
    pub deposit_account: Account<'info, SolAccount>,
 #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump = deposit_account.auth_bump)]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    pub to_token_account: Account<'info, TokenAccount>,
    pub from_token_account: Account<'info, TokenAccount>,
 #[account(mut)]
    pub deposit_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub mint: Account<'info, Mint>,
}


#[derive(Accounts)]
pub struct WithdrawSPL<'info> {
    #[account(mut)]
    pub deposit_account: Account<'info, SolAccount>,
    #[account(seeds = [b"auth", deposit_account.key().as_ref()], bump )]
    /// CHECK: no need to check this.
    pub pda_auth: UncheckedAccount<'info>,
    pub from_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub deposit_auth: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}


// Borrowed this structuring from Richards code. Good way of organizing it.
#[account]
pub struct SolAccount {
    pub deposit_authority: Pubkey, //Need owner's Pubkey(me)
    pub auth_bump: u8,
    pub vault_bump: Option<u8>
}

impl SolAccount {
    const SIZE: usize = 
    8  +   // anchor space
    32 +   // deposit authority pubkey 32
    8  +   // auth bump 8
    8;     // vault bump 8
   
}









// Error Handling
#[error_code]
pub enum ErrorCode {
    #[msg("Not the Owner of this Bank Account!")]
    NotAuthorizedOwnerOfAccount,
    #[msg("Amount of Lamports cannot be Zero")]
    AmountSOLCannotBeZERO,
}

