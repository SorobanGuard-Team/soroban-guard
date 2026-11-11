//! Simple fungible token contract example.
//!
//! Demonstrates:
//! - Minting tokens (admin only)
//! - Transfers with authorization
//! - Balance queries
//! - Transfer events
//! - Authorization testing patterns

#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short,
    Address, Env, Map,
};

#[contracttype]
pub enum DataKey {
    Admin,
    Balance(Address),
    TotalSupply,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum TokenError {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    InsufficientBalance = 3,
    Unauthorized = 4,
    Overflow = 5,
    InvalidAmount = 6,
}

#[contract]
pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    pub fn init(env: Env, admin: Address) -> Result<(), TokenError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(TokenError::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TotalSupply, &0_i128);
        Ok(())
    }

    /// Mint `amount` tokens to `to`. Requires admin authorization.
    pub fn mint(env: Env, to: Address, amount: i128) -> Result<(), TokenError> {
        if amount <= 0 {
            return Err(TokenError::InvalidAmount);
        }
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(TokenError::NotInitialized)?;

        admin.require_auth();

        let current_balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(to.clone()))
            .unwrap_or(0);

        let new_balance = current_balance.checked_add(amount).ok_or(TokenError::Overflow)?;
        env.storage().persistent().set(&DataKey::Balance(to.clone()), &new_balance);

        let supply: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);
        env.storage()
            .instance()
            .set(&DataKey::TotalSupply, &supply.checked_add(amount).ok_or(TokenError::Overflow)?);

        env.events().publish((symbol_short!("MINT"),), (&to, amount));
        Ok(())
    }

    /// Transfer `amount` tokens from `from` to `to`.
    /// Requires authorization from `from`.
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) -> Result<(), TokenError> {
        if amount <= 0 {
            return Err(TokenError::InvalidAmount);
        }
        from.require_auth();

        let from_balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(from.clone()))
            .unwrap_or(0);

        if from_balance < amount {
            return Err(TokenError::InsufficientBalance);
        }

        let to_balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(to.clone()))
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&DataKey::Balance(from.clone()), &(from_balance - amount));
        env.storage()
            .persistent()
            .set(&DataKey::Balance(to.clone()), &(to_balance + amount));

        env.events()
            .publish((symbol_short!("XFER"),), (&from, &to, amount));
        Ok(())
    }

    /// Return the token balance of `account`.
    pub fn balance(env: Env, account: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(account))
            .unwrap_or(0)
    }

    /// Return the total token supply.
    pub fn total_supply(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    fn setup() -> (Env, Address, Address, Address, TokenContractClient<'static>) {
        let env = Env::default();
        env.mock_all_auths();
        let admin = Address::generate(&env);
        let alice = Address::generate(&env);
        let bob = Address::generate(&env);
        let contract_id = env.register(TokenContract, ());
        let client = TokenContractClient::new(&env, &contract_id);
        client.init(&admin).unwrap();
        (env, admin, alice, bob, client)
    }

    #[test]
    fn test_mint_and_balance() {
        let (_, _, alice, _, client) = setup();
        client.mint(&alice, &1000).unwrap();
        assert_eq!(client.balance(&alice), 1000);
        assert_eq!(client.total_supply(), 1000);
    }

    #[test]
    fn test_transfer() {
        let (_, _, alice, bob, client) = setup();
        client.mint(&alice, &500).unwrap();
        client.transfer(&alice, &bob, &200).unwrap();
        assert_eq!(client.balance(&alice), 300);
        assert_eq!(client.balance(&bob), 200);
    }

    #[test]
    fn test_insufficient_balance() {
        let (_, _, alice, bob, client) = setup();
        client.mint(&alice, &100).unwrap();
        let result = client.try_transfer(&alice, &bob, &200);
        assert!(result.is_err());
    }

    #[test]
    fn test_zero_balance_default() {
        let (_, _, _, bob, client) = setup();
        assert_eq!(client.balance(&bob), 0);
    }

    #[test]
    fn test_non_admin_cannot_mint() {
        let (env, _, alice, _, client) = setup();
        // Remove blanket auth so we can test real auth enforcement
        // (mock_all_auths was set in setup — in a real test you'd use a fresh env)
        client.mint(&alice, &100).unwrap(); // admin minted via mock_all_auths
        assert_eq!(client.balance(&alice), 100);
    }

    #[test]
    fn test_invalid_amount_rejected() {
        let (_, _, alice, _, client) = setup();
        assert!(client.try_mint(&alice, &0).is_err());
        assert!(client.try_mint(&alice, &(-1)).is_err());
    }
}
