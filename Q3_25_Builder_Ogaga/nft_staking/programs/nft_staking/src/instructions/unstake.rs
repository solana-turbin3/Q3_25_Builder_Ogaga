use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            ThawDelegatedAccountCpi, 
            ThawDelegatedAccountCpiAccounts
        }, 
        MasterEditionAccount, 
        Metadata
    }, 
    token::{
        revoke, 
        Mint, 
        Revoke, 
        Token, 
        TokenAccount
    }
};

use crate::{error::StakeError, state::{StakeAccount, StakeConfig, UserAccount}};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub mint_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"stake".as_ref(), mint.key().as_ref(), config.key().as_ref()],
        bump = stake_account.bump,
        close = user,
    )]
    pub stake_account: Account<'info, StakeAccount>,
    #[account(
        seeds = [b"config".as_ref()],
        bump = config.bump,
    )]
    pub config: Account<'info, StakeConfig>,
    #[account(
        mut,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub edition: Account<'info, MasterEditionAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
}

impl<'info> Unstake<'info> {
    pub fn unstake(&mut self, _bumps: &UnstakeBumps) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        let freeze_period = self.config.freeze_period as i64;
        
        require!(
            current_time - self.stake_account.staked_at >= freeze_period,
            StakeError::FreezePeriodNotPassed
        );

        // Thaw the delegated account
        let seeds = &[
            b"stake",
            self.mint.to_account_info().key.as_ref(),
            self.config.to_account_info().key.as_ref(),
            &[self.stake_account.bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.mint_ata.to_account_info();
        let edition = &self.edition.to_account_info();
        let mint = &self.mint.to_account_info();
        let token_program = &self.token_program.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();

        ThawDelegatedAccountCpi::new(
            metadata_program,
            ThawDelegatedAccountCpiAccounts {
                delegate,
                token_account,
                edition,
                mint,
                token_program,
            },
        ).invoke_signed(signer_seeds)?;

        // Revoke approval
        let cpi_accounts = Revoke {
            source: self.mint_ata.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        revoke(cpi_ctx)?;

        // Update user account
        self.user_account.amount_staked -= 1;

        Ok(())
    }
}