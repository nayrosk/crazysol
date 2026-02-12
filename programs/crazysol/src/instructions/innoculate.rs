use anchor_lang::prelude::*;

use crate::{errors::CrazySolError, events::InnoculateEvent, require_initialized, require_innoculation_not_happened, states::LaboratoryState};

#[derive(Accounts)]
pub struct Innoculate<'info> {
    #[account(
        mut,
        has_one = director @ CrazySolError::Unauthorized,
        seeds = [LaboratoryState::SEED],
        bump
    )]
    pub laboratory_state: Account<'info, LaboratoryState>,

    #[account(mut)]
    pub director: Signer<'info>,
}

pub fn handle_innoculate(
    ctx: Context<Innoculate>,
) -> Result<()> {
    let laboratory_state = &mut ctx.accounts.laboratory_state;
    let timestamp = Clock::get()?.unix_timestamp;

    require_initialized(laboratory_state)?;
    require_innoculation_not_happened(laboratory_state)?;

    laboratory_state.innoculation_happened = true;

    emit!(InnoculateEvent {
        innoculation_happened: true,
        timestamp,
    });

    Ok(())
}
