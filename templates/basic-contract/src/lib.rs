//! Basic Soroban contract template.
//!
//! Demonstrates:
//! - Contract initialisation with admin storage
//! - A stateful increment operation
//! - Authorization (only admin may reset)
//! - Custom error handling
//! - Event emission
//! - Unit and integration tests using soroban-sdk v29 testutils

#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short,
    Address, Env, Symbol,
};

// ── Storage keys ────────────────────────────────────────────────────────────

#[contracttype]
pub enum DataKey {
    Admin,
    Counter,
}

// ── Errors ───────────────────────────────────────────────────────────────────

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    Unauthorized = 3,
    Overflow = 4,
}

// ── Contract ─────────────────────────────────────────────────────────────────

#[contract]
pub struct BasicContract;

#[contractimpl]
impl BasicContract {
    /// Initialise the contract with an admin address.
    /// Can only be called once.
    pub fn init(env: Env, admin: Address) -> Result<(), ContractError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(ContractError::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Counter, &0_u64);
        env.events().publish(
            (symbol_short!("INIT"),),
            admin,
        );
        Ok(())
    }

    /// Increment the counter by `amount` and return the new value.
    pub fn increment(env: Env, amount: u64) -> Result<u64, ContractError> {
        let current: u64 = env
            .storage()
            .instance()
            .get(&DataKey::Counter)
            .ok_or(ContractError::NotInitialized)?;

        let next = current.checked_add(amount).ok_or(ContractError::Overflow)?;
        env.storage().instance().set(&DataKey::Counter, &next);
        env.events().publish(
            (symbol_short!("INCR"),),
            (current, next),
        );
        Ok(next)
    }

    /// Return the current counter value.
    pub fn get_count(env: Env) -> Result<u64, ContractError> {
        env.storage()
            .instance()
            .get(&DataKey::Counter)
            .ok_or(ContractError::NotInitialized)
    }

    /// Reset the counter to zero. Only the admin may call this.
    pub fn reset(env: Env) -> Result<(), ContractError> {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(ContractError::NotInitialized)?;

        admin.require_auth();
        env.storage().instance().set(&DataKey::Counter, &0_u64);
        env.events().publish(
            (symbol_short!("RESET"),),
            (),
        );
        Ok(())
    }

    /// Return the admin address.
    pub fn get_admin(env: Env) -> Result<Address, ContractError> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(ContractError::NotInitialized)
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    fn setup() -> (Env, Address, BasicContractClient<'static>) {
        let env = Env::default();
        let admin = Address::generate(&env);
        let contract_id = env.register(BasicContract, ());
        let client = BasicContractClient::new(&env, &contract_id);

        // Allow all auth in tests
        env.mock_all_auths();

        client.init(&admin).unwrap();
        (env, admin, client)
    }

    #[test]
    fn test_init_and_get_count() {
        let (_, _, client) = setup();
        assert_eq!(client.get_count().unwrap(), 0);
    }

    #[test]
    fn test_increment() {
        let (_, _, client) = setup();
        assert_eq!(client.increment(&5).unwrap(), 5);
        assert_eq!(client.increment(&3).unwrap(), 8);
    }

    #[test]
    fn test_reset_by_admin() {
        let (_, _, client) = setup();
        client.increment(&10).unwrap();
        assert_eq!(client.get_count().unwrap(), 10);
        client.reset().unwrap();
        assert_eq!(client.get_count().unwrap(), 0);
    }

    #[test]
    fn test_double_init_fails() {
        let (env, admin, client) = setup();
        let result = client.try_init(&admin);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_admin() {
        let (_, admin, client) = setup();
        assert_eq!(client.get_admin().unwrap(), admin);
    }

    #[test]
    fn test_increment_emits_event() {
        let (env, _, client) = setup();
        client.increment(&1).unwrap();
        // Events are emitted: check at least one INCR event exists.
        let events = env.events().all();
        assert!(!events.is_empty());
    }

    #[test]
    fn test_overflow_protection() {
        let (_, _, client) = setup();
        // Set counter near u64::MAX by incrementing a large amount
        client.increment(&(u64::MAX - 1)).unwrap();
        let result = client.try_increment(&2);
        assert!(result.is_err());
    }
}
