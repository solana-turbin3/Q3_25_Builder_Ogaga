use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
    token_interface::{
        Mint, TokenAccount, TokenInterface,
    close_account, transfer_checked, TransferChecked, CloseAccount,
    },
};

use crate::state::{Listing, Marketplace};



#[derive(Accounts)]
 pub struct Delist<'info>{
    #[account(mut)]
    pub maker:Signer<'info>,

    pub maker_mint: InterfaceAccount<'info, Mint>,

      #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,
   

     #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker,
        associated_token::token_program= token_program
    )]
    pub maker_ata: InterfaceAccount<'info, TokenAccount>,

#[account(
        mut,    
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
        associated_token::token_program= token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,


 #[account(
        mut,
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump= listing.bump,
        close=maker
            )]
    pub listing: Account<'info, Listing>,



     pub collection_mint: InterfaceAccount<'info, Mint>,
     
      // Enforce collection membership
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            maker_mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true,
    )]
    pub metadata: Account<'info, MetadataAccount>,
    
    #[account(
        seeds = [
            b"metadata", 
            metadata_program.key().as_ref(),
            maker_mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub master_edition: Account<'info, MasterEditionAccount>,
    
    pub metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>
 }

impl<'info>Delist<'info>{

    pub fn delist_nft(&mut self)->Result<()>{

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked{
            from: self.vault.to_account_info(),
            to: self.maker_ata.to_account_info(),
            authority: self.listing.to_account_info(),
            mint: self.maker_mint.to_account_info(),    
        };

        let seeds = &[
            b"list",
            self.marketplace.to_account_info().key.as_ref(),
            self.maker_mint.to_account_info().key.as_ref(),
            &[self.listing.bump],
        
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx =  CpiContext::new_with_signer( cpi_program, cpi_accounts, signer_seeds );

        transfer_checked(cpi_ctx, 1, 0)?;

        let close_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let close_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            close_accounts,
            signer_seeds,
        );

        close_account(close_ctx)?;
        Ok(())


    }
}