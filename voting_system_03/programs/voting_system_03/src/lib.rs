use anchor_lang::prelude::*;

declare_id!("E9haTaKCWdk1qbP88xsA6CL6jgi4izoaUXidYPn65B2y");

#[program]
pub mod voting_system {
    use super::*;

    // 1. Initialize a new proposal with explicit duration (in seconds)
    pub fn initialize_proposal(ctx: Context<InitializeProposal>, name: String, duration_seconds: i64) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let current_time = Clock::get()?.unix_timestamp;

        proposal.name = name;
        proposal.yes_votes = 0;
        proposal.no_votes = 0;
        proposal.start_time = current_time;
        proposal.end_time = current_time + duration_seconds;
        Ok(())
    }

    // 2. Cast a vote ('yes' or 'no')
    pub fn cast_vote(ctx: Context<CastVote>, vote_type: String) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let current_time = Clock::get()?.unix_timestamp;

        // Deadline Check
        macro_rules! check_deadline {
            () => {
                require!(current_time <= proposal.end_time, VotingError::VotingEnded);
            };
        }
        check_deadline!();

        // Count vote
        if vote_type == "yes" {
            proposal.yes_votes += 1;
        } else if vote_type == "no" {
            proposal.no_votes += 1;
        } else {
            return err!(VotingError::InvalidVoteType);
        }

        // Voter PDA initialization handles tracking automatically
        let voter_record = &mut ctx.accounts.voter_record;
        voter_record.has_voted = true;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String, duration_seconds: i64)]
pub struct InitializeProposal<'info> {
    #[account(
        init, 
        payer = signer, 
        space = 8 + 4 + name.len() + 8 + 8 + 8 + 8, // 8 byte discriminator + fields
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
    
    // Creating this PDA forces a runtime error if the wallet tries to vote twice
    #[account(
        init,
        payer = signer,
        space = 8 + 1, // Discriminator + bool
        seeds = [b"voter", proposal.key().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub voter_record: Account<'info, VoterRecord>,
    
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Proposal {
    pub name: String, 
    pub yes_votes: u64,
    pub no_votes: u64,
    pub start_time: i64,
    pub end_time: i64,
}

#[account]
pub struct VoterRecord {
    pub has_voted: bool,
}

#[error_code]
pub enum VotingError {
    #[msg("This voting window has already ended.")]
    VotingEnded,
    #[msg("Invalid vote option. Choose 'yes' or 'no'.")]
    InvalidVoteType,
}
