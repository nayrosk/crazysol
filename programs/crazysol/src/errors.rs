use anchor_lang::prelude::*;

#[error_code]
pub enum CrazySolError {
    #[msg("Laboratory already initialized")]
    AlreadyInitialized,
    #[msg("Injection amount too small, minimum is 0.05 SOL")]
    InjectionTooSmall,
    #[msg("Cooldown period is still active")]
    CooldownActive,
    #[msg("Laboratory currently paused")]
    CurrentlyPaused,
    #[msg("Not enough pCRAZY available")]
    InsufficientpCRAZY,
    #[msg("Invalid fee percentage")]
    InvalidFeePercentage,
    #[msg("Invalid Pubkey")]
    InvalidPubkey,
    #[msg("Invalid recruiter")]
    InvalidRecruiter,
    #[msg("Invalid scientist address")]
    InvalidScientistAddress,
    #[msg("Invalid Scientist")]
    InvalidScientist,
    #[msg("Missing account")]
    MissingAccount,
    #[msg("Nothing to distill or mutate")]
    NoYield,
    #[msg("Overflow")]
    Overflow,
    #[msg("Innoculation already happened")]
    InnoculationAlreadyHappened,
    #[msg("Unauthorized action")]
    Unauthorized,
    #[msg("Username too long")]
    UsernameTooLong
}
