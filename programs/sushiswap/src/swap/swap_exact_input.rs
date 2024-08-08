use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, transfer, Transfer};

use crate::{customerror::error::Errors, Swap, swaputils::{u256, swap_utils}, TokenPairMetadata};

const PRECISION: u64 = 10000;

pub fn swap_exact_input(ctx:Context<Swap>,x_in: u64,y_min_out: u64,swap_x_to_y:bool)->Result<()>{

    
    if swap_x_to_y {
        deposit_to_pool(&ctx.accounts.token_program, &ctx.accounts.to_associated_account_for_x, &ctx.accounts.pool_associated_account_for_x, &ctx.accounts.user, x_in)?;
    }else {
        deposit_to_pool(&ctx.accounts.token_program, &ctx.accounts.to_associated_account_for_y, &ctx.accounts.pool_associated_account_for_y, &ctx.accounts.user, x_in)?;
    }

    let (rin, rout) = if swap_x_to_y {
        (ctx.accounts.token_pair_metadata_account.reserve_x,ctx.accounts.token_pair_metadata_account.reserve_y)
    } else{
        (ctx.accounts.token_pair_metadata_account.reserve_y,ctx.accounts.token_pair_metadata_account.reserve_x)
    };
        
    
    let amount_out = swap_utils::get_amount_out(x_in, rin, rout).unwrap();
    
    let (coins_x_out,coins_y_out) = swap(&ctx,0, amount_out,swap_x_to_y)?;

    msg!("++++++++++++++++ {}",amount_out);



    require!(coins_x_out == 0, Errors::ErrorInsufficientOutputAmount);

    require!(coins_y_out >= y_min_out, Errors::ErrorOutputLessThanMin);

    if swap_x_to_y {
        transfer_from_pool(&ctx.accounts.token_program,&ctx.accounts.pool_associated_account_for_x, &ctx.accounts.to_associated_account_for_x, &ctx.accounts.token_pair_metadata_account,coins_x_out)?;
        transfer_from_pool(&ctx.accounts.token_program,&ctx.accounts.pool_associated_account_for_y, &ctx.accounts.to_associated_account_for_y, &ctx.accounts.token_pair_metadata_account,coins_y_out)?;
    }else{
        transfer_from_pool(&ctx.accounts.token_program,&ctx.accounts.pool_associated_account_for_y, &ctx.accounts.to_associated_account_for_y, &ctx.accounts.token_pair_metadata_account,coins_x_out)?;
        transfer_from_pool(&ctx.accounts.token_program,&ctx.accounts.pool_associated_account_for_x, &ctx.accounts.to_associated_account_for_x, &ctx.accounts.token_pair_metadata_account,coins_y_out)?;
    };


    let metadata2 = &mut ctx.accounts.token_pair_metadata_account.as_mut();

    if swap_x_to_y { 
        metadata2.reserve_x += x_in;
        metadata2.reserve_y -= coins_y_out;
    }else{
        metadata2.reserve_y += x_in;
        metadata2.reserve_x -= coins_y_out;
    }

   

    let clock = Clock::get()?;

    metadata2.block_timestamp_last = clock.unix_timestamp;

    Ok(())

}




fn swap(
    ctx:&Context<Swap>,
    amount_x_out: u64,
    amount_y_out: u64,
    swap_x_to_y:bool
)->Result<(u64,u64)>{
    
    require!(amount_x_out > 0 || amount_y_out > 0, Errors::ErrorInsufficientOutputAmount);

    let metadata = &ctx.accounts.token_pair_metadata_account;
    
    let (reserve_x,reserve_y) = if swap_x_to_y{
        (metadata.reserve_x,metadata.reserve_y)
    }else{
        (metadata.reserve_y,metadata.reserve_x)
    };

    require!(amount_x_out < reserve_x && amount_y_out < reserve_y, Errors::ErrorInsufficientLiquidity);

    // if swap_x_to_y {
    //     require!(amount_x_out < metadata.reserve_x && amount_y_out < metadata.reserve_y, Errors::ErrorInsufficientLiquidity);
    // }else {
    //     require!(amount_x_out < metadata.reserve_y && amount_y_out < metadata.reserve_x, Errors::ErrorInsufficientLiquidity);
    // }



    // let coins_x_out = coin::zero<X>();
    // let coins_y_out = coin::zero<Y>();
    // if (amount_x_out > 0) coin::merge(&mut coins_x_out, extract_x(amount_x_out, metadata));
    // if (amount_y_out > 0) coin::merge(&mut coins_y_out, extract_y(amount_y_out, metadata));

    let (balance_x, balance_y) = if swap_x_to_y {
        (ctx.accounts.pool_associated_account_for_x.amount,ctx.accounts.pool_associated_account_for_y.amount) 
    }else{
        (ctx.accounts.pool_associated_account_for_y.amount,ctx.accounts.pool_associated_account_for_x.amount)
    };
    

    let amount_x_in = if balance_x > reserve_x - amount_x_out {
        balance_x - (reserve_x - amount_x_out)
    } else { 0 };

    let amount_y_in = if balance_y > reserve_y - amount_y_out {
        balance_y - (reserve_y - amount_y_out)
    } else { 0 };

    require!(amount_x_in > 0 || amount_y_in > 0, Errors::ErrorInsufficientInputAmount);

    let prec = PRECISION as u128;
    let balance_x_adjusted = (balance_x as u128) * prec - (amount_x_in as u128) * 25u128;
    let balance_y_adjusted = (balance_y as u128) * prec - (amount_y_in as u128) * 25u128;
    let reserve_x_adjusted = (reserve_x as u128) * prec;
    let reserve_y_adjusted = (reserve_y as u128) * prec;

    // No need to use u256 when balance_x_adjusted * balance_y_adjusted and reserve_x_adjusted * reserve_y_adjusted are less than MAX_U128.
    let compare_result = if balance_x_adjusted > 0 && reserve_x_adjusted > 0 && u128::MAX / balance_x_adjusted > balance_y_adjusted && u128::MAX / reserve_x_adjusted > reserve_y_adjusted {
        msg!("amount out +++++++++++++++ {}",amount_y_out);
        msg!("if true +++++++++++++++ {}",balance_x_adjusted * balance_y_adjusted);
        msg!("if true +++++++++++++++ {}",reserve_x_adjusted * reserve_y_adjusted);
        balance_x_adjusted * balance_y_adjusted >= reserve_x_adjusted * reserve_y_adjusted
    }else{
        
        let p = u256::mul_u128(balance_x_adjusted, balance_y_adjusted);
        let k = u256::mul_u128(reserve_x_adjusted, reserve_y_adjusted);
        msg!("if false +++++++++++++++ {}",u256::ge(&p, &k));
        u256::ge(&p, &k)
    };
    // require!(compare_result, Errors::ErrorK);

    Ok((amount_x_out, amount_y_out))
}


pub fn deposit_to_pool<'info>(token_program:&Program<'info,Token>,from:&Account<'info,TokenAccount>,to:&Account<'info,TokenAccount>,authority:&Signer<'info>,amount:u64,)->Result<()>{

    require!(from.amount > amount,Errors::ErrorInsufficientAmount);
    transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer { from:from.to_account_info() , to:to.to_account_info(), authority:authority.to_account_info() })
        , amount)
}


pub fn transfer_from_pool<'info>(token_program:&Program<'info,Token>,from:&Account<'info,TokenAccount>,to:&Account<'info,TokenAccount>,authority:&Box<Account<'info,TokenPairMetadata>>,amount:u64,)->Result<()>{

    require!(from.amount > amount,Errors::ErrorInsufficientAmount);
    transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info()
            , Transfer { from:from.to_account_info() , to:to.to_account_info(), authority:authority.to_account_info() },
            &[&[b"TokenPairMetadataTS",authority.token_x.key().as_ref(),authority.token_y.key().as_ref(),&[authority.own_bump]]]  
        )
        , amount)
}