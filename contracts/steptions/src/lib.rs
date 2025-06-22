#![no_std]
use sep_40_oracle::{Asset, PriceData, PriceFeedClient, PriceFeedTrait};
use soroban_sdk::{
    contract, contractimpl, contractmeta, contracttype, log, panic_with_error, symbol_short,
    token::{self, TokenClient},
    Address, Env, Error, Map, String, Symbol, Vec,
};

// Contract metadata
contractmeta!(
    key = "description",
    val = "Multi-Pool American-Style Options Trading Platform - Pool-based, cash-settled options contract"
);

#[contract]
pub struct OptionsContract;

// Storage Keys for Stellar key-value pairs
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    // Contract configuration
    Admin,

    // Pool management
    PoolCounter,
    Pool(u64),                    // Pool ID -> PoolData
    PoolExists(Address, Address), // (stable_token, underlying_asset) -> pool_id

    // Pool-specific data
    PoolTotalLiquidity(u64),
    PoolLockedCollateral(u64),
    PoolTotalLpShares(u64),
    PoolLpShares(u64, Address), // (pool_id, user) -> shares

    // Options
    OptionCounter,
    Option(u64),
}

#[contract]
pub struct MyPriceFeed;

#[contractimpl]
impl PriceFeedTrait for MyPriceFeed {
    fn base(env: Env) -> Asset {
        todo!()
    }

    fn assets(env: Env) -> Vec<Asset> {
        todo!()
    }

    fn decimals(env: Env) -> u32 {
        todo!()
    }

    fn resolution(env: Env) -> u32 {
        todo!()
    }

    fn price(env: Env, asset: Asset, timestamp: u64) -> Option<PriceData> {
        todo!()
    }

    fn prices(env: Env, asset: Asset, records: u32) -> Option<Vec<PriceData>> {
        todo!()
    }

    fn lastprice(env: Env, asset: Asset) -> Option<PriceData> {
        todo!()
    }
    // impl the trait functions
}

// Liquidity Pool struct
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PoolData {
    pub pool_id: u64,
    pub stable_token: Address,     // Token used for premiums & settlements
    pub underlying_asset: Address, // The asset this pool trades options for
    pub price_feed: Address,       // Price oracle for the underlying asset
    pub name: String,              // Human readable name like "BTC/USDC Options Pool"
    pub is_active: bool,           // Pool can be paused by admin
}

// Option types
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OptionType {
    Call,
    Put,
}

// Option struct - now includes pool_id
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OptionData {
    pub pool_id: u64, // Which pool this option belongs to
    pub buyer: Address,
    pub opt_type: OptionType,
    pub strike: i128,       // strike price (scaled 1e7 for Stellar)
    pub expiry: u64,        // unix timestamp
    pub amount: i128,       // quantity (scaled 1e7)
    pub premium_paid: i128, // premium paid
    pub collateral: i128,   // locked collateral
    pub is_exercised: bool,
    pub is_active: bool,
}

// Error types
#[contracttype]
#[derive(Clone, Debug, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum OptionsError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    Unauthorized = 3,
    InvalidAmount = 4,
    InsufficientLiquidity = 5,
    OptionNotFound = 6,
    OptionNotActive = 7,
    OptionExpired = 8,
    NotOptionOwner = 9,
    NotInTheMoney = 10,
    InsufficientShares = 11,
    PoolNotFound = 12,
    PoolNotActive = 13,
    PoolAlreadyExists = 14,
    InvalidPrice = 15,
}

impl From<OptionsError> for Error {
    fn from(error: OptionsError) -> Self {
        Error::from_contract_error(error as u32)
    }
}

// Event topics
const LIQUIDITY_PROVIDED: Symbol = symbol_short!("liq_prov");
const LIQUIDITY_WITHDRAWN: Symbol = symbol_short!("liq_with");
const OPTION_PURCHASED: Symbol = symbol_short!("opt_purch");
const OPTION_EXERCISED: Symbol = symbol_short!("opt_exerc");
const OPTION_EXPIRED: Symbol = symbol_short!("opt_exp");
const POOL_ADDED: Symbol = symbol_short!("pool_add");
const POOL_STATUS_CHANGED: Symbol = symbol_short!("pool_stat");

#[contractimpl]
impl OptionsContract {
    /// Initialize the contract with admin
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic_with_error!(&env, OptionsError::AlreadyInitialized);
        }

        admin.require_auth();

        // Store configuration
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::PoolCounter, &0u64);
        env.storage().instance().set(&DataKey::OptionCounter, &0u64);

        log!(&env, "Multi-pool options contract initialized");
    }

    /// Admin function to add a new liquidity pool
    pub fn add_liquidity_pool(
        env: Env,
        stable_token: Address,
        underlying_asset: Address,
        price_feed: Address,
        name: String,
    ) -> u64 {
        // Check admin authorization
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap_or_else(|| panic_with_error!(&env, OptionsError::NotInitialized));
        admin.require_auth();

        // Check if pool already exists for this pair
        if env.storage().persistent().has(&DataKey::PoolExists(
            stable_token.clone(),
            underlying_asset.clone(),
        )) {
            panic_with_error!(&env, OptionsError::PoolAlreadyExists);
        }

        let pool_id = Self::get_pool_counter(env.clone());

        // Create pool data
        let pool = PoolData {
            pool_id,
            stable_token: stable_token.clone(),
            underlying_asset: underlying_asset.clone(),
            price_feed,
            name: name.clone(),
            is_active: true,
        };

        // Store pool data
        env.storage()
            .persistent()
            .set(&DataKey::Pool(pool_id), &pool);
        env.storage().persistent().set(
            &DataKey::PoolExists(stable_token.clone(), underlying_asset.clone()),
            &pool_id,
        );

        // Initialize pool financial data
        env.storage()
            .persistent()
            .set(&DataKey::PoolTotalLiquidity(pool_id), &0i128);
        env.storage()
            .persistent()
            .set(&DataKey::PoolLockedCollateral(pool_id), &0i128);
        env.storage()
            .persistent()
            .set(&DataKey::PoolTotalLpShares(pool_id), &0i128);

        // Update counter
        env.storage()
            .instance()
            .set(&DataKey::PoolCounter, &(pool_id + 1));

        // Emit event
        env.events().publish(
            (POOL_ADDED, admin),
            (pool_id, stable_token, underlying_asset, name),
        );

        log!(&env, "New liquidity pool added: {}", pool_id);
        pool_id
    }

    /// Admin function to toggle pool active status
    pub fn set_pool_status(env: Env, pool_id: u64, is_active: bool) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap_or_else(|| panic_with_error!(&env, OptionsError::NotInitialized));
        admin.require_auth();

        let mut pool = Self::get_pool(env.clone(), pool_id);
        pool.is_active = is_active;
        env.storage()
            .persistent()
            .set(&DataKey::Pool(pool_id), &pool);

        env.events()
            .publish((POOL_STATUS_CHANGED, admin), (pool_id, is_active));
    }

    /// Provide liquidity to a specific pool
    pub fn provide_liquidity(env: Env, pool_id: u64, provider: Address, amount: i128) -> i128 {
        provider.require_auth();

        if amount <= 0 {
            panic_with_error!(&env, OptionsError::InvalidAmount);
        }

        let pool = Self::get_pool(env.clone(), pool_id);
        if !pool.is_active {
            panic_with_error!(&env, OptionsError::PoolNotActive);
        }

        let total_liquidity = Self::get_pool_total_liquidity(env.clone(), pool_id);
        let total_lp_shares = Self::get_pool_total_lp_shares(env.clone(), pool_id);

        // Transfer tokens to contract
        let token_client = TokenClient::new(&env, &pool.stable_token);
        token_client.transfer(&provider, &env.current_contract_address(), &amount);

        // Calculate LP shares
        let shares = if total_lp_shares == 0 {
            amount // 1:1 ratio for first provider
        } else {
            (amount * total_lp_shares) / total_liquidity
        };

        // Update user shares
        let current_shares = Self::get_pool_lp_shares(env.clone(), pool_id, provider.clone());
        env.storage().persistent().set(
            &DataKey::PoolLpShares(pool_id, provider.clone()),
            &(current_shares + shares),
        );

        // Update totals
        env.storage().persistent().set(
            &DataKey::PoolTotalLpShares(pool_id),
            &(total_lp_shares + shares),
        );
        env.storage().persistent().set(
            &DataKey::PoolTotalLiquidity(pool_id),
            &(total_liquidity + amount),
        );

        // Emit event
        env.events()
            .publish((LIQUIDITY_PROVIDED, provider), (pool_id, amount, shares));

        shares
    }

    /// Withdraw liquidity from a specific pool
    pub fn withdraw_liquidity(
        env: Env,
        pool_id: u64,
        provider: Address,
        share_amount: i128,
    ) -> i128 {
        provider.require_auth();

        let pool = Self::get_pool(env.clone(), pool_id);
        if !pool.is_active {
            panic_with_error!(&env, OptionsError::PoolNotActive);
        }

        let user_shares = Self::get_pool_lp_shares(env.clone(), pool_id, provider.clone());
        if user_shares < share_amount {
            panic_with_error!(&env, OptionsError::InsufficientShares);
        }

        let total_liquidity = Self::get_pool_total_liquidity(env.clone(), pool_id);
        let locked_collateral = Self::get_pool_locked_collateral(env.clone(), pool_id);
        let total_lp_shares = Self::get_pool_total_lp_shares(env.clone(), pool_id);

        // Calculate withdrawal amount
        let pool_portion = (share_amount * total_liquidity) / total_lp_shares;
        let unlocked = total_liquidity - locked_collateral;

        if pool_portion > unlocked {
            panic_with_error!(&env, OptionsError::InsufficientLiquidity);
        }

        // Update storage
        env.storage().persistent().set(
            &DataKey::PoolLpShares(pool_id, provider.clone()),
            &(user_shares - share_amount),
        );
        env.storage().persistent().set(
            &DataKey::PoolTotalLpShares(pool_id),
            &(total_lp_shares - share_amount),
        );
        env.storage().persistent().set(
            &DataKey::PoolTotalLiquidity(pool_id),
            &(total_liquidity - pool_portion),
        );

        // Transfer tokens to provider
        let token_client = TokenClient::new(&env, &pool.stable_token);
        token_client.transfer(&env.current_contract_address(), &provider, &pool_portion);

        // Emit event
        env.events().publish(
            (LIQUIDITY_WITHDRAWN, provider),
            (pool_id, share_amount, pool_portion),
        );

        pool_portion
    }

    /// Buy an option from a specific pool
    pub fn buy_option(
        env: Env,
        pool_id: u64,
        buyer: Address,
        opt_type: OptionType,
        strike: i128,
        expiry: u64,
        amount: i128,
    ) -> u64 {
        buyer.require_auth();

        let pool = Self::get_pool(env.clone(), pool_id);
        if !pool.is_active {
            panic_with_error!(&env, OptionsError::PoolNotActive);
        }

        if expiry <= env.ledger().timestamp() {
            panic_with_error!(&env, OptionsError::OptionExpired);
        }

        if amount <= 0 || strike <= 0 {
            panic_with_error!(&env, OptionsError::InvalidAmount);
        }

        // Calculate premium (2% of strike * amount, normalized for 1e7 scaling)
        let normalized_amount = amount / 10_000_000;
        let normalized_amount = if normalized_amount == 0 {
            1
        } else {
            normalized_amount
        };
        let premium = (strike * normalized_amount * 200) / 10000; // 2%

        // Calculate required collateral
        let collateral_needed = strike * amount / 10_000_000;

        // Check available liquidity in this pool
        let total_liquidity = Self::get_pool_total_liquidity(env.clone(), pool_id);
        let locked_collateral = Self::get_pool_locked_collateral(env.clone(), pool_id);
        let unlocked = total_liquidity - locked_collateral;

        if unlocked < collateral_needed {
            panic_with_error!(&env, OptionsError::InsufficientLiquidity);
        }

        // Transfer premium from buyer
        let token_client = TokenClient::new(&env, &pool.stable_token);
        token_client.transfer(&buyer, &env.current_contract_address(), &premium);

        // Update locked collateral for this pool
        env.storage().persistent().set(
            &DataKey::PoolLockedCollateral(pool_id),
            &(locked_collateral + collateral_needed),
        );

        // Create option
        let option_id = Self::get_option_counter(env.clone());
        let option = OptionData {
            pool_id,
            buyer: buyer.clone(),
            opt_type: opt_type.clone(),
            strike,
            expiry,
            amount,
            premium_paid: premium,
            collateral: collateral_needed,
            is_exercised: false,
            is_active: true,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Option(option_id), &option);
        env.storage()
            .instance()
            .set(&DataKey::OptionCounter, &(option_id + 1));

        // Emit event
        env.events().publish(
            (OPTION_PURCHASED, buyer),
            (
                option_id,
                pool_id,
                opt_type,
                strike,
                expiry,
                amount,
                premium,
                collateral_needed,
            ),
        );

        option_id
    }

    /// Exercise an option (American-style)
    pub fn exercise_option(env: Env, option_id: u64) -> i128 {
        let mut option = Self::get_option(env.clone(), option_id);

        if !option.is_active {
            panic_with_error!(&env, OptionsError::OptionNotActive);
        }

        option.buyer.require_auth();

        if env.ledger().timestamp() > option.expiry {
            panic_with_error!(&env, OptionsError::OptionExpired);
        }

        let pool = Self::get_pool(env.clone(), option.pool_id);

        // Get current price from the pool's price feed
        let current_price = Self::get_price_from_feed(env.clone(), pool.price_feed);
        let mut payoff = 0i128;

        // Calculate payoff
        match option.opt_type {
            OptionType::Call => {
                if current_price > option.strike {
                    let normalized_amount = option.amount / 10_000_000;
                    payoff = (current_price - option.strike)
                        * if normalized_amount == 0 {
                            1
                        } else {
                            normalized_amount
                        };
                }
            }
            OptionType::Put => {
                if current_price < option.strike {
                    let normalized_amount = option.amount / 10_000_000;
                    payoff = (option.strike - current_price)
                        * if normalized_amount == 0 {
                            1
                        } else {
                            normalized_amount
                        };
                }
            }
        }

        // Update option status
        option.is_active = false;
        option.is_exercised = true;
        env.storage()
            .persistent()
            .set(&DataKey::Option(option_id), &option);

        // Update locked collateral for this pool
        let locked_collateral = Self::get_pool_locked_collateral(env.clone(), option.pool_id);
        env.storage().persistent().set(
            &DataKey::PoolLockedCollateral(option.pool_id),
            &(locked_collateral - option.collateral),
        );

        // Transfer payoff if any
        if payoff > 0 {
            let actual_payoff = if payoff > option.collateral {
                option.collateral
            } else {
                payoff
            };
            let token_client = TokenClient::new(&env, &pool.stable_token);
            token_client.transfer(
                &env.current_contract_address(),
                &option.buyer,
                &actual_payoff,
            );

            env.events()
                .publish((OPTION_EXERCISED, option.buyer), (option_id, actual_payoff));
            actual_payoff
        } else {
            env.events()
                .publish((OPTION_EXERCISED, option.buyer), (option_id, 0i128));
            0
        }
    }

    /// Expire an option (release collateral)
    pub fn expire_option(env: Env, option_id: u64) {
        let mut option = Self::get_option(env.clone(), option_id);

        if !option.is_active {
            panic_with_error!(&env, OptionsError::OptionNotActive);
        }

        if env.ledger().timestamp() <= option.expiry {
            panic_with_error!(&env, OptionsError::OptionExpired);
        }

        // Mark as expired
        option.is_active = false;
        option.is_exercised = false;
        env.storage()
            .persistent()
            .set(&DataKey::Option(option_id), &option);

        // Free collateral from pool
        let locked_collateral = Self::get_pool_locked_collateral(env.clone(), option.pool_id);
        env.storage().persistent().set(
            &DataKey::PoolLockedCollateral(option.pool_id),
            &(locked_collateral - option.collateral),
        );

        env.events()
            .publish((OPTION_EXPIRED, option.buyer), option_id);
    }

    // View functions for pools
    pub fn get_pool_counter(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::PoolCounter)
            .unwrap_or(0)
    }

    pub fn get_pool(env: Env, pool_id: u64) -> PoolData {
        env.storage()
            .persistent()
            .get(&DataKey::Pool(pool_id))
            .unwrap_or_else(|| panic_with_error!(&env, OptionsError::PoolNotFound))
    }

    pub fn get_pool_by_assets(env: Env, stable_token: Address, underlying_asset: Address) -> u64 {
        env.storage()
            .persistent()
            .get(&DataKey::PoolExists(stable_token, underlying_asset))
            .unwrap_or_else(|| panic_with_error!(&env, OptionsError::PoolNotFound))
    }

    pub fn get_pool_total_liquidity(env: Env, pool_id: u64) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::PoolTotalLiquidity(pool_id))
            .unwrap_or(0)
    }

    pub fn get_pool_locked_collateral(env: Env, pool_id: u64) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::PoolLockedCollateral(pool_id))
            .unwrap_or(0)
    }

    pub fn get_pool_total_lp_shares(env: Env, pool_id: u64) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::PoolTotalLpShares(pool_id))
            .unwrap_or(0)
    }

    pub fn get_pool_lp_shares(env: Env, pool_id: u64, provider: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::PoolLpShares(pool_id, provider))
            .unwrap_or(0)
    }

    // Other view functions
    pub fn get_option_counter(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&DataKey::OptionCounter)
            .unwrap_or(0)
    }

    pub fn get_option(env: Env, option_id: u64) -> OptionData {
        env.storage()
            .persistent()
            .get(&DataKey::Option(option_id))
            .unwrap_or_else(|| panic_with_error!(&env, OptionsError::OptionNotFound))
    }

    pub fn get_admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap_or_else(|| panic_with_error!(&env, OptionsError::NotInitialized))
    }

    // Get price from SEP-40 oracle
    pub fn get_price_from_feed(env: Env, price_feed: Address) -> i128 {
        // Create a client for the SEP-40 price feed oracle
        let price_feed_client = PriceFeedClient::new(&env, &price_feed);

        // Create an Asset instance for the asset we want to get the price for
        // In this case, we're using the default asset (typically the base asset of the oracle)
        let asset = Asset::Other(Symbol::new(&env, "XLM"));

        // Get the latest price from the oracle
        let price_data_opt = price_feed_client.lastprice(&asset);

        // Handle the Option<PriceData> return type
        match price_data_opt {
            Some(price_data) => {
                // Extract the price from the PriceData
                price_data.price
            }
            None => {
                // If no price is available, panic with an error
                panic_with_error!(&env, OptionsError::InvalidPrice);
            }
        }
    }

    // Admin function to update price feed for a pool
    pub fn update_pool_price_feed(env: Env, pool_id: u64, new_feed: Address) {
        let admin = Self::get_admin(env.clone());
        admin.require_auth();

        let mut pool = Self::get_pool(env.clone(), pool_id);
        pool.price_feed = new_feed;
        env.storage()
            .persistent()
            .set(&DataKey::Pool(pool_id), &pool);
    }

    // Get all pools (for UI purposes) - returns Vec of pool IDs
    pub fn get_all_pools(env: Env) -> Vec<u64> {
        let pool_count = Self::get_pool_counter(env.clone());
        let mut pools = Vec::new(&env);

        for i in 0..pool_count {
            if env.storage().persistent().has(&DataKey::Pool(i)) {
                pools.push_back(i);
            }
        }

        pools
    }
}

mod test;
