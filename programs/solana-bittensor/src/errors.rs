use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Not enough balance to pay registration fee.")]
    NotEnoughBalance,
    #[msg("Not enough stake to withdraw.")]
    NotEnoughStake,
    #[msg("Validator is exist.")]
    ValidatorExist,
    #[msg("Total weight exceeds MAX_WEIGHT")]
    TotalWeightExceedsMaxWeight,
    #[msg("Validator is not exist.")]
    NotBittensorValidator,
    #[msg("Cant find account at remaining accounts.")]
    CantFindAtRemainingAccounts,
    #[msg("Validator not enough bounds.")]
    ValidatorNotEnoughBounds,
    #[msg("No miner can replace.")]
    NoMinerCanReplace,
    #[msg("No validator can replace.")]
    NoValidatorCanReplace,
    #[msg("No subnet can replace.")]
    NoSubnetCanReplace,
    #[msg("Stake amount too low.")]
    StakeAmountTooLow,
    #[msg("Epoch is ended.")]
    EpochIsEnded,
    #[msg("Invalid end step.")]
    InvalidEndStep,
    #[msg("Invalid miner group id.")]
    InvalidMinerGroupId,
    #[msg("Validator already set weights.")]
    ValidatorAlreadySetWeights,
    #[msg("Miner is not match")]
    MinerNotMatch
}
