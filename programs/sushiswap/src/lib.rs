

pub mod customerror;
pub mod swaputils;
pub mod swap;

use customerror::*;
use swaputils::*;
use swap::*;

use anchor_lang::prelude::*;
use anchor_spl::{token::{Token, TokenAccount, Mint}, associated_token::AssociatedToken};
use lptoken::cpi::accounts::{InitializeMintAccount,BurnUserToken};
use lptoken::program::Lptoken;
use lptoken::{self,Vault};


declare_id!("9mkw5Vx1as4GPeufggiCkzy1EB71hkAJDJTC3mpqZUUQ");

#[program]
pub mod sushiswap {

    use crate::swap::{swap::{self, transfer_coin}, remove_liquidity::remove_liquidity_direct, swap_exact_input::swap_exact_input};

    use super::*;
    use customerror::error::Errors;

    pub fn create_pair(ctx: Context<CreatePair>,own_bump:u8) -> Result<()> {
        
        let metadata= &mut ctx.accounts.token_pair_metadata_account;

        metadata.creator = ctx.accounts.user.key();
        metadata.k_last = 0;
        metadata.fee_amount_account = ctx.accounts.sushilptoken.key();
        metadata.token_x= ctx.accounts.token_x.key();
        metadata.token_y = ctx.accounts.token_y.key();



        metadata.reserve_x = 0;
        metadata.reserve_y = 0;

        metadata.block_timestamp_last = 0;
        
        metadata.own_bump = own_bump;

        let cpi_program = ctx.accounts.sushi_token_program.to_account_info();

        let cpi_accounts = InitializeMintAccount{
            owner:ctx.accounts.user.to_account_info(),
            associated_account:ctx.accounts.associated_account_for_sushilp.to_account_info(),
            associated_token_program:ctx.accounts.associated_token_program.to_account_info(),
            sushilptoken:ctx.accounts.sushilptoken.to_account_info(),
            token_program:ctx.accounts.token_program.to_account_info(),
            vault:ctx.accounts.vault.to_account_info(),
            rent:ctx.accounts.rent.to_account_info(),
            system_program:ctx.accounts.system_program.to_account_info(),
            token_x:ctx.accounts.token_x.to_account_info(),
            token_y:ctx.accounts.token_y.to_account_info(),
            associated_account_for_fees:ctx.accounts.associated_account_for_fees.to_account_info()
        };
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        
        lptoken::cpi::initialize(cpi_context)
        
        // Ok(())

    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>,amount_x_desired: u64,amount_y_desired: u64,amount_x_min: u64,amount_y_min: u64,)->Result<()>{
        
        let metadata = &ctx.accounts.token_pair_metadata_account;

        require!(ctx.accounts.from_associated_account_for_x.amount > amount_x_desired,Errors::ErrorInsufficientLiquidityAmount);
        require!(ctx.accounts.from_associated_account_for_y.amount > amount_y_desired,Errors::ErrorInsufficientLiquidityAmount);


        require!(ctx.accounts.to_associated_account_for_x.owner.key() == metadata.key(),Errors::ErrorInvalidToAccount);    
        require!(ctx.accounts.to_associated_account_for_y.owner.key() == metadata.key(),Errors::ErrorInvalidToAccount);    
        
        require!(ctx.accounts.from_associated_account_for_x.owner.key() == ctx.accounts.user.key(),Errors::ErrorInvalidFromAccount);    
        require!(ctx.accounts.from_associated_account_for_y.owner.key() == ctx.accounts.user.key(),Errors::ErrorInvalidFromAccount);    
       
        
        require!(ctx.accounts.associated_account_for_sushilp.owner.key() == ctx.accounts.user.key(),Errors::ErrorInvalidFromAccount);    
    

        swap::add_liquidity_direct(ctx,amount_x_desired, amount_y_desired,amount_x_min,amount_y_min)
   


    }

    pub fn remove_liquidity(ctx: Context<RemoveLiquidity>,liquidity: u64,amount_x_min: u64,amount_y_min: u64)->Result<()>{
        
        let metadata = &ctx.accounts.token_pair_metadata_account;

        require!(ctx.accounts.associated_account_for_sushilp.amount >= liquidity,Errors::ErrorInsufficientLiquidityAmount);


        require!(ctx.accounts.from_associated_account_for_x.owner.key() == metadata.key(),Errors::ErrorInvalidFromAccount);    
        require!(ctx.accounts.from_associated_account_for_y.owner.key() == metadata.key(),Errors::ErrorInvalidFromAccount);    
        
        require!(ctx.accounts.to_associated_account_for_x.owner.key() == ctx.accounts.user.key(),Errors::ErrorInvalidToAccount);    
        require!(ctx.accounts.to_associated_account_for_y.owner.key() == ctx.accounts.user.key(),Errors::ErrorInvalidToAccount);    
       
        
        require!(ctx.accounts.associated_account_for_sushilp.owner.key() == ctx.accounts.user.key(),Errors::ErrorInvalidFromAccount);    
    

        remove_liquidity_direct(ctx, liquidity, amount_x_min, amount_y_min)

    }

    pub fn swap(ctx: Context<Swap>,x_in: u64,y_min_out: u64,swap_x_to_y:bool)->Result<()>{
        
        let metadata = &ctx.accounts.token_pair_metadata_account;

        require!(ctx.accounts.pool_associated_account_for_x.owner.key() == metadata.key(),Errors::ErrorInvalidFromAccount);    
        require!(ctx.accounts.pool_associated_account_for_y.owner.key() == metadata.key(),Errors::ErrorInvalidFromAccount);    
        
        require!(ctx.accounts.to_associated_account_for_x.owner.key() == ctx.accounts.user.key(),Errors::ErrorInvalidToAccount);    
        require!(ctx.accounts.to_associated_account_for_y.owner.key() == ctx.accounts.user.key(),Errors::ErrorInvalidToAccount);    
       
       swap_exact_input(ctx, x_in, y_min_out,swap_x_to_y)
    

    }

    // fn quote(amount_x: u64, reserve_x: u64, reserve_y: u64) -> Result<u64> {
        
    //     require!(amount_x > 0, Errors::ErrorInsufficientAmount);

    //     require!(reserve_x > 0 && reserve_y > 0, Errors::ErrorInsufficientLiquidity);
        

    //     Ok((((amount_x as u128) * (reserve_y as u128) / (reserve_x as u128)) as u64))
    // }



    
    
 
   
}




#[derive(Accounts)]
pub struct CreatePair<'info> {
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(
        init,
        seeds = [b"TokenPairMetadataTS",token_x.key().as_ref(),token_y.key().as_ref()],
        bump,
        payer = user,
        space = TokenPairMetadata::SPACE,
        )]
    pub token_pair_metadata_account: Account<'info, TokenPairMetadata>,
    #[account(
        init,
        payer = user,
        associated_token::mint = token_x,
        associated_token::authority = token_pair_metadata_account,
    )]
    pub associated_account_for_x: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = user,
        associated_token::mint = token_y,
        associated_token::authority = token_pair_metadata_account,
    )]
    pub associated_account_for_y: Account<'info, TokenAccount>,
    /// CHECK:this for test
    #[account(mut)]
    pub associated_account_for_fees: AccountInfo<'info>,
    /// CHECK:this for test
    #[account(mut)]
    pub associated_account_for_sushilp: AccountInfo<'info>,
    pub token_x : Account<'info,Mint>,
    pub token_y : Account<'info,Mint>,
    /// CHECK:
    #[account(mut)]
    pub sushilptoken :AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub vault : AccountInfo<'info>,
    pub token_program : Program<'info,Token>,
    pub system_program: Program<'info, System>,
    pub sushi_token_program: Program<'info, Lptoken>,
    pub associated_token_program : Program<'info,AssociatedToken>,
    pub rent : Sysvar<'info,Rent>
}

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(
        seeds = [b"TokenPairMetadataTS",token_pair_metadata_account.token_x.key().as_ref(),token_pair_metadata_account.token_y.key().as_ref()],
        bump = token_pair_metadata_account.own_bump,
        )]
    #[account(mut)]
    pub token_pair_metadata_account: Box<Account<'info, TokenPairMetadata>>,
    #[account(mut)]
    pub from_associated_account_for_x: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub from_associated_account_for_y: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub to_associated_account_for_x: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub to_associated_account_for_y: Box<Account<'info, TokenAccount>>,
    /// CHECK:this for test
     #[account(mut)]
     pub associated_account_for_fees: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub associated_account_for_sushilp: Box<Account<'info,TokenAccount>>,
    /// CHECK:
    #[account(mut)]
    pub sushilptoken :Box<Account<'info,Mint>>,
    /// CHECK:
    #[account(mut)]
    pub vault : AccountInfo<'info>,
    pub token_program : Program<'info,Token>,
    pub system_program: Program<'info, System>,
    pub sushi_token_program: Program<'info, Lptoken>,
    pub associated_token_program : Program<'info,AssociatedToken>,
    pub rent : Sysvar<'info,Rent>,
    pub token_x : Box<Account<'info,Mint>>,
    pub token_y : Box<Account<'info,Mint>>,


}


#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
    
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(
        seeds = [b"TokenPairMetadataTS",token_pair_metadata_account.token_x.key().as_ref(),token_pair_metadata_account.token_y.key().as_ref()],
        bump = token_pair_metadata_account.own_bump,
        )]
    #[account(mut)]
    pub token_pair_metadata_account: Box<Account<'info, TokenPairMetadata>>,
    #[account(mut)]
    pub from_associated_account_for_x: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub from_associated_account_for_y: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub to_associated_account_for_x: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub to_associated_account_for_y: Box<Account<'info, TokenAccount>>,
    /// CHECK:this for test
     #[account(mut)]
     pub associated_account_for_fees: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub associated_account_for_sushilp: Box<Account<'info,TokenAccount>>,
    /// CHECK:
    #[account(mut)]
    pub sushilptoken :Box<Account<'info,Mint>>,
    /// CHECK:
    #[account(mut)]
    pub vault : AccountInfo<'info>,
    pub token_program : Program<'info,Token>,
    pub system_program: Program<'info, System>,
    pub sushi_token_program: Program<'info, Lptoken>,
    pub associated_token_program : Program<'info,AssociatedToken>,
    pub rent : Sysvar<'info,Rent>,
    pub token_x : Account<'info,Mint>,
    pub token_y : Account<'info,Mint>,

}


#[derive(Accounts)]
pub struct Swap<'info> {
    
    #[account(mut)]
    pub user:Signer<'info>,
    #[account(
        seeds = [b"TokenPairMetadataTS",token_pair_metadata_account.token_x.key().as_ref(),token_pair_metadata_account.token_y.key().as_ref()],
        bump = token_pair_metadata_account.own_bump,
        )]
    #[account(mut)]
    pub token_pair_metadata_account: Box<Account<'info, TokenPairMetadata>>,
    #[account(mut)]
    pub pool_associated_account_for_x: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub pool_associated_account_for_y: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub to_associated_account_for_x: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub to_associated_account_for_y: Box<Account<'info, TokenAccount>>,
    /// CHECK:this for test
     #[account(mut)]
     pub associated_account_for_fees: AccountInfo<'info>,
    /// CHECK:
    #[account(mut)]
    pub associated_account_for_sushilp: Box<Account<'info,TokenAccount>>,
    /// CHECK:
    #[account(mut)]
    pub sushilptoken :Box<Account<'info,Mint>>,
    /// CHECK:
    #[account(mut)]
    pub vault : AccountInfo<'info>,
    pub token_program : Program<'info,Token>,
    pub system_program: Program<'info, System>,
    pub sushi_token_program: Program<'info, Lptoken>,
    pub associated_token_program : Program<'info,AssociatedToken>,
    pub rent : Sysvar<'info,Rent>,
    pub token_x : Account<'info,Mint>,
    pub token_y : Account<'info,Mint>,

}



#[account]
pub struct TokenPairMetadata {
     /// The admin of the token pair
    pub creator: Pubkey,
    /// fee amount , record fee amount which is not withdrawed
    pub fee_amount_account: Pubkey,
    /// It's reserve_x * reserve_y, as of immediately after the most recent liquidity event
    pub k_last: u128,
    /// T0 token balance
    pub token_x:Pubkey,
    /// T1 token balance
    pub token_y:Pubkey,

    pub reserve_x: u64,
    pub reserve_y: u64,
    pub block_timestamp_last: i64,

    pub token_x_bump:u8,
    pub token_y_bump:u8,
    pub own_bump:u8,

    pub authority:Pubkey
   
}


impl TokenPairMetadata {
    
    pub const SPACE:usize = 8 + 32 + 32 + 16 + 32 + 32 + 8 + 1 + 1 + 1 + 32 +100;
}





