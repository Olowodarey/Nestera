use crate::errors::SavingsError;
use soroban_sdk::{contracttype, Address, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RewardsConfig {
    pub points_per_token: u32,      // e.g., 10 points per 1 token
    pub streak_bonus_bps: u32,      // Bonus for consistent saving
    pub long_lock_bonus_bps: u32,   // Bonus for long-term locks
    pub goal_completion_bonus: u32, // Flat points for finishing a goal
    pub enabled: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserRewards {
    pub total_points: u128,         // Total spendable/accumulated points
    pub lifetime_deposited: i128,   // Total volume for tier calculation
    pub current_streak: u32,        // Number of consecutive periods saved
    pub last_action_timestamp: u64, // To check if streak is broken
}

#[contracttype]
pub enum RewardsDataKey {
    Config,
    UserLedger(Address),
}
