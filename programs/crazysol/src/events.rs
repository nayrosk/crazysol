use anchor_lang::prelude::*;

use crate::{ ReactionFormula};

#[event]
pub struct InjectEvent {
    pub scientist: Pubkey,
    pub amount: u64,
    pub new_pill: u128,
    pub total_owned_pill: u128,
    pub total_owned_pcrazy: u64,
    pub timestamp: i64,
}

#[event]
pub struct InnoculateEvent {
    pub innoculation_happened: bool,
    pub timestamp: i64,
}

#[event]
pub struct DistillEvent {
    pub scientist: Pubkey,
    pub distilled_yield: u64,
    pub new_pill: u128,
    pub fee: u64,
    pub timestamp: i64,
}

#[event]
pub struct MutateEvent {
    pub scientist: Pubkey,
    pub mutated_yield: u64,
    pub new_pill: u128,
    pub total_owned_pill: u128,
    pub total_owned_pcrazy: u64,
    pub timestamp: i64,
}

#[event]
pub struct IncubationPeriodEvent {
    pub scientist: Pubkey,
    pub reward: u64,
    pub current_streak: u32,
    pub total_owned_pcrazy: u64,
    pub new_timestamp: i64,
}

#[event]
pub struct GiveSomePcrazyEvent {
    pub scientist: Pubkey,
    pub amount: u64,
    pub new_scientist_pcrazy: u64,
    pub new_available_pcrazy_amount: u64,
}

#[event]
pub struct DepositFromExternalReactorEvent {
    pub amount: u64,
    pub timestamp: i64
}

#[event]
pub struct IncreasepCRAZYLiquidityEvent {
    pub previous_available_pcrazy: u64,
    pub new_available_pcrazy: u64
}

#[event]
pub struct InitializeEvent {
    pub director: Pubkey,
    pub big_pharma: Pubkey,
    pub containment_tax_bps: u16,
    pub owned_pcrazy: u64,
    pub external_reactor: Pubkey,
    pub reaction_formula: ReactionFormula,
}

#[event]
pub struct RegisterScientistEvent {
    pub scientist: Pubkey,
    pub recruiter: Option<Pubkey>,
    pub recruiter_reward: Option<u64>,
}

#[event]
pub struct SwitchEmergencyLockdownEvent {
    pub is_emergency_lockdown: bool,
}

#[event]
pub struct ReplaceDirectorEvent {
    pub old_director: Pubkey,
    pub new_director: Pubkey,
}

#[event]
pub struct UpdateBigPharmaEvent {
    pub old_big_pharma: Pubkey,
    pub new_big_pharma: Pubkey,
}

#[event]
pub struct UpdateContainmentTaxBpsEvent {
    pub previous_containment_tax_bps: u16,
    pub new_containment_tax_bps: u16,
}

#[event]
pub struct UpdateRewardRatePerPillEvent {
    pub old_reward_rate_per_pill: u64,
    pub new_reward_rate_per_pill: u64,
}

#[event]
pub struct UpdateReactionFormulaEvent {
    pub new_reaction_formula: ReactionFormula,
}
