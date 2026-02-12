use anchor_lang::prelude::*;
use crate::{
    errors::CrazySolError,
    states::{LaboratoryState, ReactorState},
};

#[derive(Accounts)]
pub struct GetPillPotency<'info> {
    #[account(
        seeds = [LaboratoryState::SEED],
        bump,
    )]
    pub laboratory_state: Account<'info, LaboratoryState>,

    #[account(
        seeds = [ReactorState::SEED],
        bump,
    )]
    pub reactor_state: Account<'info, ReactorState>,
}

pub fn handle_get_pill_potency(
    ctx: Context<GetPillPotency>,
) -> Result<u64> {
    let laboratory_state = &ctx.accounts.laboratory_state;
    let reactor_state = &ctx.accounts.reactor_state;

    let p0 = laboratory_state.reaction_formula.bonding_curve_p0 as u128;
    let m = laboratory_state.reaction_formula.bonding_curve_m as u128;
    let current_supply = reactor_state.pill_supply;

    let price = p0
        .checked_add(
            m.checked_mul(current_supply)
                .ok_or(CrazySolError::Overflow)?
        )
        .ok_or(CrazySolError::Overflow)?;

    let price_u64: u64 = price
        .try_into()
        .map_err(|_| CrazySolError::Overflow)?;

    Ok(price_u64)
}
