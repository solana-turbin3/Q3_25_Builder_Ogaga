use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("You are not a member of this circle")]
    NotAMember,
    #[msg("Invalid invite code")]
    InvalidInviteCode,
    #[msg("Request not found")]
    RequestNotFound,
    #[msg("Request is not active")]
    RequestNotActive,
    #[msg("You have already voted on this request")]
    AlreadyVoted,
    #[msg("Request not approved")]
    RequestNotApproved,
    #[msg("Insufficient funds in treasury")]
    InsufficientFunds,
    #[msg("Only the requester can create requests")]
    OnlyRequesterCanCreate,
}



