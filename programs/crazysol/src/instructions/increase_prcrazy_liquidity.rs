use anchor_lang::prelude::*;

use crate::{errors::CrazySolError, events::IncreasepCRAZYLiquidityEvent, require_initialized, require_innoculation_not_happened, states::{LaboratoryState, ReactorState}};

#[derive(Accounts)]
pub struct IncreasepCRAZYLiquidity<'info> {
    #[account(
        has_one = director @ CrazySolError::Unauthorized,
        seeds = [LaboratoryState::SEED],
        bump
    )]
    pub laboratory_state: Account<'info, LaboratoryState>,

    #[account(
        mut,
        seeds = [ReactorState::SEED],
        bump
    )]
    pub reactor_state: Account<'info, ReactorState>,

    #[account(mut)]
    pub director: Signer<'info>,
}

pub fn handle_increase_pcrazy_liquidity(
    ctx: Context<IncreasepCRAZYLiquidity>,
    new_available_pcrazy_amount: u64
) -> Result<()> {
    let laboratory_state = &ctx.accounts.laboratory_state;
    let reactor_state = &mut ctx.accounts.reactor_state;

    require_initialized(laboratory_state)?;
    require_innoculation_not_happened(laboratory_state)?;

    let previous_available_pcrazy = reactor_state.owned_pcrazy;

    reactor_state.owned_pcrazy = reactor_state
        .owned_pcrazy
        .checked_add(new_available_pcrazy_amount)
        .ok_or(CrazySolError::Overflow)?;

    emit!(IncreasepCRAZYLiquidityEvent {
        previous_available_pcrazy,
        new_available_pcrazy: reactor_state.owned_pcrazy
    });

    Ok(())
}
