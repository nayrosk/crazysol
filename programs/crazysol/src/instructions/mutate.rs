use anchor_lang::prelude::*;

use crate::{compute_rate_of_centrifugation, constants::{PCRAZY_MUTATION_REWARDS}, errors::CrazySolError, events::MutateEvent, require_initialized, require_operational, states::{LaboratoryState, ReactorState, ExperimentState, ScientistState}};

#[derive(Accounts)]
pub struct Mutate<'info> {
    #[account(
        seeds = [LaboratoryState::SEED],
        bump,
    )]
    pub laboratory_state: Account<'info, LaboratoryState>,

    #[account(
        mut,
        seeds = [ReactorState::SEED],
        bump,
    )]
    pub reactor_state: Account<'info, ReactorState>,

    #[account(
        mut,
        seeds = [ExperimentState::SEED],
        bump,
    )]
    pub experiment_state: Account<'info, ExperimentState>,

    #[account(
        mut,
        seeds = [ScientistState::SEED, owner.key().as_ref()],
        bump,
        has_one = owner,
    )]
    pub scientist_state: Account<'info, ScientistState>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

pub fn handle_mutate(
    ctx: Context<Mutate>,
) -> Result<()> {
    let laboratory_state = &ctx.accounts.laboratory_state;
    let scientist_state = &mut ctx.accounts.scientist_state;
    let reactor_state = &mut ctx.accounts.reactor_state;
    let experiment_state = &mut ctx.accounts.experiment_state;
    let current_ts = Clock::get()?.unix_timestamp;

    require_initialized(laboratory_state)?;
    require_operational(laboratory_state)?;

    scientist_state.incubate_serum(laboratory_state, current_ts)?;

    let pending_yield = scientist_state.distillable_yield;
    if pending_yield == 0 {
        return Err(CrazySolError::NoYield.into());
    }

    experiment_state.total_mutations_performed = experiment_state
        .total_mutations_performed
        .checked_add(pending_yield)
        .ok_or(CrazySolError::Overflow)?;

    let mut new_pill = compute_rate_of_centrifugation(
        reactor_state.pill_supply,
        pending_yield as u128,
        laboratory_state.reaction_formula.bonding_curve_p0,
        laboratory_state.reaction_formula.bonding_curve_m
    );

    if !scientist_state.is_first_mutation_done {
        let bonus = new_pill
            .checked_mul(laboratory_state.reaction_formula.first_mutation_bonus)
            .ok_or(CrazySolError::Overflow)?
            .checked_div(100)
            .ok_or(CrazySolError::Overflow)?;
        new_pill = new_pill.checked_add(bonus).ok_or(CrazySolError::Overflow)?;
        scientist_state.is_first_mutation_done = true;
    }

    if !laboratory_state.innoculation_happened {
        let pre_tge_crazy_rewards = if reactor_state.owned_pcrazy >= PCRAZY_MUTATION_REWARDS {
            PCRAZY_MUTATION_REWARDS
        } else {
            reactor_state.owned_pcrazy
        };

    reactor_state.owned_pcrazy = reactor_state
        .owned_pcrazy
        .checked_sub(pre_tge_crazy_rewards)
        .ok_or(CrazySolError::Overflow)?;

    scientist_state.owned_pcrazy = scientist_state
        .owned_pcrazy
        .checked_add(pre_tge_crazy_rewards)
        .ok_or(CrazySolError::Overflow)?;
    }

    scientist_state.distillable_yield = 0;

    scientist_state.owned_pill = scientist_state.owned_pill
        .checked_add(new_pill)
        .ok_or(CrazySolError::Overflow)?;
    reactor_state.pill_supply = reactor_state.pill_supply
        .checked_add(new_pill)
        .ok_or(CrazySolError::Overflow)?;

    scientist_state.last_distillation_timestamp = current_ts;

    emit!(MutateEvent {
        scientist: *scientist_state.to_account_info().key,
        mutated_yield: pending_yield,
        new_pill,
        total_owned_pill: scientist_state.owned_pill,
        total_owned_pcrazy: scientist_state.owned_pcrazy,
        timestamp: current_ts,
    });

    Ok(())
}
