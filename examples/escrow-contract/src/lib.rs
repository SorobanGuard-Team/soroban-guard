//! Simple escrow contract example.
//!
//! Demonstrates:
//! - Multi-party state machine (Created → Funded → Released / Cancelled)
//! - Conditional authorization (only buyer or arbiter can release)
//! - Time-based expiry
//! - State transition events

#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short,
    Address, Env,
};

#[contracttype]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum EscrowState {
    Created,
    Released,
    Cancelled,
}

#[contracttype]
#[derive(Clone)]
pub struct EscrowData {
    pub buyer: Address,
    pub seller: Address,
    pub arbiter: Address,
    pub amount: i128,
    pub expiry_ledger: u32,
    pub state: EscrowState,
}

#[contracttype]
pub enum DataKey {
    Escrow,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum EscrowError {
    AlreadyInitialized = 1,
    NotFound = 2,
    AlreadySettled = 3,
    NotExpired = 4,
    Expired = 5,
    Unauthorized = 6,
    InvalidAmount = 7,
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    /// Create an escrow agreement.
    pub fn create(
        env: Env,
        buyer: Address,
        seller: Address,
        arbiter: Address,
        amount: i128,
        expiry_ledger: u32,
    ) -> Result<(), EscrowError> {
        if amount <= 0 {
            return Err(EscrowError::InvalidAmount);
        }
        if env.storage().instance().has(&DataKey::Escrow) {
            return Err(EscrowError::AlreadyInitialized);
        }
        buyer.require_auth();

        let data = EscrowData {
            buyer: buyer.clone(),
            seller,
            arbiter,
            amount,
            expiry_ledger,
            state: EscrowState::Created,
        };
        env.storage().instance().set(&DataKey::Escrow, &data);
        env.events()
            .publish((symbol_short!("CREATED"),), (buyer, amount));
        Ok(())
    }

    /// Release funds to the seller.
    /// Either the buyer or the arbiter may release.
    pub fn release(env: Env, caller: Address) -> Result<(), EscrowError> {
        caller.require_auth();
        let mut data: EscrowData = env
            .storage()
            .instance()
            .get(&DataKey::Escrow)
            .ok_or(EscrowError::NotFound)?;

        if data.state != EscrowState::Created {
            return Err(EscrowError::AlreadySettled);
        }
        if env.ledger().sequence() > data.expiry_ledger {
            return Err(EscrowError::Expired);
        }
        if caller != data.buyer && caller != data.arbiter {
            return Err(EscrowError::Unauthorized);
        }

        data.state = EscrowState::Released;
        env.storage().instance().set(&DataKey::Escrow, &data);
        env.events()
            .publish((symbol_short!("RELEASED"),), (data.seller, data.amount));
        Ok(())
    }

    /// Cancel the escrow after expiry, returning funds to the buyer.
    pub fn cancel(env: Env) -> Result<(), EscrowError> {
        let mut data: EscrowData = env
            .storage()
            .instance()
            .get(&DataKey::Escrow)
            .ok_or(EscrowError::NotFound)?;

        if data.state != EscrowState::Created {
            return Err(EscrowError::AlreadySettled);
        }
        if env.ledger().sequence() <= data.expiry_ledger {
            return Err(EscrowError::NotExpired);
        }

        data.state = EscrowState::Cancelled;
        env.storage().instance().set(&DataKey::Escrow, &data);
        env.events()
            .publish((symbol_short!("CANCEL"),), (data.buyer, data.amount));
        Ok(())
    }

    /// Return the current escrow data.
    pub fn get(env: Env) -> Result<EscrowData, EscrowError> {
        env.storage()
            .instance()
            .get(&DataKey::Escrow)
            .ok_or(EscrowError::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{
        testutils::{Address as _, Ledger, LedgerInfo},
        Env,
    };

    fn setup(expiry: u32) -> (Env, Address, Address, Address, EscrowContractClient<'static>) {
        let env = Env::default();
        env.mock_all_auths();
        let buyer = Address::generate(&env);
        let seller = Address::generate(&env);
        let arbiter = Address::generate(&env);
        let contract_id = env.register(EscrowContract, ());
        let client = EscrowContractClient::new(&env, &contract_id);
        client.create(&buyer, &seller, &arbiter, &1000, &expiry).unwrap();
        (env, buyer, seller, arbiter, client)
    }

    #[test]
    fn test_create_and_get() {
        let (_, buyer, _, _, client) = setup(100);
        let data = client.get().unwrap();
        assert_eq!(data.buyer, buyer);
        assert_eq!(data.amount, 1000);
        assert_eq!(data.state, EscrowState::Created);
    }

    #[test]
    fn test_buyer_can_release() {
        let (_, buyer, _, _, client) = setup(100);
        client.release(&buyer).unwrap();
        assert_eq!(client.get().unwrap().state, EscrowState::Released);
    }

    #[test]
    fn test_arbiter_can_release() {
        let (_, _, _, arbiter, client) = setup(100);
        client.release(&arbiter).unwrap();
        assert_eq!(client.get().unwrap().state, EscrowState::Released);
    }

    #[test]
    fn test_seller_cannot_release() {
        let (_, _, seller, _, client) = setup(100);
        assert!(client.try_release(&seller).is_err());
    }

    #[test]
    fn test_cancel_after_expiry() {
        let (env, _, _, _, client) = setup(10);
        // Advance ledger past expiry
        env.ledger().set(LedgerInfo {
            sequence_number: 20,
            ..env.ledger().get()
        });
        client.cancel().unwrap();
        assert_eq!(client.get().unwrap().state, EscrowState::Cancelled);
    }

    #[test]
    fn test_cancel_before_expiry_fails() {
        let (_, _, _, _, client) = setup(100);
        assert!(client.try_cancel().is_err());
    }

    #[test]
    fn test_double_release_fails() {
        let (_, buyer, _, _, client) = setup(100);
        client.release(&buyer).unwrap();
        assert!(client.try_release(&buyer).is_err());
    }
}
