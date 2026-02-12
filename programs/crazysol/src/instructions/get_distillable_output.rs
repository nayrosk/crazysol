use anchor_lang::prelude::*;
use crate::{
    constants::SECONDS_IN_24H,
    errors::CrazySolError,
    states::{LaboratoryState, ScientistState},
};

#[derive(Accounts)]
pub struct GetDistillableOutput<'info> {
    #[account(
        seeds = [LaboratoryState::SEED],
        bump,
    )]
    pub laboratory_state: Account<'info, LaboratoryState>,

    #[account(
        seeds = [ScientistState::SEED, scientist.key().as_ref()],
        bump,
    )]
    pub scientist_state: Account<'info, ScientistState>,

    /// CHECK: This is just used for PDA derivation
    pub scientist: AccountInfo<'info>,
}

pub fn handle_get_distillable_output(
    ctx: Context<GetDistillableOutput>,
) -> Result<u64> {
    let laboratory_state = &ctx.accounts.laboratory_state;
    let scientist_state = &ctx.accounts.scientist_state;
    let current_ts = Clock::get()?.unix_timestamp;


    if scientist_state.owned_pill == 0 {
        return Ok(scientist_state.distillable_yield);
    }

    let elapsed = current_ts
        .checked_sub(scientist_state.last_distillation_timestamp)
        .ok_or(CrazySolError::Overflow)?;

    if elapsed <= 0 {
        return Ok(scientist_state.distillable_yield);
    }

    let base_rate = laboratory_state.reaction_formula.reward_rate_per_pill;

    let min_daily_yield = (100_000u64)
        .checked_mul(laboratory_state.reaction_formula.min_daily_yield_bps)
        .ok_or(CrazySolError::Overflow)?
        .checked_div(10_000) // convert bps -> %
        .ok_or(CrazySolError::Overflow)?;

    let min_rate_per_sec = min_daily_yield
        .checked_div(SECONDS_IN_24H)
        .ok_or(CrazySolError::Overflow)?;

    let adjusted_rate = if base_rate < min_rate_per_sec {
        min_rate_per_sec
    } else {
        base_rate
    };

    let yield_amount = scientist_state.owned_pill
        .checked_mul(adjusted_rate as u128)
        .ok_or(CrazySolError::Overflow)?
        .checked_mul(elapsed as u128)
        .ok_or(CrazySolError::Overflow)?;

    let yield_u64: u64 = yield_amount
        .try_into()
        .map_err(|_| CrazySolError::Overflow)?;

    let total_yield = scientist_state.distillable_yield
        .checked_add(yield_u64)
        .ok_or(CrazySolError::Overflow)?;

    Ok(total_yield)
}
