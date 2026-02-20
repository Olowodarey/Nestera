use super::storage_types::{RewardsDataKey, UserRewards};
use crate::errors::SavingsError;
use soroban_sdk::{Address, Env};

/// Fetches user rewards or returns a default empty state
pub fn get_user_rewards(env: &Env, user: Address) -> UserRewards {
    let key = RewardsDataKey::UserLedger(user);

    // Automatically extend TTL on read to prevent data expiry
    if let Some(rewards) = env
        .storage()
        .persistent()
        .get::<RewardsDataKey, UserRewards>(&key)
    {
        env.storage().persistent().extend_ttl(&key, 17280, 17280); // ~1 day extension
        rewards
    } else {
        UserRewards {
            total_points: 0,
            lifetime_deposited: 0,
            current_streak: 0,
            last_action_timestamp: 0,
        }
    }
}

/// Force-saves the user rewards state
pub fn save_user_rewards(env: &Env, user: Address, rewards: &UserRewards) {
    let key = RewardsDataKey::UserLedger(user);
    env.storage().persistent().set(&key, rewards);
    env.storage().persistent().extend_ttl(&key, 17280, 17280);
}

pub fn initialize_user_rewards(env: &Env, user: Address) -> Result<(), SavingsError> {
    let key = RewardsDataKey::UserLedger(user.clone());

    if env.storage().persistent().has(&key) {
        return Err(SavingsError::UserAlreadyExists);
    }

    let initial_rewards = UserRewards {
        total_points: 0,
        lifetime_deposited: 0,
        current_streak: 0,
        last_action_timestamp: env.ledger().timestamp(),
    };

    // Now this function can find save_user_rewards because they are in the same file
    save_user_rewards(env, user, &initial_rewards);
    Ok(())
}

/// Increases user points with overflow protection
pub fn add_points(env: &Env, user: Address, points: u128) -> Result<(), SavingsError> {
    let mut rewards = get_user_rewards(env, user.clone());

    // Safety check for overflow
    rewards.total_points = rewards
        .total_points
        .checked_add(points)
        .ok_or(SavingsError::Overflow)?;

    save_user_rewards(env, user, &rewards);
    Ok(())
}

/// Resets the streak back to zero
pub fn reset_streak(env: &Env, user: Address) {
    let mut rewards = get_user_rewards(env, user.clone());
    rewards.current_streak = 0;
    save_user_rewards(env, user, &rewards);
}
