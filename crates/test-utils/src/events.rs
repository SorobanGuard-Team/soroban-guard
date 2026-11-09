use soroban_sdk::{testutils::Events, Env, Val, Vec as SorobanVec};

/// Returns the number of events emitted in this test environment.
pub fn event_count(env: &Env) -> usize {
    env.events().all().len() as usize
}

/// Assert that exactly `n` events were emitted.
pub fn assert_event_count(env: &Env, n: usize) {
    let count = event_count(env);
    assert_eq!(
        count, n,
        "Expected {n} events but found {count}"
    );
}

/// Assert that at least one event was emitted.
pub fn assert_has_events(env: &Env) {
    assert!(
        event_count(env) > 0,
        "Expected at least one event but none were emitted"
    );
}
