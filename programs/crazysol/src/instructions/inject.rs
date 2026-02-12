use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction::transfer},
};

use crate::{
    constants::{PCRAZY_INJECTION_REWARDS, RESEARCH_REWARD_PERCENTAGES},
    errors::CrazySolError,
    events::InjectEvent,
    require_initialized, require_operational,
    states::{LaboratoryState, ReactorState, ExperimentState, ScientistState},
    utils::compute_rate_of_centrifugation,
};

#[derive(Accounts)]
pub struct Inject<'info> {
    #[account(
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

    #[account(
        mut,
        seeds = [ExperimentState::SEED],
        bump,
    )]
    pub experiment_state: Account<'info, ExperimentState>,

    #[account(
        mut,
        seeds = [ScientistState::SEED, owner.key().as_ref()],
        bump,
        has_one = owner,
    )]
    pub scientist_state: Account<'info, ScientistState>,

    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK: big_pharma is validated by address and stored in laboratory state
    #[account(mut, address = laboratory_state.big_pharma)]
    pub big_pharma: UncheckedAccount<'info>,

    /// CHECK: external_reactor is validated by address and stored in reactor state
    #[account(mut, address = reactor_state.external_reactor)]
    pub external_reactor: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>
}

pub fn handle_inject<'info>(
    ctx: Context<'_, '_, '_, 'info, Inject<'info>>,
    deposit: u64,
) -> Result<()> {
    let laboratory_state = &ctx.accounts.laboratory_state;
    let scientist_state = &mut ctx.accounts.scientist_state;
    let experiment_state = &mut ctx.accounts.experiment_state;
    let reactor_state = &mut ctx.accounts.reactor_state;
    let current_ts = Clock::get()?.unix_timestamp;
    let mut remaining_deposit = deposit;
    let original_deposit = deposit;

    require_initialized(laboratory_state)?;
    require_operational(laboratory_state)?;

    scientist_state.incubate_serum(laboratory_state, current_ts)?;

    if deposit < 50_000_000 {
        return Err(CrazySolError::InjectionTooSmall.into());
    }

    let fee = deposit
        .checked_mul(laboratory_state.containment_tax_bps as u64)
        .ok_or(CrazySolError::Overflow)?
        .checked_div(10_000)
        .ok_or(CrazySolError::Overflow)?;
    
    if fee > 0 {
        remaining_deposit = remaining_deposit
            .checked_sub(fee)
            .ok_or(CrazySolError::Overflow)?;

        invoke(
            &transfer(
                ctx.accounts.owner.key,
                ctx.accounts.big_pharma.key,
                fee,
            ),
            &[
                ctx.accounts.owner.to_account_info(),
                ctx.accounts.big_pharma.to_account_info(),
            ],
        )?;

        experiment_state.total_dev_fees_collected = experiment_state
            .total_dev_fees_collected
            .checked_add(fee)
            .ok_or(CrazySolError::Overflow)?;
    }

    let mut current_recruiter = scientist_state.recruiter;
    let remaining_accounts = &ctx.remaining_accounts;
    let mut account_index = 0;
    
    for percentage in RESEARCH_REWARD_PERCENTAGES.iter() {
        let research_reward = remaining_deposit
            .checked_mul(*percentage as u64)
            .ok_or(CrazySolError::Overflow)?
            .checked_div(100)
            .ok_or(CrazySolError::Overflow)?;
        
        if research_reward == 0 {
            continue;
        }

        if let Some(recruiter_pubkey) = current_recruiter {
            let (expected_pda, _) = Pubkey::find_program_address(
                &[ScientistState::SEED, recruiter_pubkey.as_ref()],
                ctx.program_id,
            );

            require!(
                account_index + 1 < remaining_accounts.len(),
                CrazySolError::MissingAccount
            );

            let recruiter_pda_account = &remaining_accounts[account_index];
            account_index += 1;

            require!(
                recruiter_pda_account.key() == expected_pda,
                CrazySolError::InvalidRecruiter
            );

            let recruiter_wallet = &remaining_accounts[account_index];
            account_index += 1;

            require!(
                recruiter_wallet.key() == recruiter_pubkey,
                CrazySolError::InvalidRecruiter
            );

            invoke(
                &transfer(
                    ctx.accounts.owner.key,
                    recruiter_wallet.key,
                    research_reward,
                ),
                &[
                    ctx.accounts.owner.to_account_info(),
                    recruiter_wallet.to_account_info(),
                ],
            )?;

            {
                let mut recruiter_data = recruiter_pda_account.try_borrow_mut_data()?;
                let mut recruiter_state = ScientistState::try_deserialize(&mut recruiter_data.as_ref())?;

                require!(
                    recruiter_state.owner == recruiter_pubkey,
                    CrazySolError::InvalidRecruiter
                );

                recruiter_state.earned_sol = recruiter_state.earned_sol
                    .checked_add(research_reward)
                    .ok_or(CrazySolError::Overflow)?;
                recruiter_state.earned_sol_from_research = recruiter_state.earned_sol_from_research
                    .checked_add(research_reward)
                    .ok_or(CrazySolError::Overflow)?;

                current_recruiter = recruiter_state.recruiter;

                recruiter_state.try_serialize(&mut recruiter_data.as_mut())?;
            }
        } else {
            invoke(
                &transfer(
                    ctx.accounts.owner.key,
                    ctx.accounts.big_pharma.key,
                    research_reward,
                ),
                &[
                    ctx.accounts.owner.to_account_info(),
                    ctx.accounts.big_pharma.to_account_info(),
                ],
            )?;
            current_recruiter = None;
        }

        experiment_state.total_research_fees_collected = experiment_state
            .total_research_fees_collected
            .checked_add(research_reward)
            .ok_or(CrazySolError::Overflow)?;
    }

    let total_research_percentage: u64 = RESEARCH_REWARD_PERCENTAGES
        .iter()
        .map(|&p| p as u64)
        .sum();

    let total_research_rewards = remaining_deposit
        .checked_mul(total_research_percentage)
        .ok_or(CrazySolError::Overflow)?
        .checked_div(100)
        .ok_or(CrazySolError::Overflow)?;

    remaining_deposit = remaining_deposit
        .checked_sub(total_research_rewards)
        .ok_or(CrazySolError::Overflow)?;

    if remaining_deposit > 0 {
        invoke(
            &transfer(
                ctx.accounts.owner.key,
                ctx.accounts.external_reactor.key,
                remaining_deposit,
            ),
            &[
                ctx.accounts.owner.to_account_info(),
                ctx.accounts.external_reactor.to_account_info(),
            ],
        )?;
    }

    let new_pill = compute_rate_of_centrifugation(
        reactor_state.pill_supply, 
        remaining_deposit.into(),
        laboratory_state.reaction_formula.bonding_curve_p0,
        laboratory_state.reaction_formula.bonding_curve_m
    );
    
    let mut total_new_pill = new_pill;

    if !scientist_state.is_first_injection_done {
        let bonus_pill = new_pill
            .checked_mul(laboratory_state.reaction_formula.first_injection_bonus)
            .ok_or(CrazySolError::Overflow)?
            .checked_div(100)
            .ok_or(CrazySolError::Overflow)?;

        total_new_pill = total_new_pill
            .checked_add(bonus_pill)
            .ok_or(CrazySolError::Overflow)?;
    }

    scientist_state.owned_pill = scientist_state.owned_pill
        .checked_add(total_new_pill)
        .ok_or(CrazySolError::Overflow)?;

    reactor_state.pill_supply = reactor_state.pill_supply
        .checked_add(total_new_pill)
        .ok_or(CrazySolError::Overflow)?;

    scientist_state.sol_injected = scientist_state.sol_injected
        .checked_add(original_deposit)
        .ok_or(CrazySolError::Overflow)?;
    scientist_state.last_distillation_timestamp = current_ts;
    scientist_state.is_first_injection_done = true;

    if !laboratory_state.innoculation_happened {
        let pre_tge_crazy_rewards = if reactor_state.owned_pcrazy >= PCRAZY_INJECTION_REWARDS {
            PCRAZY_INJECTION_REWARDS
        } else {
            reactor_state.owned_pcrazy
        };

    reactor_state.owned_pcrazy = reactor_state
        .owned_pcrazy
        .checked_sub(pre_tge_crazy_rewards)
        .ok_or(CrazySolError::Overflow)?;

    scientist_state.owned_pcrazy = scientist_state
        .owned_pcrazy
        .checked_add(pre_tge_crazy_rewards)
        .ok_or(CrazySolError::Overflow)?;
    }

    experiment_state.total_sol_injected = experiment_state
        .total_sol_injected
        .checked_add(original_deposit)
        .ok_or(CrazySolError::Overflow)?;

    emit!(InjectEvent {
        scientist: ctx.accounts.owner.key(),
        amount: original_deposit,
        new_pill: total_new_pill,
        total_owned_pill: scientist_state.owned_pill,
        total_owned_pcrazy: scientist_state.owned_pcrazy,
        timestamp: current_ts,
    });

    Ok(())
}
