use crate::errors::SavingsError;
use crate::rewards::storage::get_user_rewards;
use crate::storage_types::DataKey;
use soroban_sdk::{contracttype, Address, Env, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionProposal {
    pub id: u64,
    pub creator: Address,
    pub description: String,
    pub start_time: u64,
    pub end_time: u64,
    pub executed: bool,
    pub for_votes: u128,
    pub against_votes: u128,
    pub abstain_votes: u128,
    pub action: ProposalAction,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Proposal {
    pub id: u64,
    pub creator: Address,
    pub description: String,
    pub start_time: u64,
    pub end_time: u64,
    pub executed: bool,
    pub for_votes: u128,
    pub against_votes: u128,
    pub abstain_votes: u128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VotingConfig {
    pub quorum: u32,
    pub voting_period: u64,
    pub timelock_duration: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GovernanceKey {
    Proposal(u64),
    ActionProposal(u64),
    NextProposalId,
    VotingConfig,
    AllProposals,
    GovernanceActive,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProposalAction {
    SetFlexiRate(i128),
    SetGoalRate(i128),
    SetGroupRate(i128),
    SetLockRate(u64, i128),
    PauseContract,
    UnpauseContract,
}

/// Calculates voting power for a user based on their lifetime deposited funds
pub fn get_voting_power(env: &Env, user: &Address) -> u128 {
    let rewards = get_user_rewards(env, user.clone());
    rewards.lifetime_deposited.max(0) as u128
}

/// Creates a new governance proposal
pub fn create_proposal(
    env: &Env,
    creator: Address,
    description: String,
) -> Result<u64, SavingsError> {
    creator.require_auth();

    let config = get_voting_config(env)?;
    let proposal_id = get_next_proposal_id(env);
    let now = env.ledger().timestamp();

    let proposal = Proposal {
        id: proposal_id,
        creator: creator.clone(),
        description,
        start_time: now,
        end_time: now + config.voting_period,
        executed: false,
        for_votes: 0,
        against_votes: 0,
        abstain_votes: 0,
    };

    env.storage()
        .persistent()
        .set(&GovernanceKey::Proposal(proposal_id), &proposal);

    let mut all_proposals: Vec<u64> = env
        .storage()
        .persistent()
        .get(&GovernanceKey::AllProposals)
        .unwrap_or(Vec::new(env));
    all_proposals.push_back(proposal_id);
    env.storage()
        .persistent()
        .set(&GovernanceKey::AllProposals, &all_proposals);

    env.storage()
        .persistent()
        .set(&GovernanceKey::NextProposalId, &(proposal_id + 1));

    env.events().publish(
        (soroban_sdk::symbol_short!("proposal"), creator, proposal_id),
        (),
    );

    Ok(proposal_id)
}

/// Creates a governance proposal with an action
pub fn create_action_proposal(
    env: &Env,
    creator: Address,
    description: String,
    action: ProposalAction,
) -> Result<u64, SavingsError> {
    creator.require_auth();

    let config = get_voting_config(env)?;
    let proposal_id = get_next_proposal_id(env);
    let now = env.ledger().timestamp();

    let proposal = ActionProposal {
        id: proposal_id,
        creator: creator.clone(),
        description,
        start_time: now,
        end_time: now + config.voting_period,
        executed: false,
        for_votes: 0,
        against_votes: 0,
        abstain_votes: 0,
        action,
    };

    env.storage()
        .persistent()
        .set(&GovernanceKey::ActionProposal(proposal_id), &proposal);

    let mut all_proposals: Vec<u64> = env
        .storage()
        .persistent()
        .get(&GovernanceKey::AllProposals)
        .unwrap_or(Vec::new(env));
    all_proposals.push_back(proposal_id);
    env.storage()
        .persistent()
        .set(&GovernanceKey::AllProposals, &all_proposals);

    env.storage()
        .persistent()
        .set(&GovernanceKey::NextProposalId, &(proposal_id + 1));

    env.events().publish(
        (soroban_sdk::symbol_short!("proposal"), creator, proposal_id),
        (),
    );

    Ok(proposal_id)
}

/// Gets an action proposal by ID
pub fn get_action_proposal(env: &Env, proposal_id: u64) -> Option<ActionProposal> {
    env.storage()
        .persistent()
        .get(&GovernanceKey::ActionProposal(proposal_id))
}

/// Gets a proposal by ID
pub fn get_proposal(env: &Env, proposal_id: u64) -> Option<Proposal> {
    env.storage()
        .persistent()
        .get(&GovernanceKey::Proposal(proposal_id))
}

/// Lists all proposal IDs
pub fn list_proposals(env: &Env) -> Vec<u64> {
    env.storage()
        .persistent()
        .get(&GovernanceKey::AllProposals)
        .unwrap_or(Vec::new(env))
}

/// Gets the voting configuration
pub fn get_voting_config(env: &Env) -> Result<VotingConfig, SavingsError> {
    env.storage()
        .persistent()
        .get(&GovernanceKey::VotingConfig)
        .ok_or(SavingsError::InternalError)
}

/// Initializes voting configuration (admin only)
pub fn init_voting_config(
    env: &Env,
    admin: Address,
    config: VotingConfig,
) -> Result<(), SavingsError> {
    admin.require_auth();

    let stored_admin: Address = env
        .storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(SavingsError::Unauthorized)?;

    if admin != stored_admin {
        return Err(SavingsError::Unauthorized);
    }

    if env.storage().persistent().has(&GovernanceKey::VotingConfig) {
        return Err(SavingsError::ConfigAlreadyInitialized);
    }

    env.storage()
        .persistent()
        .set(&GovernanceKey::VotingConfig, &config);
    env.storage()
        .persistent()
        .set(&GovernanceKey::NextProposalId, &1u64);

    Ok(())
}

fn get_next_proposal_id(env: &Env) -> u64 {
    env.storage()
        .persistent()
        .get(&GovernanceKey::NextProposalId)
        .unwrap_or(1)
}

/// Casts a weighted vote on a proposal
pub fn cast_vote(
    env: &Env,
    user: Address,
    proposal_id: u64,
    support: bool,
) -> Result<(), SavingsError> {
    user.require_auth();
    let weight = get_voting_power(env, &user);

    if weight == 0 {
        return Err(SavingsError::InsufficientBalance);
    }

    env.events().publish(
        (soroban_sdk::symbol_short!("vote"), user, proposal_id),
        (support, weight),
    );

    Ok(())
}

/// Checks if governance is active
pub fn is_governance_active(env: &Env) -> bool {
    env.storage()
        .persistent()
        .get(&GovernanceKey::GovernanceActive)
        .unwrap_or(false)
}

/// Activates governance (admin only, one-time)
pub fn activate_governance(env: &Env, admin: Address) -> Result<(), SavingsError> {
    admin.require_auth();

    let stored_admin: Address = env
        .storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(SavingsError::Unauthorized)?;

    if admin != stored_admin {
        return Err(SavingsError::Unauthorized);
    }

    env.storage()
        .persistent()
        .set(&GovernanceKey::GovernanceActive, &true);

    Ok(())
}

/// Validates caller is admin or governance is active
pub fn validate_admin_or_governance(env: &Env, caller: &Address) -> Result<bool, SavingsError> {
    if is_governance_active(env) {
        return Ok(true);
    }

    let stored_admin: Address = env
        .storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(SavingsError::Unauthorized)?;

    if caller == &stored_admin {
        Ok(false)
    } else {
        Err(SavingsError::Unauthorized)
    }
}
