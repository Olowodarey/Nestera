use crate::errors::SavingsError;
// use soroban_sdk::{Address, Env};

/// Validates that an amount is positive.
pub fn assert_non_negative(amount: i128) -> Result<(), SavingsError> {
    if amount <= 0 {
        return Err(SavingsError::InvalidAmount);
    }
    Ok(())
}

/// Validates that fee basis points are within the 0-100% range.
pub fn assert_valid_fee(fee_bps: u32) -> Result<(), SavingsError> {
    if fee_bps > 10_000 {
        // Change this from InvalidAmount to InvalidFeeBps
        return Err(SavingsError::InvalidFeeBps);
    }
    Ok(())
}

/// Validates that a withdrawal does not exceed available balance.
pub fn assert_sufficient_balance(balance: i128, amount: i128) -> Result<(), SavingsError> {
    if balance < amount {
        return Err(SavingsError::InsufficientBalance);
    }
    Ok(())
}

// Checks if the contract is currently paused.
// pub fn assert_not_paused(_env: &Env) -> Result<(), SavingsError> {
//     // This assumes you have a State or Config object in storage
//     // Example: let is_paused = env.storage().instance().get::<_, bool>(&DataKey::Paused).unwrap_or(false);
//     // if is_paused { return Err(SavingsError::ContractPaused); }
//     Ok(())
// }

// /// Checks for potential arithmetic overflow before it happens.
// pub fn check_add(a: i128, b: i128) -> Result<i128, SavingsError> {
//     a.checked_add(b).ok_or(SavingsError::Overflow)
// }
