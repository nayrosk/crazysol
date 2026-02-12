use anchor_lang::prelude::*;

use crate::{errors::CrazySolError, events::GiveSomePcrazyEvent, require_initialized, LaboratoryState, ReactorState, ScientistState};

#[derive(Accounts)]
pub struct GivePublicFunding<'info> {
    #[account(
        has_one = director @ CrazySolError::Unauthorized,
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

    /// CHECK: This account is validated manually in the instruction
    #[account(mut)]
    pub receiver: UncheckedAccount<'info>,

    #[account(mut)]
    pub director: Signer<'info>,
}

pub fn handle_give_public_funding(
    ctx: Context<GivePublicFunding>,
    scientist: Pubkey,
    amount: u64,
) -> Result<()> {
    let laboratory_state = &ctx.accounts.laboratory_state;
    let reactor_state = &mut ctx.accounts.reactor_state;

    require_initialized(laboratory_state)?;

    if reactor_state.owned_pcrazy < amount {
        return Err(CrazySolError::InsufficientpCRAZY.into());
    }

    let (expected_pda, _) = Pubkey::find_program_address(
        &[ScientistState::SEED, scientist.as_ref()],
        ctx.program_id,
    );

    if expected_pda != ctx.accounts.receiver.key() {
        return Err(CrazySolError::InvalidScientistAddress.into());
    }

    let mut scientist_data = ctx.accounts.receiver.try_borrow_mut_data()?;

    let mut scientist_state = ScientistState::try_from_slice(&mut scientist_data)
        .map_err(|_| CrazySolError::InvalidScientist)?;

    require!(
        scientist_state.owner == scientist,
        CrazySolError::InvalidScientist
    );

    scientist_state.owned_pcrazy = scientist_state
        .owned_pcrazy
        .checked_add(amount)
        .ok_or(CrazySolError::Overflow)?;

    reactor_state.owned_pcrazy = reactor_state
        .owned_pcrazy
        .checked_sub(amount)
        .ok_or(CrazySolError::Overflow)?;

    scientist_state.serialize(&mut *scientist_data)?;

    emit!(GiveSomePcrazyEvent {
        scientist,
        amount,
        new_scientist_pcrazy: scientist_state.owned_pcrazy,
        new_available_pcrazy_amount: reactor_state.owned_pcrazy,
    });

    Ok(())
}
