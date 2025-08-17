use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::state::*;

pub fn create_circle(
    context: Context<CreateCircleAccountConstraints>,
    circle_name: String,
    contribution_amount: u64,
    invite_code: String,
) -> Result<()> {
    let circle = &mut context.accounts.circle_account;
    
    circle.name = circle_name;
    circle.contribution_amount = contribution_amount;
    circle.creator = context.accounts.creator.key();
    circle.invite_code = invite_code.clone();
    circle.member_count = 1;
    circle.member1 = context.accounts.creator.key();
    circle.member2 = Pubkey::default(); // Empty
    circle.member3 = Pubkey::default(); // Empty
    circle.bump = context.bumps.circle_account;
    
    msg!(
        "Circle created with treasury! Invite code: {}",
        invite_code
    );
    Ok(())
}

#[derive(Accounts)]
#[instruction(circle_name: String, contribution_amount: u64, invite_code: String)]
pub struct CreateCircleAccountConstraints<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,

        payer = creator,

        space = CircleAccount::DISCRIMINATOR.len() + CircleAccount::INIT_SPACE,

        seeds = [b"circle", invite_code.as_bytes()],

        bump
    )]
    pub circle_account: Account<'info, CircleAccount>,

    #[account(
        init,

        payer = creator,

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

    pub system_program: Program<'info, System>,

    pub token_program: Interface<'info, TokenInterface>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}



