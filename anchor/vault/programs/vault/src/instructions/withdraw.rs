use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{errors::VaultError, state::Vault};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        require!(amount > 0, VaultError::InvalidAmount);

        let vault = &mut self.vault;

        require!(vault.balance >= amount, VaultError::InsufficientBalance);


        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info()
        };

        let seeds = &[
            b"vault",
            self.user.key.as_ref(),
            &[self.vault.bump]
        ];
        
        let signer_seeds: &[&[&[u8]]] = &[&seeds[..]];
        
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        
        transfer(cpi_ctx, amount)?;
        
        Ok(())
    }
}
