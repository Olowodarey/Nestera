#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::{Address as _, Ledger as _}, Address, Env};

// Existing tests for basic types
#[test]
fn test_user_instantiation() {
    let user = User {
        total_balance: 1_000_000,
        savings_count: 3,
    };

    assert_eq!(user.total_balance, 1_000_000);
    assert_eq!(user.savings_count, 3);
}

#[test]
fn test_flexi_savings_plan() {
    let plan = SavingsPlan {
        plan_id: 1,
        plan_type: PlanType::Flexi,
        balance: 500_000,
        start_time: 1000000,
        last_deposit: 1000100,
        last_withdraw: 0,
        interest_rate: 500, // 5.00% APY
        is_completed: false,
    };

    assert_eq!(plan.plan_id, 1);
    assert_eq!(plan.plan_type, PlanType::Flexi);
    assert_eq!(plan.balance, 500_000);
    assert!(!plan.is_completed);
}

#[test]
fn test_lock_savings_plan() {
    let locked_until = 2000000;
    let plan = SavingsPlan {
        plan_id: 2,
        plan_type: PlanType::Lock(locked_until),
        balance: 1_000_000,
        start_time: 1000000,
        last_deposit: 1000000,
        last_withdraw: 0,
        interest_rate: 800,
        is_completed: false,
    };

    assert_eq!(plan.plan_id, 2);
    match plan.plan_type {
        PlanType::Lock(until) => assert_eq!(until, locked_until),
        _ => panic!("Expected Lock plan type"),
    }
}

#[test]
fn test_goal_savings_plan() {
    let plan = SavingsPlan {
        plan_id: 3,
        plan_type: PlanType::Goal(
            symbol_short!("education"),
            5_000_000,
            1u32, // e.g. 1 = weekly
        ),
        balance: 2_000_000,
        start_time: 1000000,
        last_deposit: 1500000,
        last_withdraw: 0,
        interest_rate: 600,
        is_completed: false,
    };

    assert_eq!(plan.plan_id, 3);
    match plan.plan_type {
        PlanType::Goal(category, target_amount, contribution_type) => {
            assert_eq!(category, symbol_short!("education"));
            assert_eq!(target_amount, 5_000_000);
            assert_eq!(contribution_type, 1u32);
        }
        _ => panic!("Expected Goal plan type"),
    }
}

#[test]
fn test_group_savings_plan() {
    let plan = SavingsPlan {
        plan_id: 4,
        plan_type: PlanType::Group(101, true, 2u32, 10_000_000),
        balance: 3_000_000,
        start_time: 1000000,
        last_deposit: 1600000,
        last_withdraw: 0,
        interest_rate: 700,
        is_completed: false,
    };

    assert_eq!(plan.plan_id, 4);
    match plan.plan_type {
        PlanType::Group(group_id, is_public, contribution_type, target_amount) => {
            assert_eq!(group_id, 101);
            assert!(is_public);
            assert_eq!(contribution_type, 2u32);
            assert_eq!(target_amount, 10_000_000);
        }
        _ => panic!("Expected Group plan type"),
    }
}

#[test]
fn test_data_key_admin() {
    let key = DataKey::Admin;
    assert_eq!(key, DataKey::Admin);
}

#[test]
fn test_data_key_user() {
    let env = Env::default();
    let user_address = Address::generate(&env);
    let key = DataKey::User(user_address.clone());

    match key {
        DataKey::User(addr) => assert_eq!(addr, user_address),
        _ => panic!("Expected User data key"),
    }
}

#[test]
fn test_data_key_savings_plan() {
    let env = Env::default();
    let user_address = Address::generate(&env);
    let plan_id = 42;
    let key = DataKey::SavingsPlan(user_address.clone(), plan_id);

    match key {
        DataKey::SavingsPlan(addr, id) => {
            assert_eq!(addr, user_address);
            assert_eq!(id, plan_id);
        }
        _ => panic!("Expected SavingsPlan data key"),
    }
}

#[test]
fn test_xdr_compatibility_user() {
    let env = Env::default();
    let contract_id = env.register(NesteraContract, ());

    let user = User {
        total_balance: 1_500_000,
        savings_count: 5,
    };

    let key = symbol_short!("testuser");
    env.as_contract(&contract_id, || {
        env.storage().instance().set(&key, &user);
        let retrieved_user: User = env.storage().instance().get(&key).unwrap();
        assert_eq!(user, retrieved_user);
    });
}

#[test]
fn test_xdr_compatibility_savings_plan() {
    let env = Env::default();
    let contract_id = env.register(NesteraContract, ());

    let plan = SavingsPlan {
        plan_id: 1,
        plan_type: PlanType::Flexi,
        balance: 750_000,
        start_time: 1000000,
        last_deposit: 1100000,
        last_withdraw: 1050000,
        interest_rate: 550,
        is_completed: false,
    };

    let key = symbol_short!("testplan");
    env.as_contract(&contract_id, || {
        env.storage().instance().set(&key, &plan);
        let retrieved_plan: SavingsPlan = env.storage().instance().get(&key).unwrap();
        assert_eq!(plan, retrieved_plan);
    });
}

#[test]
fn test_xdr_compatibility_all_plan_types() {
    let env = Env::default();
    let contract_id = env.register(NesteraContract, ());

    env.as_contract(&contract_id, || {
        // Test Flexi
        let flexi_plan = SavingsPlan {
            plan_id: 0,
            plan_type: PlanType::Flexi,
            balance: 1_000_000,
            start_time: 1000000,
            last_deposit: 1100000,
            last_withdraw: 0,
            interest_rate: 500,
            is_completed: false,
        };
        env.storage().instance().set(&0u32, &flexi_plan);
        let retrieved: SavingsPlan = env.storage().instance().get(&0u32).unwrap();
        assert_eq!(flexi_plan, retrieved);

        // Test Lock
        let lock_plan = SavingsPlan {
            plan_id: 1,
            plan_type: PlanType::Lock(2000000),
            balance: 1_000_000,
            start_time: 1000000,
            last_deposit: 1100000,
            last_withdraw: 0,
            interest_rate: 500,
            is_completed: false,
        };
        env.storage().instance().set(&1u32, &lock_plan);
        let retrieved: SavingsPlan = env.storage().instance().get(&1u32).unwrap();
        assert_eq!(lock_plan, retrieved);

        // Test Goal
        let goal_plan = SavingsPlan {
            plan_id: 2,
            plan_type: PlanType::Goal(symbol_short!("vacation"), 3_000_000, 1u32),
            balance: 1_000_000,
            start_time: 1000000,
            last_deposit: 1100000,
            last_withdraw: 0,
            interest_rate: 500,
            is_completed: false,
        };
        env.storage().instance().set(&2u32, &goal_plan);
        let retrieved: SavingsPlan = env.storage().instance().get(&2u32).unwrap();
        assert_eq!(goal_plan, retrieved);

        // Test Group
        let group_plan = SavingsPlan {
            plan_id: 3,
            plan_type: PlanType::Group(200, false, 3u32, 8_000_000),
            balance: 1_000_000,
            start_time: 1000000,
            last_deposit: 1100000,
            last_withdraw: 0,
            interest_rate: 500,
            is_completed: false,
        };
        env.storage().instance().set(&3u32, &group_plan);
        let retrieved: SavingsPlan = env.storage().instance().get(&3u32).unwrap();
        assert_eq!(group_plan, retrieved);
    });
}

#[test]
fn test_completed_plan() {
    let plan = SavingsPlan {
        plan_id: 5,
        plan_type: PlanType::Goal(symbol_short!("house"), 10_000_000, 2u32),
        balance: 10_000_000,
        start_time: 1000000,
        last_deposit: 2000000,
        last_withdraw: 0,
        interest_rate: 650,
        is_completed: true,
    };

    assert!(plan.is_completed);
    assert_eq!(plan.balance, 10_000_000);
}

#[test]
fn test_plan_type_patterns() {
    // Test that we can extract values from each plan type variant
    let lock_plan = PlanType::Lock(1234567);
    if let PlanType::Lock(timestamp) = lock_plan {
        assert_eq!(timestamp, 1234567);
    }

    let goal_plan = PlanType::Goal(symbol_short!("car"), 2_000_000, 3u32);
    if let PlanType::Goal(cat, amount, contrib) = goal_plan {
        assert_eq!(cat, symbol_short!("car"));
        assert_eq!(amount, 2_000_000);
        assert_eq!(contrib, 3u32);
    }

    let group_plan = PlanType::Group(999, true, 1u32, 5_000_000);
    if let PlanType::Group(id, public, contrib, amount) = group_plan {
        assert_eq!(id, 999);
        assert!(public);
        assert_eq!(contrib, 1u32);
        assert_eq!(amount, 5_000_000);
    }
}

// New comprehensive Lock Save tests
#[test]
fn test_lock_save_struct() {
    let env = Env::default();
    let user = Address::generate(&env);
    
    let lock_save = LockSave {
        id: 1,
        owner: user.clone(),
        amount: 1_000_000,
        interest_rate: 800,
        start_time: 1000000,
        maturity_time: 1000000 + (30 * 24 * 60 * 60), // 30 days
        is_withdrawn: false,
    };
    
    assert_eq!(lock_save.id, 1);
    assert_eq!(lock_save.owner, user);
    assert_eq!(lock_save.amount, 1_000_000);
    assert_eq!(lock_save.interest_rate, 800);
    assert!(!lock_save.is_withdrawn);
}

#[test]
fn test_create_lock_save_success() {
    let env = Env::default();
    let contract_id = env.register(NesteraContract, ());
    let user = Address::generate(&env);
    
    env.mock_all_auths();
    
    // Initialize user first
    let _user_data = NesteraContract::init_user(env.clone(), user.clone());
    
    // Create lock save
    let result = NesteraContract::create_lock_save(
        env.clone(),
        user.clone(),
        1_000_000,
        30 * 24 * 60 * 60, // 30 days
    );
    
    assert!(result.is_ok());
    let lock_id = result.unwrap();
    assert_eq!(lock_id, 1);
    
    // Verify lock save was created
    let lock_save = NesteraContract::get_lock_save(env.clone(), lock_id);
    assert!(lock_save.is_some());
    
    let lock = lock_save.unwrap();
    assert_eq!(lock.id, lock_id);
    assert_eq!(lock.owner, user);
    assert_eq!(lock.amount, 1_000_000);
    assert_eq!(lock.interest_rate, 800);
    assert!(!lock.is_withdrawn);
}

#[test]
fn test_create_lock_save_invalid_amount() {
    let env = Env::default();
    let contract_id = env.register(NesteraContract, ());
    let user = Address::generate(&env);
    
    env.mock_all_auths();
    
    // Initialize user first
    let _user_data = NesteraContract::init_user(env.clone(), user.clone());
    
    // Try to create lock save with invalid amount
    let result = NesteraContract::create_lock_save(
        env.clone(),
        user.clone(),
        0, // Invalid amount
        30 * 24 * 60 * 60,
    );
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), SavingsError::InvalidAmount);
}

#[test]
fn test_create_lock_save_invalid_duration() {
    let env = Env::default();
    let contract_id = env.register(NesteraContract, ());
    let user = Address::generate(&env);
    
    env.mock_all_auths();
    
    // Initialize user first
    let _user_data = NesteraContract::init_user(env.clone(), user.clone());
    
    // Try to create lock save with invalid duration
    let result = NesteraContract::create_lock_save(
        env.clone(),
        user.clone(),
        1_000_000,
        0, // Invalid duration
    );
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), SavingsError::InvalidDuration);
}

#[test]
fn test_create_lock_save_user_not_found() {
    let env = Env::default();
    let contract_id = env.register(NesteraContract, ());
    let user = Address::generate(&env);
    
    env.mock_all_auths();
    
    // Try to create lock save without initializing user
    let result = NesteraContract::create_lock_save(
        env.clone(),
        user.clone(),
        1_000_000,
        30 * 24 * 60 * 60,
    );
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), SavingsError::UserNotFound);
}

#[test]
fn test_check_matured_lock_not_matured() {
    let env = Env::default();
    let contract_id = env.register(NesteraContract, ());
    let user = Address::generate(&env);
    
    env.mock_all_auths();
    
    // Initialize user and create lock save
    let _user_data = NesteraContract::init_user(env.clone(), user.clone());
    let lock_id = NesteraContract::create_lock_save(
        env.clone(),
        user.clone(),
        1_000_000,
        30 * 24 * 60 * 60, // 30 days
    ).unwrap();
    
    // Check maturation (should not be matured yet)
    let is_matured = NesteraContract::check_matured_lock(env.clone(), lock_id);
    assert!(!is_matured);
}

#[test]
fn test_check_matured_lock_matured() {
    let env = Env::default();
    let contract_id = env.register(NesteraContract, ());
    let user = Address::generate(&env);
    
    env.mock_all_auths();
    
    // Initialize user and create lock save with very short duration
    let _user_data = NesteraContract::init_user(env.clone(), user.clone());
    let lock_id = NesteraContract::create_lock_save(
        env.clone(),
        user.clone(),
        1_000_000,
        1, // 1 second duration
    ).unwrap();
    
    // Advance time
    env.ledger().with_mut(|li| {
        li.timestamp = li.timestamp + 2; // Advance by 2 seconds
    });
    
    // Check maturation (should be matured now)
    let is_matured = NesteraContract::check_matured_lock(env.clone(), lock_id);
    assert!(is_matured);
}

#[test]
fn test_check_matured_lock_nonexistent() {
    let env = Env::default();
    let contract_id = env.register(NesteraContract, ());
    
    // Check maturation for non-existent lock
    let is_matured = NesteraContract::check_matured_lock(env.clone(), 999);
    assert!(!is_matured);
}

#[test]
fn test_multiple_lock_saves_unique_ids() {
    let env = Env::default();
    let contract_id = env.register(NesteraContract, ());
    let user = Address::generate(&env);
    
    env.mock_all_auths();
    
    // Initialize user
    let _user_data = NesteraContract::init_user(env.clone(), user.clone());
    
    // Create multiple lock saves
    let lock_id1 = NesteraContract::create_lock_save(
        env.clone(),
        user.clone(),
        1_000_000,
        30 * 24 * 60 * 60,
    ).unwrap();
    
    let lock_id2 = NesteraContract::create_lock_save(
        env.clone(),
        user.clone(),
        2_000_000,
        60 * 24 * 60 * 60,
    ).unwrap();
    
    let lock_id3 = NesteraContract::create_lock_save(
        env.clone(),
        user.clone(),
        500_000,
        15 * 24 * 60 * 60,
    ).unwrap();
    
    // Verify unique IDs
    assert_eq!(lock_id1, 1);
    assert_eq!(lock_id2, 2);
    assert_eq!(lock_id3, 3);
    
    // Verify all locks exist and have correct amounts
    let lock1 = NesteraContract::get_lock_save(env.clone(), lock_id1).unwrap();
    let lock2 = NesteraContract::get_lock_save(env.clone(), lock_id2).unwrap();
    let lock3 = NesteraContract::get_lock_save(env.clone(), lock_id3).unwrap();
    
    assert_eq!(lock1.amount, 1_000_000);
    assert_eq!(lock2.amount, 2_000_000);
    assert_eq!(lock3.amount, 500_000);
}

#[test]
fn test_user_lock_saves_tracking() {
    let env = Env::default();
    let contract_id = env.register(NesteraContract, ());
    let user = Address::generate(&env);
    
    env.mock_all_auths();
    
    // Initialize user
    let _user_data = NesteraContract::init_user(env.clone(), user.clone());
    
    // Create multiple lock saves
    let lock_id1 = NesteraContract::create_lock_save(
        env.clone(),
        user.clone(),
        1_000_000,
        30 * 24 * 60 * 60,
    ).unwrap();
    
    let lock_id2 = NesteraContract::create_lock_save(
        env.clone(),
        user.clone(),
        2_000_000,
        60 * 24 * 60 * 60,
    ).unwrap();
    
    // Get user's lock saves
    let user_locks = NesteraContract::get_user_lock_saves(env.clone(), user.clone());
    
    assert_eq!(user_locks.len(), 2);
    assert!(user_locks.contains(&lock_id1));
    assert!(user_locks.contains(&lock_id2));
}

#[test]
fn test_user_balance_update_on_lock_creation() {
    let env = Env::default();
    let contract_id = env.register(NesteraContract, ());
    let user = Address::generate(&env);
    
    env.mock_all_auths();
    
    // Initialize user
    let _user_data = NesteraContract::init_user(env.clone(), user.clone());
    
    // Create lock save
    let _lock_id = NesteraContract::create_lock_save(
        env.clone(),
        user.clone(),
        1_000_000,
        30 * 24 * 60 * 60,
    ).unwrap();
    
    // Check user's updated balance and savings count
    let updated_user = NesteraContract::get_user(env.clone(), user.clone()).unwrap();
    assert_eq!(updated_user.total_balance, 1_000_000);
    assert_eq!(updated_user.savings_count, 1);
}

#[test]
fn test_lock_save_start_and_maturity_times() {
    let env = Env::default();
    let contract_id = env.register(NesteraContract, ());
    let user = Address::generate(&env);
    
    env.mock_all_auths();
    
    // Initialize user
    let _user_data = NesteraContract::init_user(env.clone(), user.clone());
    
    let duration = 30 * 24 * 60 * 60; // 30 days
    let start_time = env.ledger().timestamp();
    
    // Create lock save
    let lock_id = NesteraContract::create_lock_save(
        env.clone(),
        user.clone(),
        1_000_000,
        duration,
    ).unwrap();
    
    // Verify times
    let lock_save = NesteraContract::get_lock_save(env.clone(), lock_id).unwrap();
    assert_eq!(lock_save.start_time, start_time);
    assert_eq!(lock_save.maturity_time, start_time + duration);
}

#[test]
fn test_withdraw_lock_save_success() {
    let env = Env::default();
    let contract_id = env.register(NesteraContract, ());
    let user = Address::generate(&env);
    
    env.mock_all_auths();
    
    // Initialize user and create lock save with short duration
    let _user_data = NesteraContract::init_user(env.clone(), user.clone());
    let lock_id = NesteraContract::create_lock_save(
        env.clone(),
        user.clone(),
        1_000_000,
        1, // 1 second duration
    ).unwrap();
    
    // Advance time to mature the lock
    env.ledger().with_mut(|li| {
        li.timestamp = li.timestamp + 2;
    });
    
    // Withdraw from lock save
    let result = NesteraContract::withdraw_lock_save(env.clone(), user.clone(), lock_id);
    assert!(result.is_ok());
    
    // Verify lock is marked as withdrawn
    let lock_save = NesteraContract::get_lock_save(env.clone(), lock_id).unwrap();
    assert!(lock_save.is_withdrawn);
}

#[test]
fn test_withdraw_lock_save_not_matured() {
    let env = Env::default();
    let contract_id = env.register(NesteraContract, ());
    let user = Address::generate(&env);
    
    env.mock_all_auths();
    
    // Initialize user and create lock save
    let _user_data = NesteraContract::init_user(env.clone(), user.clone());
    let lock_id = NesteraContract::create_lock_save(
        env.clone(),
        user.clone(),
        1_000_000,
        30 * 24 * 60 * 60, // 30 days
    ).unwrap();
    
    // Try to withdraw before maturation
    let result = NesteraContract::withdraw_lock_save(env.clone(), user.clone(), lock_id);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), SavingsError::LockNotMatured);
}

#[test]
fn test_withdraw_lock_save_unauthorized() {
    let env = Env::default();
    let contract_id = env.register(NesteraContract, ());
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    
    env.mock_all_auths();
    
    // Initialize user1 and create lock save
    let _user_data = NesteraContract::init_user(env.clone(), user1.clone());
    let lock_id = NesteraContract::create_lock_save(
        env.clone(),
        user1.clone(),
        1_000_000,
        1, // 1 second duration
    ).unwrap();
    
    // Advance time to mature the lock
    env.ledger().with_mut(|li| {
        li.timestamp = li.timestamp + 2;
    });
    
    // Try to withdraw with different user
    let result = NesteraContract::withdraw_lock_save(env.clone(), user2.clone(), lock_id);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), SavingsError::Unauthorized);
}

#[test]
fn test_withdraw_lock_save_already_withdrawn() {
    let env = Env::default();
    let contract_id = env.register(NesteraContract, ());
    let user = Address::generate(&env);
    
    env.mock_all_auths();
    
    // Initialize user and create lock save with short duration
    let _user_data = NesteraContract::init_user(env.clone(), user.clone());
    let lock_id = NesteraContract::create_lock_save(
        env.clone(),
        user.clone(),
        1_000_000,
        1, // 1 second duration
    ).unwrap();
    
    // Advance time to mature the lock
    env.ledger().with_mut(|li| {
        li.timestamp = li.timestamp + 2;
    });
    
    // First withdrawal should succeed
    let result1 = NesteraContract::withdraw_lock_save(env.clone(), user.clone(), lock_id);
    assert!(result1.is_ok());
    
    // Second withdrawal should fail
    let result2 = NesteraContract::withdraw_lock_save(env.clone(), user.clone(), lock_id);
    assert!(result2.is_err());
    assert_eq!(result2.unwrap_err(), SavingsError::AlreadyWithdrawn);
}

#[test]
fn test_get_user_before_after_init() {
    let env = Env::default();
    let _contract_id = env.register(NesteraContract, ());
    let user = Address::generate(&env);

    // Before init_user, get_user should return None
    let before = NesteraContract::get_user(env.clone(), user.clone());
    assert!(before.is_none());

    // After init_user, get_user should return default user struct
    let created = NesteraContract::init_user(env.clone(), user.clone());
    assert_eq!(created.total_balance, 0);
    assert_eq!(created.savings_count, 0);

    let after = NesteraContract::get_user(env.clone(), user.clone());
    assert!(after.is_some());
    let fetched = after.unwrap();
    assert_eq!(fetched.total_balance, 0);
    assert_eq!(fetched.savings_count, 0);
}

#[test]
fn test_withdraw_returns_amount_with_interest() {
    let env = Env::default();
    let _contract_id = env.register(NesteraContract, ());
    let user = Address::generate(&env);

    env.mock_all_auths();

    // Initialize user
    let _ = NesteraContract::init_user(env.clone(), user.clone());

    // Use exactly one year duration to get 1x interest period
    let one_year_secs: u64 = 365 * 24 * 60 * 60;
    let principal: i128 = 1_000_000;
    let lock_id = NesteraContract::create_lock_save(
        env.clone(),
        user.clone(),
        principal,
        one_year_secs,
    )
    .unwrap();

    // Advance time to at least maturity
    env.ledger().with_mut(|li| {
        li.timestamp = li.timestamp + one_year_secs + 1;
    });

    // Withdraw and validate amount = principal + interest(8% of principal)
    let result = NesteraContract::withdraw_lock_save(env.clone(), user.clone(), lock_id);
    assert!(result.is_ok());
    let withdrawn = result.unwrap();

    let expected_interest: i128 = principal * 800 / 10_000; // 8.00% in bps
    let expected_total: i128 = principal + expected_interest;
    assert_eq!(withdrawn, expected_total);
}

#[test]
fn test_next_lock_id_increments_across_users() {
    let env = Env::default();
    let _contract_id = env.register(NesteraContract, ());
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    env.mock_all_auths();

    // Init both users
    let _ = NesteraContract::init_user(env.clone(), user1.clone());
    let _ = NesteraContract::init_user(env.clone(), user2.clone());

    // Create locks for each user and assert global incrementing IDs
    let id1 = NesteraContract::create_lock_save(
        env.clone(),
        user1.clone(),
        100,
        10,
    )
    .unwrap();
    let id2 = NesteraContract::create_lock_save(
        env.clone(),
        user2.clone(),
        200,
        20,
    )
    .unwrap();

    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
}

#[test]
fn test_user_lock_ids_persist_after_withdraw() {
    let env = Env::default();
    let _contract_id = env.register(NesteraContract, ());
    let user = Address::generate(&env);

    env.mock_all_auths();

    let _ = NesteraContract::init_user(env.clone(), user.clone());

    let lock_id = NesteraContract::create_lock_save(
        env.clone(),
        user.clone(),
        1_000,
        1,
    )
    .unwrap();

    // Mature
    env.ledger().with_mut(|li| {
        li.timestamp = li.timestamp + 2;
    });

    // Withdraw
    let _ = NesteraContract::withdraw_lock_save(env.clone(), user.clone(), lock_id).unwrap();

    // Ensure user's lock IDs still contain the withdrawn lock (we don't remove IDs on withdraw)
    let ids = NesteraContract::get_user_lock_saves(env.clone(), user.clone());
    assert!(ids.contains(&lock_id));
}

#[test]
fn test_check_matured_lock_boundary_condition() {
    let env = Env::default();
    let _contract_id = env.register(NesteraContract, ());
    let user = Address::generate(&env);

    env.mock_all_auths();

    let _ = NesteraContract::init_user(env.clone(), user.clone());

    // Create a lock and fetch its recorded times
    let duration: u64 = 10;
    let lock_id = NesteraContract::create_lock_save(
        env.clone(),
        user.clone(),
        5_000,
        duration,
    )
    .unwrap();

    let lock = NesteraContract::get_lock_save(env.clone(), lock_id).unwrap();

    // Jump to exactly maturity_time; should be considered matured (>=)
    env.ledger().with_mut(|li| {
        li.timestamp = lock.maturity_time;
    });

    let matured = NesteraContract::check_matured_lock(env.clone(), lock_id);
    assert!(matured);
}
