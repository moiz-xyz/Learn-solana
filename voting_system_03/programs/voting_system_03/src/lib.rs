use anchor_lang::prelude::*;

declare_id!("E9haTaKCWdk1qbP88xsA6CL6jgi4izoaUXidYPn65B2y");

#[program]
pub mod voting_system {
    use super::*;

    // 1. Initialize a new proposal
    pub mod_initialize_proposal(ctx: Context<InitializeProposal>, name: String) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.name = name;
        proposal.yes_votes = 0;
        proposal.no_votes = 0;
        Ok(())
    }

    // 2. Cast a vote ('yes' or 'no')
    pub fn cast_vote(ctx: Context<CastVote>, vote_type: String) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        
        if vote_type == "yes" {
            proposal.yes_votes += 1;
        } else if vote_type == "no" {
            proposal.no_votes += 1;
        }
        
        Ok(())
    }
}

// Data validation structures for instructions

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializeProposal<'info> {
    // We derive the proposal account address using seeds so it's unique
    #[account(
        init, 
        payer = signer, 
        space = 8 + 32 + 8 + 8 + 4 + name.len(), // 8 byte discriminator + fields
        seeds = [b"proposal", name.as_bytes()], 
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    pub signer: Signer<'info>,
}

// Define the structure of the data account
#[account]
pub struct Proposal {
    pub name: String,    // Max length defined during initialization
    pub yes_votes: u64,
    pub no_votes: u64,
}