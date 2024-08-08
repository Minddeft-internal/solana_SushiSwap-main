use anchor_lang::prelude::*;
use anchor_spl::token::{Transfer,transfer, TokenAccount, Token, Mint,};
use lptoken::cpi::accounts::MintTokenTo;
use num_integer::Roots;

use crate::{swaputils::swap_utils, AddLiquidity, TokenPairMetadata, customerror::error::Errors};

const MINIMUM_LIQUIDITY: u128 = 1000;


pub fn add_liquidity_direct(mut ctx:Context<AddLiquidity>,amount_x:u64,amount_y:u64,amount_x_min:u64,amount_y_min:u64)-> Result<()> {
    let (reserve_x, reserve_y) = (ctx.accounts.token_pair_metadata_account.reserve_x,ctx.accounts.token_pair_metadata_account.reserve_y);

        let (a_x, a_y) = if reserve_x == 0 && reserve_y == 0 {
            (amount_x, amount_y)
        } else {
            let amount_y_optimal = swap_utils::quote(amount_x, reserve_x, reserve_y)?;
           
            if amount_y_optimal <= amount_y {
                (amount_x, amount_y_optimal)
            } else {
                let amount_x_optimal = swap_utils::quote(amount_y, reserve_y, reserve_x)?;
                require!(amount_x_optimal <= amount_x,Errors::ErrorInvalidAmount);
                (amount_x_optimal, amount_y)
            }
        };

        require!(a_x <= amount_x, Errors::ErrorInsufficientAmount);
        require!(a_y <= amount_y, Errors::ErrorInsufficientAmount);


        let x_coin_left = amount_x - a_x;
        let y_coin_left = amount_y - a_y;

        require!((amount_x - x_coin_left ) >= amount_x_min, Errors::ErrorInsufficientXAmount);
        require!((amount_y - y_coin_left ) >= amount_y_min, Errors::ErrorInsufficientYAmount);

        transfer_coin(&ctx.accounts.token_program, &ctx.accounts.from_associated_account_for_x, &ctx.accounts.to_associated_account_for_x, &ctx.accounts.user, amount_x - x_coin_left)?;
        transfer_coin(&ctx.accounts.token_program, &ctx.accounts.from_associated_account_for_y, &ctx.accounts.to_associated_account_for_y, &ctx.accounts.user, amount_y - y_coin_left)?;

        let (liquidity,_fees)= mint(&mut ctx,amount_x - x_coin_left,amount_y - y_coin_left)?;

        require!(liquidity > 0, Errors::ErrorInsufficientLiquidity);

        mint_lp_token(&ctx,liquidity)?;
        
        Ok(())


}


pub fn transfer_coin<'info>(token_program:&Program<'info,Token>,from:&Account<'info,TokenAccount>,to:&Account<'info,TokenAccount>,authority:&Signer<'info>,amount:u64,)->Result<()>{

    transfer(
        CpiContext::new(
            token_program.to_account_info()
            , Transfer { from:from.to_account_info() , to:to.to_account_info(), authority:authority.to_account_info() })
        , amount)
}


pub fn mint(ctx:&mut Context<AddLiquidity>,x:u64,y:u64)->Result<(u128,u64)> {

   

    let (balance_x, balance_y) = if ctx.accounts.to_associated_account_for_x.amount == 0 && ctx.accounts.to_associated_account_for_y.amount == 0 {
        (x,y)
    }else{
        (ctx.accounts.to_associated_account_for_x.amount + x ,ctx.accounts.to_associated_account_for_y.amount + y)
    };


    // let (balance_x, balance_y) = (ctx.accounts.to_associated_account_for_x.amount,ctx.accounts.to_associated_account_for_y.amount);
    
    let amount_x = (balance_x as u128) - (ctx.accounts.token_pair_metadata_account.reserve_x as u128);
    let amount_y = (balance_y as u128) - (ctx.accounts.token_pair_metadata_account.reserve_y as u128);


    let fee = mint_fee(&ctx.accounts.token_pair_metadata_account,&ctx.accounts.sushilptoken);
    
    msg!("fees ++++++++++++++++++ {}",fee);

    if fee > 0 {
        mint_lp_fees_add(ctx, fee).unwrap();
    }
  

   

    let total_supply = ctx.accounts.sushilptoken.supply as u128;

        let liquidity = if total_supply == 0u128 {
            let sqrt = (amount_x * amount_y).sqrt();
            require!(sqrt > MINIMUM_LIQUIDITY, Errors::ErrorInsufficientLiquidityMinted);
            let l = sqrt - MINIMUM_LIQUIDITY;
            // permanently lock the first MINIMUM_LIQUIDITY tokens
            mint_lp_fees_add(ctx, MINIMUM_LIQUIDITY as u64).unwrap();
            l
        } else {
            let liquidity = (amount_x * total_supply / (ctx.accounts.token_pair_metadata_account.reserve_x as u128)).min(amount_y * total_supply / (ctx.accounts.token_pair_metadata_account.reserve_y as u128));
            require!(liquidity > 0u128, Errors::ErrorInsufficientLiquidityMinted);
            liquidity
            
        };

    
        let metadata =&mut ctx.accounts.token_pair_metadata_account.as_mut();

        metadata.reserve_x = ctx.accounts.to_associated_account_for_x.amount + x;
       
        metadata.reserve_y = ctx.accounts.to_associated_account_for_y.amount + y;
       

        let clock = Clock::get()?;

        metadata.block_timestamp_last = clock.unix_timestamp;
            
    
        metadata.k_last = ((metadata.reserve_x) as u128) * ((metadata.reserve_y)  as u128);

    
    Ok((liquidity,fee))

}


pub fn mint_lp_token(ctx:&Context<AddLiquidity>,liquidity:u128) -> Result<()> {
    let cpi_program = ctx.accounts.sushi_token_program.to_account_info();
    let cpi_accounts = MintTokenTo{
        authority: ctx.accounts.user.to_account_info(),
        sushilptoken:ctx.accounts.sushilptoken.to_account_info(),
        to: ctx.accounts.associated_account_for_sushilp.to_account_info(),
        vault: ctx.accounts.vault.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
        token_x:ctx.accounts.token_x.to_account_info(),
        token_y:ctx.accounts.token_y.to_account_info()
    };

    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

    lptoken::cpi::mint_token_to(cpi_context, liquidity as u64)
}

pub fn mint_lp_fees_add(ctx:&mut Context<AddLiquidity>,fees:u64) -> Result<()> {
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




pub fn mint_fee(metadata:&Account<'_,TokenPairMetadata>,sushilptoken:&Box<Account<'_,Mint>>)-> u64 {
    let mut fee = 0u64;
    
    if metadata.k_last != 0 {
        // # TODO: change this wrong calculation
        let root_k = (metadata.reserve_x as u128).checked_mul(metadata.reserve_y as u128).unwrap().sqrt();
        let root_k_last = (metadata.k_last).sqrt();

        if root_k > root_k_last {
            let numerator = (sushilptoken.supply as u128) * (root_k - root_k_last) * 8u128;
            let denominator = root_k_last * 17u128 + (root_k * 8u128);
            let liquidity = numerator / denominator;
            fee = liquidity as u64;
            // record fee amount in metadata, in case of fee_to with register.
           
        };
    };

    fee
}