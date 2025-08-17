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
    #[msg("Request not approved - only approved requests can be disbursed")]
    RequestNotApproved,
    #[msg("Insufficient funds in treasury")]
    InsufficientFunds,
    #[msg("Only the requester can create requests")]
    OnlyRequesterCanCreate,
    #[msg("Request has been rejected by the group")]
    RequestRejected,
    #[msg("Request belongs to a different circle")]
    WrongCircle,
    #[msg("Token account does not belong to the requester")]
    WrongTokenOwner,
}



