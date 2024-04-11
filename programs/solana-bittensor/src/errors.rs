use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Not enough stake to withdraw.")]
    NotEnoughStake,
    #[msg("Validator is exist.")]
    ValidatorExist,
    #[msg("Total weight exceeds MAX_WEIGHT")]
    TotalWeightExceedsMaxWeight,
    #[msg("Validator is not exist.")]
    NotBittensorValidator,
}
