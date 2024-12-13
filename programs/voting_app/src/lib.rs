use anchor_lang::prelude::*;

declare_id!("8WxMscTKPfCZo5735pFz3iRWQFQRQ52VnrZ4uDeCsw1f");

const GLOBAL_ACCOUNT_SEED: &[u8] = b"global_account";
const POLL_SEED: &[u8] = b"poll";
const VOTER_SEED: &[u8] = b"voter";

#[program]
pub mod voting_app {
    use super::*;

    // Initialize the global account
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let global_account = &mut ctx.accounts.global_account;
        global_account.polls_counter = 1;
        Ok(())
    }

    // Create a new poll
    pub fn create_poll(ctx: Context<CreatePoll>, question: String) -> Result<()> {
        let global_account = &mut ctx.accounts.global_account;
        let poll_account = &mut ctx.accounts.poll_account;
        let user = &ctx.accounts.user;
    
        // Validate the question length
        if question.len() > 200 {
            return Err(ErrorCode::QuestionTooLong.into());
        }
    
        // Initialize poll account fields
        poll_account.number = global_account.polls_counter;
        poll_account.question = question;
        poll_account.author = user.key();
        poll_account.yes = 0;
        poll_account.no = 0;
          
        // Increment the global poll counter
        global_account.polls_counter += 1;

        // Emit de Poll created event
        emit!(PollCreated {
            poll_pda: ctx.accounts.poll_account.key(),
        });
    
        Ok(())
    }

    // Vote on a poll
    pub fn vote(ctx: Context<Vote>, vote: bool) -> Result<()> {
        let poll_account = &mut ctx.accounts.poll_account;
        let voter_account = &mut ctx.accounts.voter_account;

        // If the user already voted in this poll, the program
        // will thrown an "account Address already in use" error

        // Update the poll results
        if vote {
            poll_account.yes += 1;
        } else {
            poll_account.no += 1;
        }

        // Update voter account fields
        voter_account.poll = poll_account.key();
        voter_account.voter = ctx.accounts.user.key();
        voter_account.vote = vote;
        voter_account.voted = true;

        Ok(())
    }
}

// Context for initializing the global account
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = user, 
        space = 8 + 8 + 1, 
        seeds = [GLOBAL_ACCOUNT_SEED], 
        bump
    )]
    pub global_account: Account<'info, GlobalAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Context for creating a poll
#[derive(Accounts)]
pub struct CreatePoll<'info> {
    #[account(mut)]
    pub global_account: Account<'info, GlobalAccount>,
    #[account(
        init, 
        payer = user, 
        space = 8 + 8 + (4 + 200) + 32 + 8 + 8 + 1, 
        seeds = [POLL_SEED, &global_account.polls_counter.to_le_bytes()], 
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Context for voting on a poll
#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub poll_account: Account<'info, PollAccount>,
    #[account(
        init, 
        payer = user, 
        space = 8 + 32 + 32 + 1 + 1, 
        seeds = [VOTER_SEED, poll_account.key().as_ref(), user.key().as_ref()], 
        bump
    )]
    pub voter_account: Account<'info, VoterAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Poll account structure
#[account]
pub struct PollAccount {
    pub number: u64,      // Unique poll number
    pub question: String, // Poll question
    pub author: Pubkey,   // Author of the poll
    pub yes: u64,         // Count of "yes" votes
    pub no: u64,          // Count of "no" votes
}

// Voter account structure
#[account]
pub struct VoterAccount {
    pub poll: Pubkey,  // Associated poll PDA
    pub voter: Pubkey, // Voter's public key
    pub vote: bool,    // `true` for "yes", `false` for "no"
    pub voted: bool,   // Whether the voter has already voted
}

// Global account structure
#[account]
pub struct GlobalAccount {
    pub polls_counter: u64, // Counter for polls
}

#[event]
pub struct PollCreated {
    pub poll_pda: Pubkey,
}

// Error codes
#[error_code]
pub enum ErrorCode {
    #[msg("User already voted on this poll.")]
    AlreadyVoted,
    #[msg("The question exceeds the maximum length of 200 characters.")]
    QuestionTooLong,
}
