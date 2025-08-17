#![allow(unexpected_cfgs)]
#![allow(deprecated)]
#![allow(ambiguous_glob_reexports)]
use anchor_lang::prelude::*;

declare_id!("3ZN15DR98zTR6nb6A9ekAziw2vRZ3JiWMZ3nrSyNkjMV");

pub mod error;
pub mod state;
pub mod handlers;

pub use error::*;
pub use state::*;
pub use handlers::*;

#[program]
pub mod capstone {
    use super::*;

    pub fn create_circle(
        context: Context<CreateCircleAccountConstraints>,
        circle_name: String,
        contribution_amount: u64,
        invite_code: String,
    ) -> Result<()> {
        create_circle::create_circle(context, circle_name, contribution_amount, invite_code)
    }

    pub fn join_circle(context: Context<JoinCircleAccountConstraints>, invite_code: String) -> Result<()> {
        join_circle::join_circle(context, invite_code)
    }

    pub fn contribute(context: Context<ContributeAccountConstraints>, invite_code: String) -> Result<()> {
        contribute::contribute(context, invite_code)
    }

    pub fn create_request(
        context: Context<CreateRequestAccountConstraints>,
        invite_code: String,
        amount: u64,
        description: String,
    ) -> Result<()> {
        create_request::create_request(context, invite_code, amount, description)
    }

    pub fn vote_on_request(
        context: Context<VoteOnRequestAccountConstraints>,
        invite_code: String,
        vote: bool,
    ) -> Result<()> {
        vote_on_request::vote_on_request(context, invite_code, vote)
    }

    pub fn disburse_funds(context: Context<DisburseFundsAccountConstraints>, invite_code: String) -> Result<()> {
        disburse_funds::disburse_funds(context, invite_code)
    }
}