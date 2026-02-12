use anchor_lang::prelude::*;
use crate::{constants::{PCRAZY_RECRUITMENT_REWARDS}, errors::CrazySolError, events::RegisterScientistEvent, require_initialized, require_operational, states::{ExperimentState, LaboratoryState, ReactorState, ScientistState}};

#[derive(Accounts)]
pub struct RegisterScientist<'info> {
    #[account(
        seeds = [LaboratoryState::SEED],
        bump,
    )]
    pub laboratory_state: Account<'info, LaboratoryState>,

    #[account(
        init,
        payer = scientist,
        space = ScientistState::SIZE,
        seeds = [ScientistState::SEED, scientist.key().as_ref()],
        bump
    )]
    pub scientist_state: Account<'info, ScientistState>,

    #[account(
        mut,
        seeds = [ExperimentState::SEED],
        bump,
    )]
    pub experiment_state: Account<'info, ExperimentState>,

    #[account(
        mut,
        seeds = [ReactorState::SEED],
        bump,
    )]
    pub reactor_state: Account<'info, ReactorState>,

    /// CHECK: This account is validated manually in the instruction
    #[account(mut)]
    pub recruiter_account: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub scientist: Signer<'info>,
}

pub fn handle_register_scientist<'info>(
    ctx: Context<'_, '_, '_, 'info, RegisterScientist<'info>>,
    username: String,
    recruiter: Option<Pubkey>,
) -> Result<()> {
    let laboratory_state = &ctx.accounts.laboratory_state;
    let experiment_state = &mut ctx.accounts.experiment_state;
    let scientist_state = &mut ctx.accounts.scientist_state;
    let reactor_state = &mut ctx.accounts.reactor_state;
    let remaining_accounts = &ctx.remaining_accounts;
    let mut recruiter_reward: Option<u64> = None;
    let mut account_index = 0;

    require_initialized(laboratory_state)?;
    require_operational(laboratory_state)?;

    if let Some(recruiter_key) = recruiter {
        require!(
            recruiter_key != ctx.accounts.scientist.key(),
            CrazySolError::InvalidRecruiter
        );

        let (expected_pda, _) = Pubkey::find_program_address(
            &[ScientistState::SEED, recruiter_key.as_ref()],
            ctx.program_id
        );

        require!(
            ctx.accounts.recruiter_account.key() == expected_pda,
            CrazySolError::InvalidRecruiter
        );

        require!(
            !ctx.accounts.recruiter_account.data_is_empty(),
            CrazySolError::InvalidRecruiter
        );

        let mut recruiter_data = ctx.accounts.recruiter_account.try_borrow_mut_data()?;

        let mut recruiter_state = ScientistState::try_deserialize(&mut recruiter_data.as_ref())
            .map_err(|_| CrazySolError::InvalidRecruiter)?;

        require!(
            recruiter_state.owner == recruiter_key,
            CrazySolError::InvalidRecruiter
        );

        recruiter_state.test_subjects_count = recruiter_state.test_subjects_count
            .checked_add(1)
            .ok_or(CrazySolError::Overflow)?;

        if !laboratory_state.innoculation_happened {
            if reactor_state.owned_pcrazy >= PCRAZY_RECRUITMENT_REWARDS {
                reactor_state.owned_pcrazy = reactor_state.owned_pcrazy
                    .checked_sub(PCRAZY_RECRUITMENT_REWARDS)
                    .ok_or(CrazySolError::Overflow)?;

                recruiter_state.owned_pcrazy = recruiter_state.owned_pcrazy
                    .checked_add(PCRAZY_RECRUITMENT_REWARDS)
                    .ok_or(CrazySolError::Overflow)?;

                recruiter_reward = Some(PCRAZY_RECRUITMENT_REWARDS);
            }
        }

        let mut current_recruiter = recruiter_state.recruiter;

        recruiter_state.try_serialize(&mut recruiter_data.as_mut())?;

        drop(recruiter_data);

        if let Some(specimen_key) = current_recruiter {
            let (expected_specimen_pda, _) = Pubkey::find_program_address(
                &[ScientistState::SEED, specimen_key.as_ref()],
                ctx.program_id
            );

            require!(
                account_index < remaining_accounts.len(),
                CrazySolError::MissingAccount
            );

            let specimen_account = &remaining_accounts[account_index];
            account_index += 1;

            require!(
                specimen_account.key() == expected_specimen_pda,
                CrazySolError::InvalidRecruiter
            );

            {
                let mut specimen_data = specimen_account.try_borrow_mut_data()?;
                let mut specimen_state = ScientistState::try_deserialize(&mut specimen_data.as_ref())
                    .map_err(|_| CrazySolError::InvalidRecruiter)?;

                require!(
                    specimen_state.owner == specimen_key,
                    CrazySolError::InvalidRecruiter
                );

                specimen_state.specimens_count = specimen_state.specimens_count
                    .checked_add(1)
                    .ok_or(CrazySolError::Overflow)?;

                current_recruiter = specimen_state.recruiter;

                specimen_state.try_serialize(&mut specimen_data.as_mut())?;
            }

            if let Some(sample_key) = current_recruiter {
                let (expected_sample_pda, _) = Pubkey::find_program_address(
                    &[ScientistState::SEED, sample_key.as_ref()],
                    ctx.program_id
                );

                require!(
                    account_index < remaining_accounts.len(),
                    CrazySolError::MissingAccount
                );

                let sample_account = &remaining_accounts[account_index];

                require!(
                    sample_account.key() == expected_sample_pda,
                    CrazySolError::InvalidRecruiter
                );

                {
                    let mut sample_data = sample_account.try_borrow_mut_data()?;
                    let mut sample_state = ScientistState::try_deserialize(&mut sample_data.as_ref())
                        .map_err(|_| CrazySolError::InvalidRecruiter)?;

                    require!(
                        sample_state.owner == sample_key,
                        CrazySolError::InvalidRecruiter
                    );


                    sample_state.samples_count = sample_state.samples_count
                        .checked_add(1)
                        .ok_or(CrazySolError::Overflow)?;

                    sample_state.try_serialize(&mut sample_data.as_mut())?;
                }
            }
        }

        experiment_state.total_scientists_recruited = experiment_state.total_scientists_recruited
            .checked_add(1)
            .ok_or(CrazySolError::Overflow)?;
    }

    if username.len() > ScientistState::MAX_USERNAME_LENGTH {
        return Err(CrazySolError::UsernameTooLong.into());
    }

    scientist_state.owner = ctx.accounts.scientist.key();
    scientist_state.username = username;
    scientist_state.recruiter = recruiter;
    scientist_state.owned_pill = 0;
    scientist_state.owned_pcrazy = 0;
    scientist_state.is_first_injection_done = false;
    scientist_state.is_first_mutation_done = false;
    scientist_state.sol_injected = 0;
    scientist_state.last_streak_timestamp = 0;
    scientist_state.current_streak = 0;
    scientist_state.last_distillation_timestamp = 0;
    scientist_state.distillable_yield = 0;
    scientist_state.test_subjects_count = 0;
    scientist_state.specimens_count = 0;
    scientist_state.samples_count = 0;
    scientist_state.earned_sol = 0;
    scientist_state.earned_sol_from_research = 0;

    experiment_state.total_scientists = experiment_state.total_scientists
        .checked_add(1)
        .ok_or(CrazySolError::Overflow)?;

    emit!(RegisterScientistEvent {
        scientist: ctx.accounts.scientist.key(),
        recruiter: scientist_state.recruiter,
        recruiter_reward,
    });

    Ok(())
}
