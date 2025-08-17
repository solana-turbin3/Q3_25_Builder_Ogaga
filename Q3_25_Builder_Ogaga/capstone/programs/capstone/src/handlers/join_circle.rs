use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::*;

pub fn join_circle(context: Context<JoinCircleAccountConstraints>, invite_code: String) -> Result<()> {
    let circle = &mut context.accounts.circle_account;
    
    require!(
        circle.invite_code == invite_code,
        CustomError::InvalidInviteCode
    );
    
    // Add member to next available slot
    let joiner_key = context.accounts.joiner.key();
    if circle.member_count < 3 {
        match circle.member_count {
            1 => circle.member2 = joiner_key,
            2 => circle.member3 = joiner_key,
            _ => return Err(CustomError::NotAMember.into()),
        }
        circle.member_count += 1;
    }
    
    msg!("User {} joined circle!", context.accounts.joiner.key());
    Ok(())
}

#[derive(Accounts)]
#[instruction(invite_code: String)]
pub struct JoinCircleAccountConstraints<'info> {
    #[account(mut)]
    pub joiner: Signer<'info>,

    #[account(
        mut,

        seeds = [b"circle", invite_code.as_bytes()],

        bump
    )]
    pub circle_account: Account<'info, CircleAccount>,
}

