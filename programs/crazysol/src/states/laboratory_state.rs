use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, InitSpace, Copy)]
pub struct ReactionFormula {
    pub bonding_curve_m: u8,
    pub bonding_curve_p0: u32,
    pub reward_rate_per_pill: u64,
    pub min_daily_yield_bps: u64,
    pub first_injection_bonus: u128,
    pub first_mutation_bonus: u128,
}

#[account]
pub struct LaboratoryState {
    pub director: Pubkey,
    pub big_pharma: Pubkey,
    pub containment_tax_bps: u16,
    pub is_initialized: bool,
    pub emergency_lockdown: bool,
    pub innoculation_happened: bool,
    pub reserved: [u8; 128],
    pub reaction_formula: ReactionFormula,
}

impl LaboratoryState {
    pub const SIZE: usize =
        8 + // discriminator
        32 + // director
        32 + // big_pharma
        2 + // fee_percentage
        1 + // is_initialized
        1 + // is_paused
        1 + // tge_happened
        128 + // reserved
        ReactionFormula::INIT_SPACE;
    pub const SEED: &[u8] = b"laboratory-state";
}
