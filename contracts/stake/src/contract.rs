use soroban_sdk::{contractimpl, contractmeta, log, Address, Env};

use crate::{
    error::ContractError,
    msg::{
        AllStakedResponse, AnnualizedRewardsResponse, DistributedRewardsResponse, StakedResponse,
        WithdrawableRewardsResponse,
    },
    storage::{get_config, get_stakes, save_stakes, BondingInfo, Stake},
};

// Metadata that is added on to the WASM custom section
contractmeta!(
    key = "Description",
    val = "Phoenix Protocol LP Share token staking"
);

pub struct Staking;

pub trait StakingTrait {
    // Sets the token contract addresses for this pool
    // epoch: Number of seconds between payments
    fn initialize(
        env: Env,
        admin: Address,
        lp_token: Address,
        token_per_power: u128,
        min_bond: u128,
        max_distributions: u32,
    ) -> Result<(), ContractError>;

    fn bond(env: Env, sender: Address, tokens: u128) -> Result<(), ContractError>;

    fn unbond(env: Env, tokens: u128) -> Result<(), ContractError>;

    fn create_distribution_flow(
        env: Env,
        manager: Address,
        asset: Address,
    ) -> Result<(), ContractError>;

    fn distribute_rewards(env: Env) -> Result<(), ContractError>;

    fn withdraw_rewards(env: Env, receiver: Option<Address>) -> Result<(), ContractError>;

    fn fund_distribution(
        env: Env,
        start_time: u64,
        distribution_duration: u64,
        amount: u128,
    ) -> Result<(), ContractError>;

    // QUERIES

    fn query_staked(env: Env, address: Address) -> Result<StakedResponse, ContractError>;

    fn query_all_staked(env: Env) -> Result<AllStakedResponse, ContractError>;

    fn query_annualized_rewards(env: Env) -> Result<AnnualizedRewardsResponse, ContractError>;

    fn query_withdrawable_rewards(
        env: Env,
        address: Address,
    ) -> Result<WithdrawableRewardsResponse, ContractError>;

    fn query_distributed_rewards(env: Env) -> Result<DistributedRewardsResponse, ContractError>;
}

#[contractimpl]
impl StakingTrait for Staking {
    fn initialize(
        _env: Env,
        _admin: Address,
        _lp_token: Address,
        _token_per_power: u128,
        _min_bond: u128,
        _max_distributions: u32,
    ) -> Result<(), ContractError> {
        unimplemented!();
    }

    fn bond(env: Env, sender: Address, tokens: u128) -> Result<(), ContractError> {
        sender.require_auth();

        let ledger = env.ledger();
        let config = get_config(&env)?;

        if tokens < config.min_bond {
            log!(
                &env,
                "Trying to bond {} which is less then minimum {} required!",
                tokens,
                config.min_bond
            );
            return Err(ContractError::StakeLessThenMinBond);
        }

        let lp_token_client = token_contract::Client::new(&env, &config.lp_token);
        lp_token_client.transfer(&sender, &env.current_contract_address(), &tokens);

        let mut stakes = get_stakes(&env, &sender)?;
        let stake = Stake {
            stake: tokens,
            stake_timestamp: ledger.timestamp(),
        };
        // TODO: Discuss: Add implementation to add stake if another is present in +-24h timestamp to avoid
        // creating multiple stakes the same day

        stakes.stakes.push_back(stake);

        unimplemented!();
    }

    fn unbond(_env: Env, _tokens: u128) -> Result<(), ContractError> {
        unimplemented!();
    }

    fn create_distribution_flow(
        _env: Env,
        _manager: Address,
        _asset: Address,
    ) -> Result<(), ContractError> {
        unimplemented!();
    }

    fn distribute_rewards(_env: Env) -> Result<(), ContractError> {
        unimplemented!();
    }

    fn withdraw_rewards(_env: Env, _receiver: Option<Address>) -> Result<(), ContractError> {
        unimplemented!();
    }

    fn fund_distribution(
        _env: Env,
        _start_time: u64,
        _distribution_duration: u64,
        _amount: u128,
    ) -> Result<(), ContractError> {
        unimplemented!();
    }

    // QUERIES

    fn query_staked(_env: Env, _address: Address) -> Result<StakedResponse, ContractError> {
        unimplemented!();
    }

    fn query_all_staked(_env: Env) -> Result<AllStakedResponse, ContractError> {
        unimplemented!();
    }

    fn query_annualized_rewards(_env: Env) -> Result<AnnualizedRewardsResponse, ContractError> {
        unimplemented!();
    }

    fn query_withdrawable_rewards(
        _env: Env,
        _address: Address,
    ) -> Result<WithdrawableRewardsResponse, ContractError> {
        unimplemented!();
    }

    fn query_distributed_rewards(_env: Env) -> Result<DistributedRewardsResponse, ContractError> {
        unimplemented!();
    }
}
