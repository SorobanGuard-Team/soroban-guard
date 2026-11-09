use soroban_sdk::{Address, Env};

/// Thin wrapper around `soroban_sdk::Env` for convenient test setup.
pub struct TestEnv {
    pub inner: Env,
}

impl TestEnv {
    /// Create a fresh Soroban test environment.
    pub fn new() -> Self {
        Self {
            inner: Env::default(),
        }
    }

    /// Generate a random test address.
    pub fn generate_address(&self) -> Address {
        Address::generate(&self.inner)
    }

    /// Generate `n` random test addresses.
    pub fn generate_addresses(&self, n: usize) -> Vec<Address> {
        (0..n).map(|_| self.generate_address()).collect()
    }

    /// Return a reference to the underlying `Env`.
    pub fn env(&self) -> &Env {
        &self.inner
    }
}

impl Default for TestEnv {
    fn default() -> Self {
        Self::new()
    }
}
