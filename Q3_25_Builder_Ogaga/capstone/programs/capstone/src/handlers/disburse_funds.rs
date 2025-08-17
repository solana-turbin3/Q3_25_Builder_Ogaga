use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{self, Mint, TokenAccount, TokenInterface, Transfer},
};

use crate::state::*;
use crate::error::*;

pub fn disburse_funds(context: Context<DisburseFundsAccountConstraints>, invite_code: String) -> Result<()> {
    let _request = &context.accounts.funding_request;
    
    // Verify request is approved
    require!(
        context.accounts.funding_request.status == RequestStatus::Approved,
        CustomError::RequestNotApproved
    );

    // Check treasury has sufficient funds
    let treasury_balance = context.accounts.treasury_token_account.amount;
    let request_amount = context.accounts.funding_request.amount;
    require!(
        treasury_balance >= request_amount,
        CustomError::InsufficientFunds
    );

    // Prepare signer seeds for treasury authority
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"treasury_auth",
        invite_code.as_bytes(),
        &[context.bumps.treasury_authority],
    ]];

    // Transfer USDC from treasury to requester
    token_interface::transfer(
        context
            .accounts
            .into_transfer_context()
            .with_signer(signer_seeds),
        request_amount,
    )?;

    // Mark request as disbursed
    let request = &mut context.accounts.funding_request;
    request.status = RequestStatus::Disbursed;

    msg!(
        "Disbursed {} USDC to {}",
        request.amount,
        request.requester
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(invite_code: String)]
pub struct DisburseFundsAccountConstraints<'info> {
    #[account(mut)]
    pub authority: Signer<'info>, // Could be anyone, the logic is in the constraints

    #[account(
        seeds = [b"circle", invite_code.as_bytes()],

        bump
    )]
    pub circle_account: Account<'info, CircleAccount>,

    #[account(
        mut,

        constraint = funding_request.circle == circle_account.key(),

        constraint = funding_request.status == RequestStatus::Approved
    )]
    pub funding_request: Account<'info, FundingRequest>,

    #[account(
        mut,

        constraint = requester_token_account.owner == funding_request.requester
    )]
    pub requester_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,

        associated_token::mint = usdc_mint,

        associated_token::authority = treasury_authority,
    )]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: Treasury authority PDA
    #[account(
        seeds = [b"treasury_auth", invite_code.as_bytes()],

        bump
    )]
    pub treasury_authority: UncheckedAccount<'info>,

    pub usdc_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
}

// Impl blocks for CPI contexts
impl<'info> DisburseFundsAccountConstraints<'info> {
    pub fn into_transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.treasury_token_account.to_account_info(),
            to: self.requester_token_account.to_account_info(),
            authority: self.treasury_authority.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}
