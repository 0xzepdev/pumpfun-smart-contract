use anchor_lang::{prelude::*, solana_program::program::invoke_signed};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::state::{LiquidityPool, LiquidityPoolAccount, LiquidityProvider};
use raydium_contract_instructions::amm_instruction;

pub fn remove_liquidity(
    ctx: Context<RemoveLiquidity>,
    nonce: u8,
    init_pc_amount: u64,
) -> Result<()> {
    
    // you can add your cpi in this part

    Ok(())
}

#[derive(Accounts)]
pub struct RemoveLiquidity<'info> {
    #[account(
        mut,
        seeds = [LiquidityPool::POOL_SEED_PREFIX.as_bytes(), coin_mint.key().as_ref()],
        bump = pool.bump
    )]
    pub pool: Box<Account<'info, LiquidityPool>>,

 
    #[account(
        mut,
        seeds = [b"global"],
        bump,
    )]
    pub global_account: AccountInfo<'info>,
    
   
    
}
