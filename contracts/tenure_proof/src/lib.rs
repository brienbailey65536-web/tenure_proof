#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

/// A single tenure attestation issued by an employer about a worker.
/// `end == 0` denotes a currently ongoing engagement.
#[contracttype]
#[derive(Clone)]
pub struct TenureRecord {
    pub employer: Address,
    pub worker: Address,
    pub role: Symbol,
    pub start: u64,
    pub end: u64,
    pub revoked: bool,
    pub reason: Symbol,
    pub recorded_at: u64,
}

/// Persistent storage key layout.
#[contracttype]
pub enum DataKey {
    /// (worker, index) -> TenureRecord
    Record(Address, u32),
    /// worker -> u32 (number of records ever recorded for this worker)
    Count(Address),
}

/// Status codes returned by `verify`.
/// 0 = NOT_FOUND, 1 = VALID_CURRENT, 2 = VALID_PAST, 3 = REVOKED.
pub const STATUS_NOT_FOUND: u32 = 0;
pub const STATUS_VALID_CURRENT: u32 = 1;
pub const STATUS_VALID_PAST: u32 = 2;
pub const STATUS_REVOKED: u32 = 3;

#[contract]
pub struct TenureProof;

#[contractimpl]
impl TenureProof {
    /// Record a new tenure attestation for a worker.
    ///
    /// Only the `employer` (HR or manager wallet) may call this; the call
    /// must be signed by `employer`. `end == 0` marks an ongoing engagement.
    /// Returns the newly-assigned record index for that worker.
    pub fn record_tenure(
        env: Env,
        employer: Address,
        worker: Address,
        role: Symbol,
        start: u64,
        end: u64,
    ) -> u32 {
        employer.require_auth();

        if end != 0 && end < start {
            panic!("end before start");
        }

        let count_key = DataKey::Count(worker.clone());
        let index: u32 = env
            .storage()
            .persistent()
            .get(&count_key)
            .unwrap_or(0u32);

        let record = TenureRecord {
            employer: employer.clone(),
            worker: worker.clone(),
            role,
            start,
            end,
            revoked: false,
            reason: Symbol::new(&env, "none"),
            recorded_at: env.ledger().timestamp(),
        };

        env.storage()
            .persistent()
            .set(&DataKey::Record(worker, index), &record);
        env.storage().persistent().set(&count_key, &(index + 1));

        index
    }

    /// Revoke an existing tenure record. Only the original issuing
    /// employer may revoke, and only if the record is not already revoked.
    /// `reason` is stored on-chain alongside the record.
    pub fn revoke_tenure(
        env: Env,
        employer: Address,
        worker: Address,
        index: u32,
        reason: Symbol,
    ) {
        employer.require_auth();

        let key = DataKey::Record(worker, index);
        let mut record: TenureRecord = env
            .storage()
            .persistent()
            .get(&key)
            .expect("record not found");

        if record.employer != employer {
            panic!("not original employer");
        }
        if record.revoked {
            panic!("already revoked");
        }

        record.revoked = true;
        record.reason = reason;
        env.storage().persistent().set(&key, &record);
    }

    /// Verify the status of a tenure record.
    ///
    /// Returns one of: `STATUS_NOT_FOUND` (0), `STATUS_VALID_CURRENT` (1),
    /// `STATUS_VALID_PAST` (2), or `STATUS_REVOKED` (3). No authorization
    /// is required — this is a public verifier-facing read.
    pub fn verify(env: Env, worker: Address, index: u32) -> u32 {
        let record: Option<TenureRecord> = env
            .storage()
            .persistent()
            .get(&DataKey::Record(worker, index));

        match record {
            None => STATUS_NOT_FOUND,
            Some(r) => {
                if r.revoked {
                    STATUS_REVOKED
                } else if r.end == 0 {
                    STATUS_VALID_CURRENT
                } else {
                    STATUS_VALID_PAST
                }
            }
        }
    }

    /// View the employer (issuer) address of a tenure record.
    pub fn get_employer(env: Env, worker: Address, index: u32) -> Address {
        let record: TenureRecord = env
            .storage()
            .persistent()
            .get(&DataKey::Record(worker, index))
            .expect("record not found");
        record.employer
    }

    /// View the role / job title stored in a tenure record.
    pub fn get_role(env: Env, worker: Address, index: u32) -> Symbol {
        let record: TenureRecord = env
            .storage()
            .persistent()
            .get(&DataKey::Record(worker, index))
            .expect("record not found");
        record.role
    }

    /// Total number of tenure records ever issued for `worker`
    /// (including revoked ones, since indices are never re-used).
    pub fn list_records(env: Env, worker: Address) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::Count(worker))
            .unwrap_or(0u32)
    }

    /// Returns `true` if the tenure record is non-revoked and still ongoing
    /// (`end == 0`), i.e. the worker is presently employed in that role.
    pub fn is_current(env: Env, worker: Address, index: u32) -> bool {
        let record: Option<TenureRecord> = env
            .storage()
            .persistent()
            .get(&DataKey::Record(worker, index));

        match record {
            None => false,
            Some(r) => !r.revoked && r.end == 0,
        }
    }
}
