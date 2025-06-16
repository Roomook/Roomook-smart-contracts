use anchor_lang::prelude::*;

declare_id!("RMKRmKRmKRmKRmKRmKRmKRmKRmKRmKRmKRmKRmKRmK");

#[program]
pub mod rmk_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _bump: u8) -> Result<()> {
        msg!("Roomook RMK Utility Token Program initialized");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
}
