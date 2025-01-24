use anchor_lang::prelude::*;

declare_id!("H5CtT4c2mmkcwSAWc4nWTFXjD9bQGgcag57WEwrucLwQ");

#[program]
pub mod trading_agent_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
