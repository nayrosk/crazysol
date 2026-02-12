use anchor_lang::prelude::*;

use crate::{errors::CrazySolError, events::UpdateBigPharmaEvent, require_initialized, states::LaboratoryState};

#[derive(Accounts)]
pub struct UpdateBigPharma<'info> {
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

pub fn handle_update_big_pharma(
    ctx: Context<UpdateBigPharma>,
    new_big_pharma: Pubkey
) -> Result<()> {
    let laboratory_state = &mut ctx.accounts.laboratory_state;

    require_initialized(laboratory_state)?;

    let old_big_pharma = laboratory_state.big_pharma;

    if !new_big_pharma.is_on_curve() {
        return Err(CrazySolError::InvalidPubkey.into());
    }

    laboratory_state.big_pharma = new_big_pharma;

    emit!(UpdateBigPharmaEvent {
        old_big_pharma,
        new_big_pharma,
    });

    Ok(())
}
