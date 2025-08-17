use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::*;

pub fn create_request(
    context: Context<CreateRequestAccountConstraints>,
    _invite_code: String,
    amount: u64,
    description: String,
) -> Result<()> {
    let circle = &context.accounts.circle_account;
    let requester_key = context.accounts.requester.key();
    
    // Check if requester is in the circle
    let is_member = requester_key == circle.member1 || 
                   requester_key == circle.member2 || 
                   requester_key == circle.member3;
    
    require!(is_member, CustomError::NotAMember);

    let request = &mut context.accounts.funding_request;
    request.requester = context.accounts.requester.key();
    request.circle = context.accounts.circle_account.key();
    request.amount = amount;
    request.description = description;
    request.votes_for = 0;
    request.votes_against = 0;
    request.voter_count = 0;
    request.voter1 = Pubkey::default();
    request.voter2 = Pubkey::default();
    request.voter3 = Pubkey::default();
    request.status = RequestStatus::Active;
    request.created_at = Clock::get()?.unix_timestamp;
    request.bump = context.bumps.funding_request;

    msg!(
        "Funding request created by {} for {} USDC",
        request.requester,
        amount
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(invite_code: String, amount: u64, description: String)]
pub struct CreateRequestAccountConstraints<'info> {
    #[account(mut)]
    pub requester: Signer<'info>,

    #[account(
        seeds = [b"circle", invite_code.as_bytes()],

        bump
    )]
    pub circle_account: Account<'info, CircleAccount>,

    #[account(
        init,

        payer = requester,

        space = FundingRequest::DISCRIMINATOR.len() + FundingRequest::INIT_SPACE,

        seeds = [
            b"request",
            circle_account.key().as_ref(),
            requester.key().as_ref()
        ],

        bump
    )]
    pub funding_request: Account<'info, FundingRequest>,

    pub system_program: Program<'info, System>,
}



