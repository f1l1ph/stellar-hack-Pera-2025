#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, MockAuth, MockAuthInvoke},
    token, Address, Env, IntoVal, String,
};

// Create a mock token contract for testing
#[derive(Clone)]
pub struct TestToken {
    pub address: Address,
    pub env: Env,
}

impl TestToken {
    pub fn new(env: &Env, admin: &Address) -> Self {
        let address = env.register_stellar_asset_contract(admin.clone());
        Self {
            address,
            env: env.clone(),
        }
    }

    pub fn mint(&self, to: &Address, amount: &i128) {
        let token_client = token::StellarAssetClient::new(&self.env, &self.address);
        token_client.mint(to, amount);
    }

    // pub fn balance(&self, id: &Address) -> i128 {
    //     let token_client = token::StellarAssetClient::new(&self.env, &self.address);
    //     token_client(id)
    // }
    //
    // pub fn transfer(&self, from: &Address, to: &Address, amount: &i128) {
    //     let token_client = token::StellarAssetClient::new(&self.env, &self.address);
    //     token_client.transfer(from, to, amount);
    // }
}

fn create_test_contract<'a>(e: &Env) -> OptionsContractClient<'a> {
    OptionsContractClient::new(e, &e.register_contract(None, OptionsContract {}))
}

fn create_token_contract(e: &Env, admin: &Address) -> TestToken {
    TestToken::new(e, admin)
}

#[test]
fn test_initialize() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract = create_test_contract(&env);

    // Test successful initialization
    contract.initialize(&admin);

    // Verify admin is set
    assert_eq!(contract.get_admin(), admin);
    assert_eq!(contract.get_pool_counter(), 0);
    assert_eq!(contract.get_option_counter(), 0);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_initialize_twice() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract = create_test_contract(&env);

    contract.initialize(&admin);
    // Should panic with AlreadyInitialized error
    contract.initialize(&admin);
}

#[test]
fn test_add_liquidity_pool() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract = create_test_contract(&env);

    let stable_token = Address::generate(&env);
    let underlying_asset = Address::generate(&env);
    let price_feed = Address::generate(&env);
    let pool_name = String::from_str(&env, "BTC/USDC Pool");

    // Initialize contract
    contract.initialize(&admin);

    // Add liquidity pool
    let pool_id =
        contract.add_liquidity_pool(&stable_token, &underlying_asset, &price_feed, &pool_name);

    // Verify pool was created
    assert_eq!(pool_id, 0);

    let pool = contract.get_pool(&pool_id);
    assert_eq!(pool.pool_id, 0);
    assert_eq!(pool.stable_token, stable_token);
    assert_eq!(pool.underlying_asset, underlying_asset);
    assert_eq!(pool.price_feed, price_feed);
    assert_eq!(pool.name, pool_name);
    assert_eq!(pool.is_active, true);

    // Verify pool counter was incremented
    assert_eq!(contract.get_pool_counter(), 1);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #14)")]
fn test_add_duplicate_pool() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract = create_test_contract(&env);

    let stable_token = Address::generate(&env);
    let underlying_asset = Address::generate(&env);
    let price_feed = Address::generate(&env);
    let pool_name = String::from_str(&env, "BTC/USDC Pool");

    contract.initialize(&admin);

    // Add first pool
    contract.add_liquidity_pool(&stable_token, &underlying_asset, &price_feed, &pool_name);

    // Try to add duplicate pool - should panic
    contract.add_liquidity_pool(&stable_token, &underlying_asset, &price_feed, &pool_name);
}

#[test]
fn test_provide_liquidity() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let provider = Address::generate(&env);
    let contract = create_test_contract(&env);

    let stable_token = create_token_contract(&env, &admin);
    let underlying_asset = Address::generate(&env);
    let price_feed = Address::generate(&env);
    let pool_name = String::from_str(&env, "BTC/USDC Pool");

    // Initialize and create pool
    contract.initialize(&admin);
    let pool_id = contract.add_liquidity_pool(
        &stable_token.address,
        &underlying_asset,
        &price_feed,
        &pool_name,
    );

    // Mint tokens for provider
    stable_token.mint(&provider, &10000);

    // Provide liquidity
    let amount = 1000i128;
    let shares = contract.provide_liquidity(&pool_id, &provider, &amount);

    // For first provider, shares should equal amount (1:1 ratio)
    assert_eq!(shares, amount);

    // Verify pool state
    assert_eq!(contract.get_pool_total_liquidity(&pool_id), amount);
    assert_eq!(contract.get_pool_total_lp_shares(&pool_id), amount);
    assert_eq!(contract.get_pool_lp_shares(&pool_id, &provider), amount);
}

#[test]
fn test_buy_call_option() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let contract = create_test_contract(&env);

    let stable_token = create_token_contract(&env, &admin);
    let underlying_asset = Address::generate(&env);
    let price_feed = Address::generate(&env);
    let pool_name = String::from_str(&env, "BTC/USDC Pool");

    // Setup
    contract.initialize(&admin);
    let pool_id = contract.add_liquidity_pool(
        &stable_token.address,
        &underlying_asset,
        &price_feed,
        &pool_name,
    );

    // Provide initial liquidity
    let provider = Address::generate(&env);
    stable_token.mint(&provider, &100000);
    contract.provide_liquidity(&pool_id, &provider, &50000);

    // Mint tokens for buyer (for premium)
    stable_token.mint(&buyer, &10000);

    // Buy call option
    let strike = 2100_0000000i128; // $2100
    let expiry = env.ledger().timestamp() + 86400; // 1 day from now
    let amount = 10_000_000i128; // 1 unit (1e7 scaling)

    // Mock authorization for the buyer specifically for token transfer
    env.mock_auths(&[MockAuth {
        address: &buyer,
        invoke: &MockAuthInvoke {
            contract: &stable_token.address,
            fn_name: "transfer",
            args: (&buyer, &contract.address, 4200_0000000i128).into_val(&env), // Expected premium
            sub_invokes: &[],
        },
    }]);

    let option_id = contract.buy_option(
        &pool_id,
        &buyer,
        &OptionType::Call,
        &strike,
        &expiry,
        &amount,
    );

    // Verify option was created
    let option = contract.get_option(&option_id);
    assert_eq!(option.pool_id, pool_id);
    assert_eq!(option.buyer, buyer);
    assert_eq!(option.opt_type, OptionType::Call);
    assert_eq!(option.strike, strike);
    assert_eq!(option.expiry, expiry);
    assert_eq!(option.amount, amount);
    assert_eq!(option.is_active, true);
    assert_eq!(option.is_exercised, false);

    // Verify collateral was locked
    let expected_collateral = strike * amount / 10_000_000;
    assert_eq!(
        contract.get_pool_locked_collateral(&pool_id),
        expected_collateral
    );
}
//
// #[test]
// fn test_exercise_call_option_in_the_money() {
//     let env = Env::default();
//     env.mock_all_auths();
//
//     let admin = Address::generate(&env);
//     let buyer = Address::generate(&env);
//     let contract = create_test_contract(&env);
//
//     let stable_token = create_token_contract(&env, &admin);
//     let underlying_asset = Address::generate(&env);
//     let price_feed = Address::generate(&env);
//     let pool_name = String::from_str(&env, "BTC/USDC Pool");
//
//     // Setup
//     contract.initialize(&admin);
//     let pool_id = contract.add_liquidity_pool(
//         &stable_token.address,
//         &underlying_asset,
//         &price_feed,
//         &pool_name,
//     );
//
//     // Provide liquidity and buy option
//     let provider = Address::generate(&env);
//     stable_token.mint(&provider, &100000);
//     contract.provide_liquidity(&pool_id, &provider, &50000);
//
//     stable_token.mint(&buyer, &10000);
//
//     let strike = 1900_0000000i128; // $1900 (below current price of $2000)
//     let expiry = env.ledger().timestamp() + 86400;
//     let amount = 10_000_000i128;
//
//     let option_id = contract.buy_option(
//         &pool_id,
//         &buyer,
//         &OptionType::Call,
//         &strike,
//         &expiry,
//         &amount,
//     );
//
//     let initial_buyer_balance = stable_token.balance(&buyer);
//
//     // Exercise option (current price is $2000, strike is $1900, so it's in the money)
//     let payoff = contract.exercise_option(&option_id);
//
//     // Verify payoff (should be $100 = $2000 - $1900)
//     let expected_payoff = (2000_0000000i128 - strike) * 1; // normalized amount = 1
//     assert_eq!(payoff, expected_payoff);
//
//     // Verify option status
//     let option = contract.get_option(&option_id);
//     assert_eq!(option.is_active, false);
//     assert_eq!(option.is_exercised, true);
//
//     // Verify buyer received payoff
//     assert_eq!(stable_token.balance(&buyer), initial_buyer_balance + payoff);
// }
//
// #[test]
// fn test_withdraw_liquidity() {
//     let env = Env::default();
//     env.mock_all_auths();
//
//     let admin = Address::generate(&env);
//     let provider = Address::generate(&env);
//     let contract = create_test_contract(&env);
//
//     let stable_token = create_token_contract(&env, &admin);
//     let underlying_asset = Address::generate(&env);
//     let price_feed = Address::generate(&env);
//     let pool_name = String::from_str(&env, "BTC/USDC Pool");
//
//     // Setup
//     contract.initialize(&admin);
//     let pool_id = contract.add_liquidity_pool(
//         &stable_token.address,
//         &underlying_asset,
//         &price_feed,
//         &pool_name,
//     );
//
//     // Provide liquidity
//     stable_token.mint(&provider, &10000);
//     let amount = 1000i128;
//     let shares = contract.provide_liquidity(&pool_id, &provider, &amount);
//
//     let initial_balance = stable_token.balance(&provider);
//
//     // Withdraw half the liquidity
//     let withdraw_shares = shares / 2;
//     let withdrawn_amount = contract.withdraw_liquidity(&pool_id, &provider, &withdraw_shares);
//
//     // Verify withdrawal
//     assert_eq!(withdrawn_amount, amount / 2);
//     assert_eq!(stable_token.balance(&provider), initial_balance + withdrawn_amount);
//     assert_eq!(contract.get_pool_lp_shares(&pool_id, &provider), shares - withdraw_shares);
// }

#[test]
fn test_get_all_pools() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract = create_test_contract(&env);

    contract.initialize(&admin);

    // Initially no pools
    let pools = contract.get_all_pools();
    assert_eq!(pools.len(), 0);

    // Add a few pools
    for _i in 0..3 {
        let stable_token = Address::generate(&env);
        let underlying_asset = Address::generate(&env);
        let price_feed = Address::generate(&env);
        let pool_name = String::from_str(&env, "Pool");

        contract.add_liquidity_pool(&stable_token, &underlying_asset, &price_feed, &pool_name);
    }

    // Verify all pools are returned
    let pools = contract.get_all_pools();
    assert_eq!(pools.len(), 3);
    assert_eq!(pools.get(0).unwrap(), 0);
    assert_eq!(pools.get(1).unwrap(), 1);
    assert_eq!(pools.get(2).unwrap(), 2);
}

#[test]
fn test_set_pool_status() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let contract = create_test_contract(&env);

    let stable_token = Address::generate(&env);
    let underlying_asset = Address::generate(&env);
    let price_feed = Address::generate(&env);
    let pool_name = String::from_str(&env, "BTC/USDC Pool");

    contract.initialize(&admin);
    let pool_id =
        contract.add_liquidity_pool(&stable_token, &underlying_asset, &price_feed, &pool_name);

    // Pool should be active by default
    let pool = contract.get_pool(&pool_id);
    assert_eq!(pool.is_active, true);

    // Deactivate pool
    contract.set_pool_status(&pool_id, &false);
    let pool = contract.get_pool(&pool_id);
    assert_eq!(pool.is_active, false);

    // Reactivate pool
    contract.set_pool_status(&pool_id, &true);
    let pool = contract.get_pool(&pool_id);
    assert_eq!(pool.is_active, true);
}
