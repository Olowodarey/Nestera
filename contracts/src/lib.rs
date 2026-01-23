#![no_std]
#![allow(non_snake_case)]
use soroban_sdk::{contract, contractimpl, Address, Env, Vec};

mod storage_types;
pub use storage_types::*;

mod lock;
pub use lock::*;

#[contract]
pub struct NesteraContract;

#[contractimpl]
impl NesteraContract {
    /// Initialize a new user in the system
    pub fn init_user(env: Env, user: Address) -> User {
        user.require_auth();
        
        let user_data = User {
            total_balance: 0,
            savings_count: 0,
        };
        
        let user_key = DataKey::User(user);
        env.storage().persistent().set(&user_key, &user_data);
        
        user_data
    }
    
    /// Create a new Lock Save plan
    pub fn create_lock_save(
        env: Env,
        user: Address,
        amount: i128,
        duration: u64,
    ) -> Result<u64, SavingsError> {
        user.require_auth();
        lock::create_lock_save(&env, user, amount, duration)
    }
    
    /// Check if a Lock Save plan has matured
    pub fn check_matured_lock(env: Env, lock_id: u64) -> bool {
        lock::check_matured_lock(&env, lock_id)
    }
    
    /// Withdraw from a matured Lock Save plan
    pub fn withdraw_lock_save(
        env: Env,
        user: Address,
        lock_id: u64,
    ) -> Result<i128, SavingsError> {
        user.require_auth();
        lock::withdraw_lock_save(&env, user, lock_id)
    }
    
    /// Get a Lock Save plan by ID
    pub fn get_lock_save(env: Env, lock_id: u64) -> Option<LockSave> {
        lock::get_lock_save(&env, lock_id)
    }
    
    /// Get all Lock Save IDs for a user
    pub fn get_user_lock_saves(env: Env, user: Address) -> Vec<u64> {
        lock::get_user_lock_saves(&env, user)
    }
    
    /// Get user information
    pub fn get_user(env: Env, user: Address) -> Option<User> {
        let user_key = DataKey::User(user);
        env.storage().persistent().get(&user_key)
    }
}

mod test;
