use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct CircleAccount {
    pub contribution_amount: u64,
    pub creator: Pubkey,
    pub bump: u8,
    
    #[max_len(32)]
    pub name: String,
    
    #[max_len(16)]
    pub invite_code: String,
    
    pub member_count: u8,
    pub member1: Pubkey,
    pub member2: Pubkey,
    pub member3: Pubkey,
}


