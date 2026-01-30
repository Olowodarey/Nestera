use crate::errors::SavingsError;
use crate::flexi;
use crate::storage_types::{AutoSave, DataKey};
use crate::users;
use soroban_sdk::{Address, Env, Vec};

/// Creates a new AutoSave schedule for recurring Flexi deposits
///
/// # Arguments
/// * `env` - The contract environment
/// * `user` - The user creating the schedule
/// * `amount` - The amount to deposit on each execution (must be > 0)
/// * `interval_seconds` - How often the schedule runs in seconds (must be > 0)
/// * `start_time` - Unix timestamp for the first execution
///
/// # Returns
/// * `Ok(u64)` - The unique schedule ID
/// * `Err(SavingsError)` - If validation fails
pub fn create_autosave(
    env: &Env,
    user: Address,
    amount: i128,
    interval_seconds: u64,
    start_time: u64,
) -> Result<u64, SavingsError> {
    user.require_auth();

    // Validate amount
    if amount <= 0 {
        return Err(SavingsError::InvalidAmount);
    }

    // Validate interval
    if interval_seconds == 0 {
        return Err(SavingsError::InvalidTimestamp);
    }

    // Ensure user exists
    if !users::user_exists(env, &user) {
        return Err(SavingsError::UserNotFound);
    }

    // Generate unique schedule ID
    let schedule_id = get_next_schedule_id(env);

    // Create the schedule
    let schedule = AutoSave {
        id: schedule_id,
        user: user.clone(),
        amount,
        interval_seconds,
        next_execution_time: start_time,
        is_active: true,
    };

    // Store the schedule
    env.storage()
        .persistent()
        .set(&DataKey::AutoSave(schedule_id), &schedule);

    // Link schedule to user
    add_schedule_to_user(env, &user, schedule_id);

    // Increment the next schedule ID
    increment_next_schedule_id(env);

    Ok(schedule_id)
}

/// Executes an AutoSave schedule if it's due
///
/// # Arguments
/// * `env` - The contract environment
/// * `schedule_id` - The ID of the schedule to execute
///
/// # Returns
/// * `Ok(())` - If execution succeeds
/// * `Err(SavingsError)` - If the schedule is not found, inactive, or not yet due
pub fn execute_autosave(env: &Env, schedule_id: u64) -> Result<(), SavingsError> {
    // Fetch the schedule
    let mut schedule: AutoSave = env
        .storage()
        .persistent()
        .get(&DataKey::AutoSave(schedule_id))
        .ok_or(SavingsError::PlanNotFound)?;

    // Ensure schedule is active
    if !schedule.is_active {
        return Err(SavingsError::InvalidPlanConfig);
    }

    // Ensure current time >= next_execution_time
    let current_time = env.ledger().timestamp();
    if current_time < schedule.next_execution_time {
        return Err(SavingsError::InvalidTimestamp);
    }

    // Perform Flexi deposit
    flexi::flexi_deposit(env.clone(), schedule.user.clone(), schedule.amount)?;

    // Update next execution time
    schedule.next_execution_time += schedule.interval_seconds;

    // Save updated schedule
    env.storage()
        .persistent()
        .set(&DataKey::AutoSave(schedule_id), &schedule);

    Ok(())
}

/// Cancels an AutoSave schedule
///
/// # Arguments
/// * `env` - The contract environment
/// * `user` - The user cancelling the schedule
/// * `schedule_id` - The ID of the schedule to cancel
///
/// # Returns
/// * `Ok(())` - If cancellation succeeds
/// * `Err(SavingsError)` - If the schedule is not found or user is not the owner
pub fn cancel_autosave(env: &Env, user: Address, schedule_id: u64) -> Result<(), SavingsError> {
    user.require_auth();

    // Fetch the schedule
    let mut schedule: AutoSave = env
        .storage()
        .persistent()
        .get(&DataKey::AutoSave(schedule_id))
        .ok_or(SavingsError::PlanNotFound)?;

    // Ensure caller owns the schedule
    if schedule.user != user {
        return Err(SavingsError::Unauthorized);
    }

    // Deactivate the schedule
    schedule.is_active = false;

    // Save updated schedule
    env.storage()
        .persistent()
        .set(&DataKey::AutoSave(schedule_id), &schedule);

    Ok(())
}

/// Gets an AutoSave schedule by ID
pub fn get_autosave(env: &Env, schedule_id: u64) -> Option<AutoSave> {
    env.storage()
        .persistent()
        .get(&DataKey::AutoSave(schedule_id))
}

/// Gets all AutoSave schedule IDs for a user
pub fn get_user_autosaves(env: &Env, user: &Address) -> Vec<u64> {
    env.storage()
        .persistent()
        .get(&DataKey::UserAutoSaves(user.clone()))
        .unwrap_or(Vec::new(env))
}

// ========== Helper Functions ==========

fn get_next_schedule_id(env: &Env) -> u64 {
    env.storage()
        .persistent()
        .get(&DataKey::NextAutoSaveId)
        .unwrap_or(1)
}

fn increment_next_schedule_id(env: &Env) {
    let current_id = get_next_schedule_id(env);
    env.storage()
        .persistent()
        .set(&DataKey::NextAutoSaveId, &(current_id + 1));
}

fn add_schedule_to_user(env: &Env, user: &Address, schedule_id: u64) {
    let key = DataKey::UserAutoSaves(user.clone());
    let mut schedules: Vec<u64> = env
        .storage()
        .persistent()
        .get(&key)
        .unwrap_or(Vec::new(env));

    schedules.push_back(schedule_id);
    env.storage().persistent().set(&key, &schedules);
}
