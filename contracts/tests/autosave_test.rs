#[cfg(test)]
mod autosave_tests {
    use soroban_sdk::{testutils::Address as _, Address, Env};
    use Nestera::{NesteraContract, NesteraContractClient};

    fn setup_test_contract() -> (Env, NesteraContractClient<'static>, Address) {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(NesteraContract, ());
        let client = NesteraContractClient::new(&env, &contract_id);

        let user = Address::generate(&env);

        // Initialize user
        client.initialize_user(&user);

        (env, client, user)
    }

    #[test]
    fn test_create_autosave_success() {
        let (env, client, user) = setup_test_contract();

        let amount = 1000;
        let interval = 86400; // 1 day
        let start_time = env.ledger().timestamp();

        let result = client.create_autosave(&user, &amount, &interval, &start_time);
        assert_eq!(result, 1); // First schedule ID

        // Verify schedule was stored
        let schedules = client.get_user_autosaves(&user);
        assert_eq!(schedules.len(), 1);
        assert_eq!(schedules.get(0).unwrap(), 1);
    }

    #[test]
    fn test_create_autosave_zero_amount() {
        let (env, client, user) = setup_test_contract();

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            client.create_autosave(&user, &0, &86400, &env.ledger().timestamp());
        }));
        assert!(result.is_err()); // Should panic with InvalidAmount
    }

    #[test]
    fn test_create_autosave_zero_interval() {
        let (env, client, user) = setup_test_contract();

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            client.create_autosave(&user, &1000, &0, &env.ledger().timestamp());
        }));
        assert!(result.is_err()); // Should panic with InvalidTimestamp
    }

    #[test]
    fn test_create_autosave_user_not_found() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(NesteraContract, ());
        let client = NesteraContractClient::new(&env, &contract_id);

        let user = Address::generate(&env); // Not initialized

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            client.create_autosave(&user, &1000, &86400, &env.ledger().timestamp());
        }));
        assert!(result.is_err()); // Should panic with UserNotFound
    }

    #[test]
    fn test_execute_autosave_success() {
        let (env, client, user) = setup_test_contract();

        let amount = 1000;
        let interval = 86400;
        let start_time = env.ledger().timestamp();

        let schedule_id = client.create_autosave(&user, &amount, &interval, &start_time);

        // Get initial balance
        let initial_balance = client.get_flexi_balance(&user);

        // Execute the schedule
        client.execute_autosave(&schedule_id);

        // Verify balance increased
        let new_balance = client.get_flexi_balance(&user);
        assert_eq!(new_balance, initial_balance + amount);
    }

    #[test]
    fn test_execute_autosave_before_due_time() {
        let (env, client, user) = setup_test_contract();

        let amount = 1000;
        let interval = 86400;
        let start_time = env.ledger().timestamp() + 10000; // Future time

        let schedule_id = client.create_autosave(&user, &amount, &interval, &start_time);

        // Try to execute before due time
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            client.execute_autosave(&schedule_id);
        }));
        assert!(result.is_err()); // Should panic with InvalidTimestamp
    }

    #[test]
    fn test_cancel_autosave_success() {
        let (env, client, user) = setup_test_contract();

        let schedule_id = client.create_autosave(&user, &1000, &86400, &env.ledger().timestamp());

        // Cancel the schedule
        client.cancel_autosave(&user, &schedule_id);

        // Try to execute cancelled schedule - should fail
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            client.execute_autosave(&schedule_id);
        }));
        assert!(result.is_err());
    }

    #[test]
    fn test_cancel_autosave_unauthorized() {
        let (env, client, user) = setup_test_contract();

        let schedule_id = client.create_autosave(&user, &1000, &86400, &env.ledger().timestamp());

        let other_user = Address::generate(&env);
        client.initialize_user(&other_user);

        // Try to cancel someone else's schedule
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            client.cancel_autosave(&other_user, &schedule_id);
        }));
        assert!(result.is_err()); // Should panic with Unauthorized
    }

    #[test]
    fn test_get_user_autosaves() {
        let (env, client, user) = setup_test_contract();

        let start_time = env.ledger().timestamp();

        let id1 = client.create_autosave(&user, &1000, &86400, &start_time);
        let id2 = client.create_autosave(&user, &2000, &172800, &start_time);

        let schedules = client.get_user_autosaves(&user);
        assert_eq!(schedules.len(), 2);
        assert_eq!(schedules.get(0).unwrap(), id1);
        assert_eq!(schedules.get(1).unwrap(), id2);
    }

    #[test]
    fn test_execute_cancelled_schedule() {
        let (env, client, user) = setup_test_contract();

        let schedule_id = client.create_autosave(&user, &1000, &86400, &env.ledger().timestamp());

        // Cancel the schedule
        client.cancel_autosave(&user, &schedule_id);

        // Try to execute - should fail
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            client.execute_autosave(&schedule_id);
        }));
        assert!(result.is_err()); // Should panic with InvalidPlanConfig
    }
}
