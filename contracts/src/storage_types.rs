use soroban_sdk::{contracttype, contracterror, Address, Symbol};

/// Represents the different types of savings plans available in Nestera
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanType {
    Flexi,
    Lock(u64),
    Goal(Symbol, i128, u32),
    Group(u64, bool, u32, i128),
}

/// Represents an individual savings plan for a user
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SavingsPlan {
    pub plan_id: u64,
    pub plan_type: PlanType,
    pub balance: i128,
    pub start_time: u64,
    pub last_deposit: u64,
    pub last_withdraw: u64,
    /// Annual Percentage Yield (APY) as an integer (e.g., 500 = 5.00%)
    pub interest_rate: u32,
    pub is_completed: bool,
}

/// Represents a user's aggregated savings information
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    pub total_balance: i128,
    pub savings_count: u32,
}

/// Represents a Lock Save plan with fixed duration
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LockSave {
    pub id: u64,
    pub owner: Address,
    pub amount: i128,
    pub interest_rate: u32,
    pub start_time: u64,
    pub maturity_time: u64,
    pub is_withdrawn: bool,
}

/// Custom error types for the savings contract
#[contracterror]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SavingsError {
    InvalidAmount = 1,
    InvalidDuration = 2,
    UserNotFound = 3,
    LockNotFound = 4,
    LockNotMatured = 5,
    AlreadyWithdrawn = 6,
    Unauthorized = 7,
}

/// Storage keys for the contract's persistent data
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
    User(Address),
    /// Maps a (user address, plan_id) tuple to a SavingsPlan
    SavingsPlan(Address, u64),
    /// Maps lock plan ID to LockSave struct
    LockSave(u64),
    /// Maps user to a list of their LockSave IDs
    UserLockSaves(Address),
    /// Stores the next auto-incrementing LockSave ID
    NextLockId,
}
