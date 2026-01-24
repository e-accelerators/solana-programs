use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub deposit_amount: u64,
    pub receive_amount: u64,
    pub bump: u8,
}
