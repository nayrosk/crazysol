use anchor_lang::prelude::*;

use crate::{errors::CrazySolError, events::UpdateContainmentTaxBpsEvent, require_initialized, states::LaboratoryState};

#[derive(Accounts)]
pub struct UpdateContainmentTaxBps<'info> {
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

pub fn handle_update_containment_tax_bps(
    ctx: Context<UpdateContainmentTaxBps>,
    new_containment_tax_bps: u16
) -> Result<()> {
    let laboratory_state = &mut ctx.accounts.laboratory_state;

    require_initialized(laboratory_state)?;

    let previous_containment_tax_bps = laboratory_state.containment_tax_bps;

    if new_containment_tax_bps > 100 {
        return Err(CrazySolError::InvalidFeePercentage.into());
    }

    laboratory_state.containment_tax_bps = new_containment_tax_bps;

    emit!(UpdateContainmentTaxBpsEvent {
        previous_containment_tax_bps,
        new_containment_tax_bps: laboratory_state.containment_tax_bps,
    });

    Ok(())
}
