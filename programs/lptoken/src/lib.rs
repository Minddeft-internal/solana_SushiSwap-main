use anchor_lang::prelude::*;
use anchor_spl::{token::{Transfer,Mint,Token,TokenAccount,MintTo,mint_to,transfer,FreezeAccount,freeze_account,ThawAccount,thaw_account},associated_token::AssociatedToken};
use mpl_token_metadata::state::TokenMetadataAccount;



declare_id!("Dbt4xEW1CJyAxuegXs9bo5hs9qtXsE9eobJEU1bPNZsp");
// declare_id!("6xujtrFHJhcDkxs8xabt67495SbPVGVLkAX9SjvsxKHs");

#[program]
pub mod lptoken {

  
    use anchor_lang::solana_program::program::invoke;
    use anchor_spl::token::{Burn, burn};
    use mpl_token_metadata::{ID as MetaDataID,instruction::create_metadata_accounts_v3, state::Creator};
   

    use super::*;

    pub fn create_new_token(ctx: Context<InitializeToken>,token_name:String,b1:u8,b2:u8) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        vault.authority = ctx.accounts.owner.key();
        vault.sushi_lptoken_account = ctx.accounts.sushilptoken.key();
        vault.sushi_lptoken_bump = b1;
        vault.vault_bump = b2;

        // let (metadata, _) = Metadata::find_pda(&ctx.accounts.sushilptoken.key());
        // let (master_edition, _) = MasterEdition::find_pda(&ctx.accounts.sushilptoken.key());

        // when we create a non-fungible metadata

        let account_info = vec![
            ctx.accounts.metadata_account.to_account_info(),
            ctx.accounts.sushilptoken.to_account_info(),
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.owner.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        
        
    
        let ix = create_metadata_accounts_v3(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata_account.key(),
            ctx.accounts.sushilptoken.key(),
            ctx.accounts.owner.key(),
            ctx.accounts.owner.key(),
            ctx.accounts.owner.key(),
            "Solana Ether".to_string(),
            "SETH".to_string(),
            "https://arweave.net/KZDlKw8aCG4kfZtj9Qmh8tmYpH4Q287P_jmUtkl2s-k".to_string(),
            Some(vec![Creator {
                address: ctx.accounts.owner.key(),
                verified: false,
                share: 100,
            }]),
            1,
            true,
            true,
            None,
            None,
            None,
        );

        invoke(&ix,
            account_info.as_slice()
        )?;
        

        Ok(())
    }

    pub fn initialize(ctx: Context<InitializeMintAccount>) -> Result<()> {

        let vault = &mut ctx.accounts.vault;

        vault.authority = ctx.accounts.owner.key();
        vault.sushi_lptoken_account = ctx.accounts.sushilptoken.key();
        vault.sushi_lptoken_bump = *ctx.bumps.get("sushilptoken").unwrap();
        vault.vault_bump = *ctx.bumps.get("vault").unwrap();

        

        Ok(())
    }

    pub fn mint_token_to(ctx: Context<MintTokenTo>,amount:u64)->Result<()>{
                
        mint_to(
            CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo{
                mint:ctx.accounts.sushilptoken.to_account_info(),
                to:ctx.accounts.to.to_account_info(),
                authority:ctx.accounts.sushilptoken.to_account_info()
            },
            &[&[b"sushilptokenTS",ctx.accounts.token_x.key().as_ref(),ctx.accounts.token_y.key().as_ref(),&[ctx.accounts.vault.sushi_lptoken_bump]]]
        ), amount)?;

        Ok(())
    }

    pub fn mint_coin_to(ctx: Context<MintCoinTo>,token_name:String,amount:u64)->Result<()>{
                
        mint_to(
            CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo{
                mint:ctx.accounts.sushilptoken.to_account_info(),
                to:ctx.accounts.to.to_account_info(),
                authority:ctx.accounts.sushilptoken.to_account_info()
            },
            &[&[b"token",token_name.as_bytes(),&[ctx.accounts.vault.sushi_lptoken_bump]]]
        ), amount)?;

        Ok(())
    }

    pub fn register_lptoken_account(_ctx: Context<RegisterSushiLP>)->Result<()>{
    
        Ok(())
    }

    pub fn transfer_token(ctx: Context<TransferToken>,amount:u64) -> Result<()> {
        
        transfer(CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer{
                from:ctx.accounts.from_ata.to_account_info(),
                to:ctx.accounts.to_ata.to_account_info(),
                authority:ctx.accounts.vault.to_account_info()
            },&[&[b"vault",&[ctx.accounts.vault.vault_bump]]]), amount)?;
            
        Ok(())
    }

    pub fn freeze_user_account(ctx: Context<FreezeUserAccount>) -> Result<()> {
        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            FreezeAccount{
                mint:ctx.accounts.sushilptoken.to_account_info(),
                account:ctx.accounts.account_to_be_freeze.to_account_info(),
                authority:ctx.accounts.authority.to_account_info()
            }
            
        );

        freeze_account(cpi_context)?;

        Ok(())
    }
    
    pub fn unfreeze_user_account(ctx: Context<UnfreezeUserAccount>) -> Result<()> {

        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            ThawAccount{
                mint:ctx.accounts.sushilptoken.to_account_info(),
                account:ctx.accounts.account_to_be_unfreeze.to_account_info(),
                authority:ctx.accounts.authority.to_account_info()
            }
        );
        
        thaw_account(cpi_context)?;

        Ok(())
    }

    pub fn burn_token(ctx: Context<BurnUserToken>,amount:u64)-> Result<()>{
        
        burn(
            CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn{
                mint:ctx.accounts.sushilptoken.to_account_info(),
                from:ctx.accounts.from_ata.to_account_info(),
                authority:ctx.accounts.authority.to_account_info()
            },
        ), amount)?;

        Ok(())

    }

    
}

#[derive(Accounts)]
pub struct InitializeMintAccount<'info> {
    #[account(mut)]
    pub owner : Signer<'info>,
    #[account(
        init,
        seeds = [b"sushilptokenTS".as_ref(),token_x.key().as_ref(),token_y.key().as_ref()],
        bump,
        payer = owner,
        mint::decimals = 8,
        mint::authority = sushilptoken,
        mint::freeze_authority = sushilptoken,
        )]
    pub sushilptoken: Account<'info, Mint>,
    #[account(
        init,
        payer = owner,
        associated_token::mint = sushilptoken,
        associated_token::authority = sushilptoken,
    )]
    pub associated_account_for_fees: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = owner,
        associated_token::mint = sushilptoken,
        associated_token::authority = owner,
    )]
    pub associated_account: Account<'info, TokenAccount>,
    pub token_program : Program<'info,Token>,
    pub system_program: Program<'info, System>,
    pub rent : Sysvar<'info,Rent>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_x : Account<'info,Mint>,
    pub token_y : Account<'info,Mint>,
    #[account(
        init,
        seeds = [b"vaultTS".as_ref(),token_x.key().as_ref(),token_y.key().as_ref()],
        bump,
        payer = owner,
        space = 8 + Vault::SPACE
        )]
    pub vault: Account<'info,Vault>,

}


#[derive(Accounts)]
#[instruction(token_name:String)]
pub struct InitializeToken<'info> {
    #[account(mut)]
    pub owner : Signer<'info>,
    #[account(
        init,
        seeds = [b"token".as_ref(),token_name.as_bytes()],  
        bump,
        payer = owner,
        mint::decimals = 8,
        mint::authority = owner,
        mint::freeze_authority = owner,
        )]
    pub sushilptoken: Account<'info, Mint>,
    #[account(
        init,
        payer = owner,
        associated_token::mint = sushilptoken,
        associated_token::authority = owner,
    )]
    pub associated_account: Account<'info, TokenAccount>,
    #[account(mut)]
    /// CHECK:for test
    pub metadata_account: UncheckedAccount<'info>,
    /// CHECK:for test
    pub token_metadata_program :UncheckedAccount<'info>,
    pub token_program : Program<'info,Token>,
    pub system_program: Program<'info, System>,
    pub rent : Sysvar<'info,Rent>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(
        init,
        seeds = [b"vault".as_ref(),token_name.as_bytes()],
        bump,
        payer = owner,
        space = 8 + Vault::SPACE
        )]
    pub vault: Account<'info,Vault>,

}




#[derive(Accounts)]
#[instruction(amount:u64)]
pub struct MintTokenTo<'info> {
    #[account(mut)]
    pub authority : Signer<'info>,
    #[account(mut,seeds=[b"sushilptokenTS".as_ref(),token_x.key().as_ref(),token_y.key().as_ref()],bump=vault.sushi_lptoken_bump )]
    pub sushilptoken: Account<'info, Mint>,
    #[account(mut)]
    pub to : Account<'info,TokenAccount>,
    #[account(seeds=[b"vaultTS".as_ref(),token_x.key().as_ref(),token_y.key().as_ref()],bump=vault.vault_bump)]
    pub vault : Account<'info,Vault>,
    pub token_program: Program<'info,Token>,
    pub system_program: Program<'info, System>,
    pub rent : Sysvar<'info,Rent>,
    pub token_x : Account<'info,Mint>,
    pub token_y : Account<'info,Mint>,
}


#[derive(Accounts)]
#[instruction(token_name:String)]
pub struct MintCoinTo<'info> {
    #[account(mut)]
    pub authority : Signer<'info>,
    #[account(mut,seeds=[b"token".as_ref(),token_name.as_bytes()],bump=vault.sushi_lptoken_bump )]
    pub sushilptoken: Account<'info, Mint>,
    #[account(mut)]
    pub to : Account<'info,TokenAccount>,
    #[account(seeds=[b"vault".as_ref(),token_name.as_bytes()],bump=vault.vault_bump)]
    pub vault : Account<'info,Vault>,
    pub token_program: Program<'info,Token>,
    pub system_program: Program<'info, System>,
    pub rent : Sysvar<'info,Rent>
}


#[derive(Accounts)]
pub struct TransferToken<'info> {
    #[account(mut)]
    pub from:Signer<'info>,
    #[account(mut)]
    pub from_ata:Account<'info,TokenAccount>,
    #[account(mut)]
    pub to_ata:Account<'info,TokenAccount>,
    #[account(mut,seeds=[b"vault".as_ref()],bump=vault.vault_bump)]
    pub vault : Account<'info,Vault>,
    pub token_program: Program<'info,Token>,
}




#[derive(Accounts)]
#[instruction(token_x:String,token_y:String)]
pub struct RegisterSushiLP<'info> {
    #[account(
        init,
        payer = user,
        associated_token::mint = sushilptoken,
        associated_token::authority = vault,
    )]
    pub sushi_lptoken_account: Account<'info, TokenAccount>,
    
    #[account(mut,seeds=[b"sushilptoken".as_ref(),token_x.as_bytes(),token_y.as_bytes()],bump=vault.sushi_lptoken_bump )]
    pub sushilptoken: Account<'info, Mint>,
    #[account(mut,seeds=[b"vault".as_ref(),token_x.as_bytes(),token_y.as_bytes()],bump=vault.vault_bump)]
    pub vault : Account<'info,Vault>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent : Sysvar<'info,Rent>
}



#[derive(Accounts)]
pub struct FreezeUserAccount<'info> {
    
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut,seeds=[b"sushilptoken".as_ref()],bump=vault.sushi_lptoken_bump )]
    pub sushilptoken: Account<'info, Mint>,

    #[account(seeds=[b"vault".as_ref()],bump=vault.vault_bump,has_one=authority)]
    pub vault : Account<'info,Vault>,

    #[account(mut)]
    pub account_to_be_freeze : Account<'info,TokenAccount>,

    pub token_program : Program<'info,Token>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct UnfreezeUserAccount<'info> {
    
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut,seeds=[b"sushilptoken".as_ref()],bump=vault.sushi_lptoken_bump )]
    pub sushilptoken: Account<'info, Mint>,

    #[account(seeds=[b"vault".as_ref()],bump=vault.vault_bump,has_one=authority)]
    pub vault : Account<'info,Vault>,

    #[account(mut)]
    pub account_to_be_unfreeze : Account<'info,TokenAccount>,

    pub token_program : Program<'info,Token>,
    pub system_program: Program<'info, System>
}


#[derive(Accounts)]
#[instruction(token_x:String,token_y:String)]
pub struct BurnUserToken<'info> {
    
    #[account(mut)]
    pub authority : Signer<'info>,
    #[account(mut,seeds=[b"sushilptokenTS".as_ref(),token_x.key().as_ref(),token_y.key().as_ref()],bump=vault.sushi_lptoken_bump )]
    pub sushilptoken: Account<'info, Mint>,
    #[account(mut)]
    pub from_ata : Account<'info,TokenAccount>,
    #[account(seeds=[b"vaultTS".as_ref(),token_x.key().as_ref(),token_y.key().as_ref()],bump=vault.vault_bump)]
    pub vault : Account<'info,Vault>,
    pub token_program: Program<'info,Token>,
    pub system_program: Program<'info, System>,
    pub rent : Sysvar<'info,Rent>,
    pub token_x : Account<'info,Mint>,
    pub token_y : Account<'info,Mint>,
}







#[account]
pub struct Vault {
    sushi_lptoken_bump: u8,
    vault_bump:u8,
    authority:Pubkey,
    sushi_lptoken_account:Pubkey
}

impl Vault {
    pub const SPACE:usize = 1 + 1 + 32 + 32;
}

