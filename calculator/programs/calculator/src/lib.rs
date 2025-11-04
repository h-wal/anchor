use anchor_lang::prelude::*;

declare_id!("DQnP1SXYu4tNfeP4cdReiS1jWA9qNTgsX3itBmosSNUc");

#[program]
pub mod calculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn double()
}

#[derive(Accounts)]
pub struct Initialize {}
