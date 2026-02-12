use anchor_lang::prelude::*;

use crate::{constants::{PCRAZY_DAILY_STREAK_BASE_REWARDS, SECONDS_IN_24H}, errors::CrazySolError, events::IncubationPeriodEvent, require_initialized, require_operational, require_innoculation_not_happened, states::{LaboratoryState, ReactorState, ScientistState}};

#[derive(Accounts)]
pub struct IncubationPeriod<'info> {
    #[account(
        seeds = [LaboratoryState::SEED],
        bump
    )]
    pub laboratory_state: Account<'info, LaboratoryState>,

    #[account(
        mut,
        seeds = [ScientistState::SEED, owner.key().as_ref()],
        bump,
        has_one = owner @ CrazySolError::Unauthorized,
    )]
    pub scientist_account: Account<'info, ScientistState>,

    #[account(
        mut,
        seeds = [ReactorState::SEED],
        bump
    )]
    pub reactor_state: Account<'info, ReactorState>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

pub fn handle_incubation_period(
    ctx: Context<IncubationPeriod>,
) -> Result<()> {
    let laboratory_state = &ctx.accounts.laboratory_state;
    let reactor_state = &mut ctx.accounts.reactor_state;
    let scientist_account = &mut ctx.accounts.scientist_account;
    let current_time = Clock::get()?.unix_timestamp;

    require_initialized(laboratory_state)?;
    require_operational(laboratory_state)?;
    require_innoculation_not_happened(laboratory_state)?;

    let time_since_last_streak = if scientist_account.last_streak_timestamp == 0 {
        i64::MAX
    } else {
        current_time
            .checked_sub(scientist_account.last_streak_timestamp)
            .ok_or(CrazySolError::Overflow)?
    };

    let twenty_four_hours = SECONDS_IN_24H as i64;
    let forty_eight_hours = SECONDS_IN_24H
        .checked_mul(2)
        .ok_or(CrazySolError::Overflow)? as i64;

    let reward_amount: u64;
    let new_streak: u32;

    if scientist_account.last_streak_timestamp == 0 {
        reward_amount = PCRAZY_DAILY_STREAK_BASE_REWARDS;
        new_streak = 1;
    } else if time_since_last_streak < twenty_four_hours {
        return Err(CrazySolError::CooldownActive.into());
    } else if time_since_last_streak <= forty_eight_hours {
        new_streak = scientist_account.current_streak
            .checked_add(1)
            .ok_or(CrazySolError::Overflow)?;

        reward_amount = PCRAZY_DAILY_STREAK_BASE_REWARDS
            .checked_mul(new_streak as u64)
            .ok_or(CrazySolError::Overflow)?;
    } else {
        reward_amount = PCRAZY_DAILY_STREAK_BASE_REWARDS;
        new_streak = 1;
    }

    let actual_reward = if reactor_state.owned_pcrazy >= reward_amount {
        reward_amount
    } else {
        reactor_state.owned_pcrazy
    };

    if actual_reward == 0 {
        return Err(CrazySolError::InsufficientpCRAZY.into());
    }

    reactor_state.owned_pcrazy = reactor_state
        .owned_pcrazy
        .checked_sub(actual_reward)
        .ok_or(CrazySolError::Overflow)?;

    scientist_account.owned_pcrazy = scientist_account
        .owned_pcrazy
        .checked_add(actual_reward)
        .ok_or(CrazySolError::Overflow)?;

    scientist_account.current_streak = new_streak;
    scientist_account.last_streak_timestamp = current_time;

    emit!(IncubationPeriodEvent {
        scientist: *scientist_account.to_account_info().key,
        reward: actual_reward,
        current_streak: scientist_account.current_streak,
        total_owned_pcrazy: scientist_account.owned_pcrazy,
        new_timestamp: scientist_account.last_streak_timestamp,
    });

    Ok(())
}
