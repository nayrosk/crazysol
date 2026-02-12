use anchor_lang::prelude::*;

use crate::{constants::{VAPORIZATION_PERCENTAGE}, errors::CrazySolError, events::DistillEvent, require_initialized, require_operational, states::{ExperimentState, LaboratoryState, ReactorState, ScientistState}};

#[derive(Accounts)]
pub struct Distill<'info> {
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

    /// CHECK: big_pharma is validated by address and stored in laboratory state
    #[account(mut, address = laboratory_state.big_pharma)]
    pub big_pharma: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handle_distill(
    ctx: Context<Distill>,
) -> Result<()> {
    let laboratory_state = &ctx.accounts.laboratory_state;
    let scientist_state = &mut ctx.accounts.scientist_state;
    let reactor_state = &mut ctx.accounts.reactor_state;
    let experiment_state = &mut ctx.accounts.experiment_state;
    let current_ts = Clock::get()?.unix_timestamp;

    require_initialized(laboratory_state)?;
    require_operational(laboratory_state)?;

    scientist_state.incubate_serum(laboratory_state, current_ts)?;
    let mut yield_amount = scientist_state.distillable_yield;
    if yield_amount == 0 {
        return Err(CrazySolError::NoYield.into());
    }

    let rent = Rent::get()?;
    let min_balance = rent.minimum_balance(reactor_state.to_account_info().data_len());
    let reactor_balance = reactor_state.to_account_info().lamports();
    let available_balance = reactor_balance
        .checked_sub(min_balance)
        .ok_or(CrazySolError::Overflow)?;

    if yield_amount > available_balance {
        yield_amount = available_balance;
    }

    let fee = yield_amount
        .checked_mul(laboratory_state.containment_tax_bps as u64)
        .ok_or(CrazySolError::Overflow)?
        .checked_div(10_000)
        .ok_or(CrazySolError::Overflow)?;

    if fee > 0 {
        yield_amount = yield_amount
            .checked_sub(fee)
            .ok_or(CrazySolError::Overflow)?;

        **reactor_state.to_account_info().try_borrow_mut_lamports()? = reactor_state
            .to_account_info()
            .lamports()
            .checked_sub(fee)
            .ok_or(CrazySolError::Overflow)?;

        **ctx.accounts.big_pharma.to_account_info().try_borrow_mut_lamports()? = ctx
            .accounts
            .big_pharma
            .to_account_info()
            .lamports()
            .checked_add(fee)
            .ok_or(CrazySolError::Overflow)?;

        experiment_state.total_dev_fees_collected = experiment_state
            .total_dev_fees_collected
            .checked_add(fee)
            .ok_or(CrazySolError::Overflow)?;
    }

    **reactor_state.to_account_info().try_borrow_mut_lamports()? = reactor_state
        .to_account_info()
        .lamports()
        .checked_sub(yield_amount)
        .ok_or(CrazySolError::Overflow)?;

    **ctx.accounts.owner.to_account_info().try_borrow_mut_lamports()? = ctx
        .accounts
        .owner
        .to_account_info()
        .lamports()
        .checked_add(yield_amount)
        .ok_or(CrazySolError::Overflow)?;

    scientist_state.distillable_yield = scientist_state
        .distillable_yield
        .checked_sub(yield_amount)
        .ok_or(CrazySolError::Overflow)?;

    scientist_state.last_distillation_timestamp = current_ts;

    let pill_vaporized = scientist_state.owned_pill
        .checked_mul(VAPORIZATION_PERCENTAGE as u128)
        .ok_or(CrazySolError::Overflow)?
        .checked_div(100)
        .ok_or(CrazySolError::Overflow)?;

    scientist_state.owned_pill = scientist_state.owned_pill
        .checked_sub(pill_vaporized)
        .ok_or(CrazySolError::Overflow)?;

    scientist_state.earned_sol = scientist_state.earned_sol
        .checked_add(yield_amount)
        .ok_or(CrazySolError::Overflow)?;

    reactor_state.pill_supply = reactor_state.pill_supply
        .checked_sub(pill_vaporized)
        .ok_or(CrazySolError::Overflow)?;

    experiment_state.total_pills_vaporized = experiment_state
        .total_pills_vaporized
        .checked_add(pill_vaporized)
        .ok_or(CrazySolError::Overflow)?;

    experiment_state.total_yield_distilled = experiment_state
        .total_yield_distilled
        .checked_add(yield_amount)
        .ok_or(CrazySolError::Overflow)?;

    emit!(DistillEvent {
        scientist: ctx.accounts.owner.key(),
        distilled_yield: yield_amount,
        fee,
        new_pill: scientist_state.owned_pill,
        timestamp: current_ts,
    });

    Ok(())
}
