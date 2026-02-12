use anchor_lang::prelude::*;

use crate::{constants::SECONDS_IN_24H, errors::CrazySolError, states::{LaboratoryState, ScientistState}};

impl ScientistState {
    pub fn incubate_serum(&mut self, laboratory_state: &LaboratoryState, current_ts: i64) -> Result<()> {
        if self.owned_pill == 0 {
            return Ok(());
        }

        let elapsed = current_ts
            .checked_sub(self.last_distillation_timestamp)
            .ok_or(CrazySolError::Overflow)?;
        if elapsed <= 0 {
            return Ok(());
        }

        let base_rate = laboratory_state.reaction_formula.reward_rate_per_pill;

        let min_daily_yield = (100_000u64)
            .checked_mul(laboratory_state.reaction_formula.min_daily_yield_bps)
            .ok_or(CrazySolError::Overflow)?
            .checked_div(10_000) 
            .ok_or(CrazySolError::Overflow)?;

        let min_rate_per_sec = min_daily_yield
            .checked_div(SECONDS_IN_24H)
            .ok_or(CrazySolError::Overflow)?;

        let adjusted_rate = if base_rate < min_rate_per_sec {
            min_rate_per_sec
        } else {
            base_rate
        };

        let yield_amount = self.owned_pill
            .checked_mul(adjusted_rate as u128)
            .ok_or(CrazySolError::Overflow)?
            .checked_mul(elapsed as u128)
            .ok_or(CrazySolError::Overflow)?;

        let yield_u64: u64 = yield_amount
            .try_into()
            .map_err(|_| CrazySolError::Overflow)?;

        self.distillable_yield = self.distillable_yield
            .checked_add(yield_u64)
            .ok_or(CrazySolError::Overflow)?;
        self.last_distillation_timestamp = current_ts;

        Ok(())
    }
}
