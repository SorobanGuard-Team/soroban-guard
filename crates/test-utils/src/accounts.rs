use soroban_sdk::{Address, Env};

/// A named test account with a generated address.
pub struct TestAccount {
    pub name: &'static str,
    pub address: Address,
}

impl TestAccount {
    /// Create a named test account with a freshly generated address.
    pub fn new(env: &Env, name: &'static str) -> Self {
        Self {
            name,
            address: Address::generate(env),
        }
    }
}

/// Create a named set of test accounts in one call.
///
/// # Example
/// ```rust,no_run
/// use soroban_sdk::Env;
/// use sorobanguard_test_utils::accounts::named_accounts;
///
/// let env = Env::default();
/// let accounts = named_accounts(&env, &["alice", "bob", "carol"]);
/// ```
pub fn named_accounts<'a>(env: &Env, names: &[&'static str]) -> Vec<TestAccount> {
    names.iter().map(|&name| TestAccount::new(env, name)).collect()
}
