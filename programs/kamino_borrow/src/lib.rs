use anchor_lang::prelude::*;

declare_id!("3DSox9PrsL3PTJDuNq5p4kCr9TkfVFN8dAhMffJ6DiBr");

#[program]
pub mod kamino_borrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
