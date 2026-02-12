use anchor_lang::prelude::*;

use crate::{errors::CrazySolError, events::{SwitchEmergencyLockdownEvent}, require_initialized, states::LaboratoryState};

#[derive(Accounts)]
pub struct SwitchEmergencyLockdown<'info> {
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

pub fn handle_switch_emergency_lockdown(
    ctx: Context<SwitchEmergencyLockdown>,
    emergency_lockdown: bool
) -> Result<()> {
    let laboratory_state = &mut ctx.accounts.laboratory_state;

    require_initialized(laboratory_state)?;

    laboratory_state.emergency_lockdown = emergency_lockdown;

    emit!(SwitchEmergencyLockdownEvent {
        is_emergency_lockdown: emergency_lockdown,
    });

    Ok(())
}
