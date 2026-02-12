use anchor_lang::prelude::*;

#[account]
pub struct ScientistState {
    pub owner: Pubkey,
    pub username: String,
    pub recruiter: Option<Pubkey>,
    pub owned_pill: u128,
    pub owned_pcrazy: u64,
    pub is_first_injection_done: bool,
    pub is_first_mutation_done: bool,
    pub sol_injected: u64,
    pub last_streak_timestamp: i64,
    pub current_streak: u32,
    pub last_distillation_timestamp: i64,
    pub distillable_yield: u64,
    pub test_subjects_count: u32,
    pub specimens_count: u32,
    pub samples_count: u32,
    pub earned_sol: u64,
    pub earned_sol_from_research: u64,
    pub reserved: [u8; 128],
}

impl ScientistState {
    pub const MAX_USERNAME_LENGTH: usize = 128;
    pub const SIZE: usize =
        8 + // discriminator
        32 + // owner
        4 + Self::MAX_USERNAME_LENGTH + // username
        33 + // recruiter
        16 + // owned_pill
        8 + // owned_pre_tge_crazy
        1 + // is_first_injection_done
        1 + // is_first_mutation_done
        8 + // sol_injected
        8 + // last_streak_timestamp
        4 + // current_streak
        8 + // last_distillation_timestamp
        8 + // distillable_yield
        4 + // test_subjects_count
        4 + // specimens_count
        4 + // samples_count
        8 + // earned_sol
        8 + // earned_sol_from_research
        128; // reserved
    pub const SEED: &[u8] = b"scientist-state";
}
