use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct FundingRequest {
    pub requester: Pubkey,
    pub circle: Pubkey,
    pub amount: u64,
    pub votes_for: u32,
    pub votes_against: u32,
    pub status: RequestStatus,
    pub created_at: i64,
    pub bump: u8,
    
    #[max_len(100)]
    pub description: String,
    
    pub voter_count: u8,
    pub voter1: Pubkey,
    pub voter2: Pubkey,
    pub voter3: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum RequestStatus {
    Active,
    Approved,
    Rejected,
    Disbursed,
}


