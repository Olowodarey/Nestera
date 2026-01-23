use soroban_sdk::{Address, Env, Vec};
use crate::{DataKey, LockSave, SavingsError, User};

/// Creates a new Lock Save plan for a user
/// 
/// # Arguments
/// * `env` - The contract environment
/// * `user` - The address of the user creating the lock save
/// * `amount` - The amount to lock (must be > 0)
/// * `duration` - The lock duration in seconds (must be > 0)
/// 
/// # Returns
/// * `Result<u64, SavingsError>` - The lock ID on success, or an error
pub fn create_lock_save(
    env: &Env,
    user: Address,
    amount: i128,
    duration: u64,
) -> Result<u64, SavingsError> {
    // Validate inputs
    if amount <= 0 {
        return Err(SavingsError::InvalidAmount);
    }
    
    if duration == 0 {
        return Err(SavingsError::InvalidDuration);
    }
    
    // Ensure user exists
    let user_key = DataKey::User(user.clone());
    if !env.storage().persistent().has(&user_key) {
        return Err(SavingsError::UserNotFound);
    }
    
    // Get next lock ID
    let next_id_key = DataKey::NextLockId;
    let lock_id: u64 = env.storage().persistent().get(&next_id_key).unwrap_or(1);
    
    // Update next lock ID
    env.storage().persistent().set(&next_id_key, &(lock_id + 1));
    
    // Get current timestamp
    let start_time = env.ledger().timestamp();
    let maturity_time = start_time + duration;
    
    // Create LockSave struct
    let lock_save = LockSave {
        id: lock_id,
        owner: user.clone(),
        amount,
        interest_rate: 800, // 8% APY for lock saves
        start_time,
        maturity_time,
        is_withdrawn: false,
    };
    
    // Store the LockSave
    let lock_key = DataKey::LockSave(lock_id);
    env.storage().persistent().set(&lock_key, &lock_save);
    
    // Add lock_id to user's lock saves list
    let user_locks_key = DataKey::UserLockSaves(user.clone());
    let mut user_locks: Vec<u64> = env.storage().persistent().get(&user_locks_key).unwrap_or(Vec::new(env));
    user_locks.push_back(lock_id);
    env.storage().persistent().set(&user_locks_key, &user_locks);
    
    // Update user's total balance and savings count
    let mut user_data: User = env.storage().persistent().get(&user_key).unwrap();
    user_data.total_balance += amount;
    user_data.savings_count += 1;
    env.storage().persistent().set(&user_key, &user_data);
    
    Ok(lock_id)
}

/// Checks if a Lock Save plan has matured
/// 
/// # Arguments
/// * `env` - The contract environment
/// * `lock_id` - The ID of the lock save to check
/// 
/// # Returns
/// * `bool` - True if the lock has matured, false otherwise
pub fn check_matured_lock(env: &Env, lock_id: u64) -> bool {
    let lock_key = DataKey::LockSave(lock_id);
    
    if let Some(lock_save) = env.storage().persistent().get::<DataKey, LockSave>(&lock_key) {
        let current_time = env.ledger().timestamp();
        current_time >= lock_save.maturity_time
    } else {
        false
    }
}

/// Withdraws from a matured Lock Save plan
/// 
/// # Arguments
/// * `env` - The contract environment
/// * `user` - The address of the user withdrawing
/// * `lock_id` - The ID of the lock save to withdraw from
/// 
/// # Returns
/// * `Result<i128, SavingsError>` - The withdrawn amount on success, or an error
pub fn withdraw_lock_save(
    env: &Env,
    user: Address,
    lock_id: u64,
) -> Result<i128, SavingsError> {
    let lock_key = DataKey::LockSave(lock_id);
    
    // Get the lock save
    let mut lock_save: LockSave = env.storage().persistent()
        .get(&lock_key)
        .ok_or(SavingsError::LockNotFound)?;
    
    // Verify ownership
    if lock_save.owner != user {
        return Err(SavingsError::Unauthorized);
    }
    
    // Check if already withdrawn
    if lock_save.is_withdrawn {
        return Err(SavingsError::AlreadyWithdrawn);
    }
    
    // Check if matured
    if !check_matured_lock(env, lock_id) {
        return Err(SavingsError::LockNotMatured);
    }
    
    // Calculate interest (simple interest for demonstration)
    let duration_years = (lock_save.maturity_time - lock_save.start_time) as i128 / (365 * 24 * 60 * 60);
    let interest = (lock_save.amount * lock_save.interest_rate as i128 * duration_years) / 10000;
    let total_amount = lock_save.amount + interest;
    
    // Mark as withdrawn
    lock_save.is_withdrawn = true;
    env.storage().persistent().set(&lock_key, &lock_save);
    
    // Update user's total balance
    let user_key = DataKey::User(user.clone());
    let mut user_data: User = env.storage().persistent().get(&user_key).unwrap();
    user_data.total_balance -= lock_save.amount; // Remove original amount from locked balance
    env.storage().persistent().set(&user_key, &user_data);
    
    Ok(total_amount)
}

/// Gets a Lock Save plan by ID
/// 
/// # Arguments
/// * `env` - The contract environment
/// * `lock_id` - The ID of the lock save to retrieve
/// 
/// # Returns
/// * `Option<LockSave>` - The lock save if found, None otherwise
pub fn get_lock_save(env: &Env, lock_id: u64) -> Option<LockSave> {
    let lock_key = DataKey::LockSave(lock_id);
    env.storage().persistent().get(&lock_key)
}

/// Gets all Lock Save IDs for a user
/// 
/// # Arguments
/// * `env` - The contract environment
/// * `user` - The address of the user
/// 
/// # Returns
/// * `Vec<u64>` - Vector of lock save IDs owned by the user
pub fn get_user_lock_saves(env: &Env, user: Address) -> Vec<u64> {
    let user_locks_key = DataKey::UserLockSaves(user);
    env.storage().persistent().get(&user_locks_key).unwrap_or(Vec::new(env))
}