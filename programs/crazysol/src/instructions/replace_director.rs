use anchor_lang::prelude::*;

use crate::{errors::CrazySolError, events::ReplaceDirectorEvent, require_initialized, states::LaboratoryState};

#[derive(Accounts)]
pub struct ReplaceDirector<'info> {
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

pub fn handle_replace_director(
    ctx: Context<ReplaceDirector>,
    new_director: Pubkey
) -> Result<()> {
    let laboratory_state = &mut ctx.accounts.laboratory_state;

    require_initialized(laboratory_state)?;

    let old_director = laboratory_state.director;

    laboratory_state.director = new_director;

    emit!(ReplaceDirectorEvent {
        old_director,
        new_director,
    });

    Ok(())
}
