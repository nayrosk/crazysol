use anchor_lang::{prelude::*, system_program};

use crate::{errors::CrazySolError, events::DepositFromExternalReactorEvent, require_initialized, states::{LaboratoryState, ReactorState}};

#[derive(Accounts)]
pub struct DepositFromExternalReactor<'info> {
    #[account(
        seeds = [LaboratoryState::SEED],
        bump
    )]
    pub laboratory_state: Account<'info, LaboratoryState>,

    #[account(
        mut,
        has_one = external_reactor @ CrazySolError::Unauthorized,
        seeds = [ReactorState::SEED],
        bump
    )]
    pub reactor_state: Account<'info, ReactorState>,

    #[account(mut)]
    pub external_reactor: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handle_deposit_from_external_reactor(
    ctx: Context<DepositFromExternalReactor>, 
    lamports: u64
) -> Result<()> {
    let laboratory_state = &ctx.accounts.laboratory_state;
    let reactor_state = &mut ctx.accounts.reactor_state;
    let timestamp = Clock::get()?.unix_timestamp;

    require_initialized(laboratory_state)?;

    let transfer_instruction = system_program::Transfer {
        from: ctx.accounts.external_reactor.to_account_info(),
        to: reactor_state.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        transfer_instruction,
    );

    system_program::transfer(cpi_ctx, lamports)?;

    reactor_state.sol_reserves = reactor_state
        .sol_reserves
        .checked_add(lamports)
        .ok_or(CrazySolError::Overflow)?;

    emit!(DepositFromExternalReactorEvent {
        amount: lamports,
        timestamp
    });

    Ok(())
}
