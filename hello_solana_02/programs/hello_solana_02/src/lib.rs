use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111"); 

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, Solana Blockchain!"); 
        
        Ok(())
    }
}

#[derive(Accounts)]
pub mod Initialize {}
