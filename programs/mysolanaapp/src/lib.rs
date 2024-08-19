use anchor_lang::prelude::*;

declare_id!("5DbyizbAvjUrzrjbbww6dGcGocW2iJiM4xxsSZSSLTc5");

#[program]
pub mod mysolanaapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
