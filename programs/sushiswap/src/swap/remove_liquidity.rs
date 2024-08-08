use std::ffi::c_long;

use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer, Token, TokenAccount, Mint,Burn,burn};
use lptoken::cpi::accounts::{MintTokenTo,};

use crate::{RemoveLiquidity, customerror::error::Errors, TokenPairMetadata};

use super::swap::{ mint_fee};

pub fn remove_liquidity_direct(ctx:Context<RemoveLiquidity>,liquidity:u64,amount_x_min:u64,amount_y_min:u64) -> Result<()> {
    
    let metadata = &ctx.accounts.token_pair_metadata_account;
    let (balance_x, balance_y) = (ctx.accounts.from_associated_account_for_x.amount,ctx.accounts.from_associated_account_for_y.amount);
    
    let fees = mint_fee(metadata,&ctx.accounts.sushilptoken);
    
    msg!("fees ++++++++++++++++++ {}",fees);

    if fees > 0 {
        mint_lp_fees_remove(&ctx, fees)?;
    }

    let total_lp_supply = ctx.accounts.sushilptoken.supply;
    
    let amount_x = ((balance_x as u128) * (liquidity as u128) / (total_lp_supply as u128)) as u64 ;
    let amount_y = ((balance_y as u128) * (liquidity as u128) / (total_lp_supply as u128)) as u64;
    
    require!(amount_x >= amount_x_min, Errors::ErrorInsufficientXAmount);
    require!(amount_y >= amount_y_min, Errors::ErrorInsufficientYAmount);

    require!(amount_x > 0 && amount_y > 0, Errors::ErrorInsufficientLiquidityAmountBurned);

    msg!("++++++++++++++ first {}{}",amount_x,amount_y);
    
    burn_lp(&ctx, liquidity)?;

    msg!("++++++++++++++ second");

    transfer_coin_from_pool(&ctx.accounts.token_program, &ctx.accounts.from_associated_account_for_x, &ctx.accounts.to_associated_account_for_x, &ctx.accounts.token_pair_metadata_account, amount_x)?;
    
    transfer_coin_from_pool(&ctx.accounts.token_program, &ctx.accounts.from_associated_account_for_y, &ctx.accounts.to_associated_account_for_y, &ctx.accounts.token_pair_metadata_account, amount_y)?;


    let metadata2 = &mut ctx.accounts.token_pair_metadata_account.as_mut();

    msg!("Break 1");
    
    metadata2.reserve_x = ctx.accounts.from_associated_account_for_x.amount - amount_x;
    metadata2.reserve_y = ctx.accounts.from_associated_account_for_y.amount - amount_y;
    
    msg!("Break 2");
    
    let clock = Clock::get()?;
    
    metadata2.block_timestamp_last = clock.unix_timestamp;
    
    metadata2.k_last = (metadata2.reserve_x as u128) * (metadata2.reserve_y as u128)  as u128;
    
    msg!("Break 3");

    
    Ok(())
}


fn mint_lp_fees_remove(ctx:&Context<RemoveLiquidity>,fees:u64) -> Result<()> {

    let cpi_program = ctx.accounts.sushi_token_program.to_account_info();
    let cpi_accounts = MintTokenTo{
        authority: ctx.accounts.user.to_account_info(),
        sushilptoken:ctx.accounts.sushilptoken.to_account_info(),
        to: ctx.accounts.associated_account_for_fees.to_account_info(),
        vault: ctx.accounts.vault.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
        token_x:ctx.accounts.token_x.to_account_info(),
        token_y:ctx.accounts.token_y.to_account_info()
    };

    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

    lptoken::cpi::mint_token_to(cpi_context, fees)
}

fn burn_lp(ctx:&Context<RemoveLiquidity>,amount:u64)->Result<()>{
    
    burn(CpiContext::new(ctx.accounts.token_program.to_account_info(),
     
     Burn { 
        mint: ctx.accounts.sushilptoken.to_account_info(), 
        from: ctx.accounts.associated_account_for_sushilp.to_account_info(), 
        authority: ctx.accounts.user.to_account_info() }
    ), amount)?;

    
    Ok(())

}


pub fn transfer_coin_from_pool<'info>(token_program:&Program<'info,Token>,from:&Account<'info,TokenAccount>,to:&Account<'info,TokenAccount>,metadata:&Account<'info,TokenPairMetadata>,amount:u64)->Result<()>{

    require!(from.amount > amount,Errors::ErrorInsufficientAmount);

   transfer(
    CpiContext::new_with_signer(
    token_program.to_account_info(),
    Transfer{
        from:from.to_account_info(),
        to:to.to_account_info(),
        authority:metadata.to_account_info()
   }, 
   &[&[b"TokenPairMetadataTS",metadata.token_x.key().as_ref(),metadata.token_y.key().as_ref(),&[metadata.own_bump]]])
   , amount)?;
   
   Ok(())
}


