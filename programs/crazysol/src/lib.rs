use anchor_lang::prelude::*;

declare_id!("8RRuAhgwCR5WmMG7ZkGt4YUFTuB2cYiQtWKhQwv6xFDa");

pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod states;
pub mod utils;

pub use instructions::*;
pub use utils::*;
pub use states::*;

#[program]
pub mod crazysol {
    use crate::states::ReactionFormula;

    use super::*;

    // -------------------------
    // directoristration functions
    // -------------------------

    pub fn innoculate(
        ctx: Context<Innoculate>,
    ) -> Result<()> {
        handle_innoculate(
            ctx
        )
    }

    pub fn increase_pcrazy_liquidity(
        ctx: Context<IncreasepCRAZYLiquidity>,
        new_available_pcrazy_amount: u64
    ) -> Result<()> {
        handle_increase_pcrazy_liquidity(
            ctx,
            new_available_pcrazy_amount
        )
    }

    pub fn deposit_from_external_reactor(
        ctx: Context<DepositFromExternalReactor>,
        lamports: u64
    ) -> Result<()> {
        handle_deposit_from_external_reactor(
            ctx,
            lamports)
    }

    pub fn initialize(
        ctx: Context<Initialize>,
        big_pharma: Pubkey,
        containment_tax_bps: u16,
        owned_pcrazy: u64,
        external_reactor: Pubkey
    ) -> Result<()> {
        handle_initialize(
            ctx,
            big_pharma,
            containment_tax_bps,
            owned_pcrazy,
            external_reactor
        )
    }

    pub fn switch_emergency_lockdown(
        ctx: Context<SwitchEmergencyLockdown>,
        emergency_lockdown: bool
    ) -> Result<()> {
        handle_switch_emergency_lockdown(
            ctx,
            emergency_lockdown
        )
    }

    pub fn replace_director(
        ctx: Context<ReplaceDirector>,
        new_director: Pubkey
    ) -> Result<()> {
        handle_replace_director(
            ctx,
            new_director
        )
    }

    pub fn update_containment_tax_bps(
        ctx: Context<UpdateContainmentTaxBps>,
        new_containment_tax_bps: u16
    ) -> Result<()> {
        handle_update_containment_tax_bps(
            ctx,
            new_containment_tax_bps
        )
    }

    pub fn update_big_pharma(
        ctx: Context<UpdateBigPharma>,
        new_big_pharma: Pubkey
    ) -> Result<()> {
        handle_update_big_pharma(
            ctx,
            new_big_pharma
        )
    }

    pub fn update_reaction_formula(
        ctx: Context<UpdateReactionFormula>,
        new_reaction_formula: ReactionFormula,
    ) -> Result<()> {
        handle_update_reaction_formula(
            ctx,
            new_reaction_formula
        )
    }

    pub fn give_public_funding(
        ctx: Context<GivePublicFunding>,
        scientist: Pubkey,
        amount: u64
    ) -> Result<()> {
        handle_give_public_funding(
            ctx,
            scientist,
            amount
        )
    }

    // ---------------
    // User functions
    // ---------------

    pub fn inject<'info>(
        ctx: Context<'_, '_, '_, 'info, Inject<'info>>,
        deposit: u64,
    ) -> Result<()> {
            handle_inject(
                ctx,
                deposit
            )
        }

    pub fn distill(
        ctx: Context<Distill>,
    ) -> Result<()> {
        handle_distill(
            ctx
        )
    }

    pub fn mutate(
        ctx: Context<Mutate>,
    ) -> Result<()> {
        handle_mutate(
            ctx
        )
    }

    pub fn incubation_period(
        ctx: Context<IncubationPeriod>,
    ) -> Result<()> {
        handle_incubation_period(
            ctx
        )
    }

    pub fn register_scientist<'info>(
        ctx: Context<'_, '_, '_, 'info, RegisterScientist<'info>>,
        username: String,
        recruiter: Option<Pubkey>,
    ) -> Result<()> {
        handle_register_scientist(
            ctx,
            username,
            recruiter
        )
    }

    // ---------------
    // "View" functions
    // ---------------

    pub fn get_distillable_output(
        ctx: Context<GetDistillableOutput>,
    ) -> Result<u64> {
        handle_get_distillable_output(
            ctx
        )
    }

    pub fn get_pill_potency(
        ctx: Context<GetPillPotency>,
    ) -> Result<u64> {
        handle_get_pill_potency(
            ctx
        )
    }
}
