use anchor_lang::prelude::*;

use crate::{errors::CrazySolError, states::LaboratoryState};

pub fn verify_laboratory_initialized_and_operational(laboratory_state: &LaboratoryState) -> Result<()> {
    require_initialized(laboratory_state)?;
    require_operational(laboratory_state)?;
    Ok(())
}

pub fn require_not_initialized(laboratory_state: &LaboratoryState) -> Result<()> {
    require!(
        !laboratory_state.is_initialized,
        CrazySolError::AlreadyInitialized
    );
    Ok(())
}

pub fn require_initialized(laboratory_state: &LaboratoryState) -> Result<()> {
    require!(
        laboratory_state.is_initialized,
        CrazySolError::AlreadyInitialized
    );
    Ok(())
}

pub fn require_operational(laboratory_state: &LaboratoryState) -> Result<()> {
    require!(
        !laboratory_state.emergency_lockdown,
        CrazySolError::CurrentlyPaused
    );
    Ok(())
}

pub fn require_innoculation_not_happened(laboratory_state: &LaboratoryState) -> Result<()> {
    require!(
        !laboratory_state.innoculation_happened,
        CrazySolError::InnoculationAlreadyHappened
    );
    Ok(())
}
