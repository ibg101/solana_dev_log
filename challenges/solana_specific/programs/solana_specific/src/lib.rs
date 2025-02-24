use anchor_lang::prelude::*;

declare_id!("4VsHiSmv5rpYYVDRmKQcAii4eZZKjeHfXgWNDL7TNX8T");

#[program]
pub mod solana_specific {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
