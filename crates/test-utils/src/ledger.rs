use soroban_sdk::{
    testutils::{Ledger, LedgerInfo},
    Env,
};

/// Set the ledger timestamp (useful for testing time-sensitive logic).
pub fn set_time(env: &Env, timestamp: u64) {
    env.ledger().set(LedgerInfo {
        timestamp,
        protocol_version: env.ledger().get().protocol_version,
        sequence_number: env.ledger().get().sequence_number,
        network_id: env.ledger().get().network_id,
        base_reserve: env.ledger().get().base_reserve,
        min_temp_entry_ttl: env.ledger().get().min_temp_entry_ttl,
        min_persistent_entry_ttl: env.ledger().get().min_persistent_entry_ttl,
        max_entry_ttl: env.ledger().get().max_entry_ttl,
    });
}

/// Advance the ledger by `seconds`.
pub fn advance_time(env: &Env, seconds: u64) {
    let current = env.ledger().get().timestamp;
    set_time(env, current + seconds);
}

/// Advance the ledger sequence number by `n`.
pub fn advance_sequence(env: &Env, n: u32) {
    let info = env.ledger().get();
    env.ledger().set(LedgerInfo {
        sequence_number: info.sequence_number + n,
        ..info
    });
}
