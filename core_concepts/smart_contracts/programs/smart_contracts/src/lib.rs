#![allow(unexpected_cfgs)]  //silences conflict related to nightly rustc
use anchor_lang::prelude::*;

declare_id!("wLdqJZg7heBecsP3vT57smP3yfVEa8mfyttaEagCeg5");


#[program]
pub mod smart_contracts {
    use super::*;

    pub fn init_pda(ctx: Context<InitPDA>) -> Result<()> {
        ctx.accounts.meta.bump_seed = ctx.bumps.meta;
        Ok(())
    }

    pub fn update_pda(ctx: Context<UpdatePDA>) -> Result<()> {
        ctx.accounts.meta.counter += 1;
        msg!("counter state: {}", ctx.accounts.meta.counter);
        Ok(())
    }
}


#[derive(Accounts)]
pub struct InitPDA<'info> {
    #[account(
        init,
        payer = signer,
        seeds = [b"meta", signer.key().as_ref()],
        bump,
        space = 8 + 16

    )]
    pub meta: Account<'info, PDAmeta>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct UpdatePDA<'info> {
    #[account(
        mut,
        seeds = [b"meta", signer.key().as_ref()],
        bump = meta.bump_seed
    )]
    pub meta: Account<'info, PDAmeta>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct PDAmeta {
    pub counter: u64,
    pub bump_seed: u8,
    _padding: [u8; 7]
}