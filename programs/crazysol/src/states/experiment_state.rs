use anchor_lang::prelude::*;

#[account]
pub struct ExperimentState {
    pub total_dev_fees_collected: u64,
    pub total_pills_vaporized: u128,
    pub total_research_fees_collected: u64,
    pub total_yield_distilled: u64,
    pub total_mutations_performed: u64,
    pub total_sol_injected: u64,
    pub total_scientists: u64,
    pub total_scientists_recruited: u64,
    pub reserved: [u8; 128],
}

impl ExperimentState {
    pub const SIZE: usize =
        8 + // discriminator
        8 + // total_dev_fees_collected
        16 + // total_pills_vaporized
        8 + // total_research_fees_collected
        8 + // total_yield_distilled
        8 + // total_mutations_performed
        8 + // total_sol_injected
        8 + // total_scientists
        8 + // total_scientists_recruited
        128; // reserved
    pub const SEED: &[u8] = b"experiment-state";
}
