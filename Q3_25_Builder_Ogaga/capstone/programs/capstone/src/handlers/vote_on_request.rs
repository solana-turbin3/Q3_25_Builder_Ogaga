use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::*;

pub fn vote_on_request(
    context: Context<VoteOnRequestAccountConstraints>,
    _invite_code: String,
    vote: bool, // true = yes, false = no
) -> Result<()> {
    let circle = &context.accounts.circle_account;
    let request = &mut context.accounts.funding_request;
    let voter_key = context.accounts.voter.key();
    
    // Check if voter is in the circle
    let is_member = voter_key == circle.member1 || 
                   voter_key == circle.member2 || 
                   voter_key == circle.member3;
    
    require!(is_member, CustomError::NotAMember);

    // Verify request is still active
    require!(
        request.status == RequestStatus::Active,
        CustomError::RequestNotActive
    );

    // Check if voter has already voted
    let already_voted = voter_key == request.voter1 || 
                       voter_key == request.voter2 || 
                       voter_key == request.voter3;
    
    require!(!already_voted, CustomError::AlreadyVoted);

    // Record the vote in next available slot
    if request.voter_count < 3 {
        match request.voter_count {
            0 => request.voter1 = voter_key,
            1 => request.voter2 = voter_key,
            2 => request.voter3 = voter_key,
            _ => return Err(CustomError::AlreadyVoted.into()),
        }
        request.voter_count += 1;
    }
    
    if vote {
        request.votes_for += 1;
    } else {
        request.votes_against += 1;
    }

    // Check if we have majority vote (more than half of members)
    let total_members = circle.member_count as u32;
    let majority_threshold = total_members / 2 + 1;

    if request.votes_for >= majority_threshold {
        request.status = RequestStatus::Approved;
        msg!("Request approved! {} votes for out of {} members", request.votes_for, total_members);
    } else if request.votes_against >= majority_threshold {
        request.status = RequestStatus::Rejected;
        msg!("Request rejected! {} votes against out of {} members", request.votes_against, total_members);
    }

    msg!(
        "Vote recorded: {} voted {} on request for {} USDC",
        context.accounts.voter.key(),
        if vote { "YES" } else { "NO" },
        request.amount
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(invite_code: String, vote: bool)]
pub struct VoteOnRequestAccountConstraints<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,

    #[account(
        seeds = [b"circle", invite_code.as_bytes()],

        bump
    )]
    pub circle_account: Account<'info, CircleAccount>,

    #[account(
        mut,

        constraint = funding_request.circle == circle_account.key()
    )]
    pub funding_request: Account<'info, FundingRequest>,
}

