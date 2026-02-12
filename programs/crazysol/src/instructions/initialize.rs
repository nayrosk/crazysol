use anchor_lang::prelude::*;

use crate::{events::InitializeEvent, states::{LaboratoryState, ReactorState, ExperimentState, ReactionFormula}, utils::require_not_initialized};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub director: Signer<'info>,

    #[account(
        init,
        payer = director,
        space = LaboratoryState::SIZE,
        seeds = [LaboratoryState::SEED],
        bump
    )]
    pub laboratory_state: Account<'info, LaboratoryState>,

    #[account(
        init,
        payer = director,
        space = ReactorState::SIZE,
        seeds = [ReactorState::SEED],
        bump
    )]
    pub reactor: Account<'info, ReactorState>,

    #[account(
        init,
        payer = director,
        space = ExperimentState::SIZE,
        seeds = [ExperimentState::SEED],
        bump
    )]
    pub experiment_state: Account<'info, ExperimentState>,

    pub system_program: Program<'info, System>,
}

pub fn handle_initialize(
    ctx: Context<Initialize>,
    big_pharma: Pubkey,
    containment_tax_bps: u16,
    owned_pcrazy: u64,
    external_reactor: Pubkey
) -> Result<()> {
    let laboratory_state = &mut ctx.accounts.laboratory_state;
    let reactor_state = &mut ctx.accounts.reactor;
    let experiment_state = &mut ctx.accounts.experiment_state;

    require_not_initialized(laboratory_state)?;

    laboratory_state.director = ctx.accounts.director.key();
    laboratory_state.big_pharma = big_pharma;
    laboratory_state.containment_tax_bps = containment_tax_bps;
    laboratory_state.is_initialized = true;
    laboratory_state.emergency_lockdown = false;
    laboratory_state.innoculation_happened = false;

    laboratory_state.reaction_formula = ReactionFormula {
        bonding_curve_m: 30,
        bonding_curve_p0: 100_000,
        reward_rate_per_pill: 2_222,
        min_daily_yield_bps: 800,
        first_injection_bonus: 10,
        first_mutation_bonus: 15,
    };

    reactor_state.sol_reserves = 0;
    reactor_state.owned_pcrazy = owned_pcrazy;
    reactor_state.pill_supply = 0;
    reactor_state.external_reactor = external_reactor;

    experiment_state.total_dev_fees_collected = 0;
    experiment_state.total_pills_vaporized = 0;
    experiment_state.total_research_fees_collected = 0;
    experiment_state.total_yield_distilled = 0;
    experiment_state.total_mutations_performed = 0;
    experiment_state.total_sol_injected = 0;
    experiment_state.total_scientists = 0;
    experiment_state.total_scientists_recruited = 0;

    emit!(InitializeEvent {
        director: laboratory_state.director,
        big_pharma,
        containment_tax_bps,
        owned_pcrazy,
        external_reactor,
        reaction_formula: laboratory_state.reaction_formula,
    });

    Ok(())
}
