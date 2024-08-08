use anchor_lang::error_code;


#[error_code]
pub enum Errors {
    #[msg("Amount must be grater then 0")]
    ErrorInsufficientAmount,
    #[msg("Insufficient Liquidity")]
    ErrorInsufficientLiquidity,
    #[msg("Invalid Amount")]
    ErrorInvalidAmount,
    #[msg("Wrong account provided for To")]
    ErrorInvalidToAccount,
    #[msg("Associated account for From must be owned by signer")]
    ErrorInvalidFromAccount,
    #[msg("Insufficient Liquidity Minted")]
    ErrorInsufficientLiquidityMinted,
    #[msg("Amount less the you required")]
    ErrorInsufficientXAmount,
    #[msg("Amount less the you required")]
    ErrorInsufficientYAmount,
    #[msg("Don't have enough amount for add liquidity")]
    ErrorInsufficientLiquidityAmount,

    #[msg("Don't have enough liquidity")]
    ErrorInsufficientLiquidityAmountBurned,
    #[msg("Input amount must be grater then 0")]
    ErrorInsufficientInputAmount,
    #[msg("Output amount must be grater then 0")]
    ErrorInsufficientOutputAmount,
    #[msg("ErrorK Failed to compar")]
    ErrorK,
    #[msg("Output amount less than you require")]
    ErrorOutputLessThanMin



}