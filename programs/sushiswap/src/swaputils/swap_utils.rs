
use anchor_lang::*;

use crate::customerror::error::Errors;


pub fn quote(amount_x: u64, reserve_x: u64, reserve_y: u64) -> Result<u64> {
    
    require!(amount_x > 0,Errors::ErrorInsufficientAmount);
    require!(reserve_x > 0 && reserve_y > 0, Errors::ErrorInsufficientLiquidity);

    Ok(((amount_x as u128) * (reserve_y as u128) / (reserve_x as u128)) as u64)
   
}


pub fn get_amount_out(
    amount_in: u64,
    reserve_in: u64,
    reserve_out: u64
)-> Result<u64> {

    require!(amount_in > 0, Errors::ErrorInsufficientInputAmount);
    require!(reserve_in > 0 && reserve_out > 0, Errors::ErrorInsufficientLiquidity);

    let amount_in_with_fee = (amount_in as u128) * 9975u128;
    let numerator = amount_in_with_fee * (reserve_out as u128);
    let denominator = (reserve_in as u128) * 10000u128 + amount_in_with_fee;
    

    Ok((numerator / denominator) as u64)
}