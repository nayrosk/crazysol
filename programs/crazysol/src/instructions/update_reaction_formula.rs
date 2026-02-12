use anchor_lang::prelude::*;

use crate::{errors::CrazySolError, events::UpdateReactionFormulaEvent, require_initialized, states::{LaboratoryState, ReactionFormula}};

#[derive(Accounts)]
pub struct UpdateReactionFormula<'info> {
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

pub fn handle_update_reaction_formula(
    ctx: Context<UpdateReactionFormula>,
    new_reaction_formula: ReactionFormula,
) -> Result<()> {
    let laboratory_state = &mut ctx.accounts.laboratory_state;

    require_initialized(laboratory_state)?;

    laboratory_state.reaction_formula = new_reaction_formula;

    emit!(UpdateReactionFormulaEvent {
        new_reaction_formula,
    });

    Ok(())
}
