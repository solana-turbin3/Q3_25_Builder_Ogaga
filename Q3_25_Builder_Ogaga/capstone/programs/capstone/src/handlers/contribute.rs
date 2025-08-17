use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{self, Mint, TokenAccount, TokenInterface, Transfer},
};

use crate::state::*;
use crate::error::*;

pub fn contribute(context: Context<ContributeAccountConstraints>, _invite_code: String) -> Result<()> {
    let circle = &context.accounts.circle_account;
    let member_key = context.accounts.member.key();
    
    // Check if member is in the circle
    let is_member = member_key == circle.member1 || 
                   member_key == circle.member2 || 
                   member_key == circle.member3;
    
    require!(is_member, CustomError::NotAMember);

    token_interface::transfer(
        context.accounts.into_transfer_context(),
        circle.contribution_amount,
    )?;

    msg!(
        "Member {} contributed {} USDC to circle",
        context.accounts.member.key(),
        circle.contribution_amount
    );
    Ok(())
}

#[derive(Accounts)]
#[instruction(invite_code: String)]
pub struct ContributeAccountConstraints<'info> {
    #[account(mut)]
    pub member: Signer<'info>,

    #[account(
        seeds = [b"circle", invite_code.as_bytes()],

        bump
    )]
    pub circle_account: Account<'info, CircleAccount>,

    #[account(
        mut,

        constraint = member_token_account.owner == member.key()
    )]
    pub member_token_account: InterfaceAccount<'info, TokenAccount>,

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

    pub associated_token_program: Program<'info, AssociatedToken>,
}

// Impl blocks for CPI contexts
impl<'info> ContributeAccountConstraints<'info> {
    pub fn into_transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.member_token_account.to_account_info(),
            to: self.treasury_token_account.to_account_info(),
            authority: self.member.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}



