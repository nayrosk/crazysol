use anchor_lang::prelude::*;

#[account]
pub struct ReactorState {
    pub sol_reserves: u64,
    pub owned_pcrazy: u64,
    pub pill_supply: u128,
    pub external_reactor: Pubkey,
    pub reserved: [u8; 128],
}

impl ReactorState {
    pub const SIZE: usize =
        8 + // discriminator
        8 + // sol_reserves
        8 + // owned_pre_tge_crazy
        16 + // pill_supply
        32 + // external_reactor
        128; // reserved
    pub const SEED: &[u8] = b"reactor-state";
}
