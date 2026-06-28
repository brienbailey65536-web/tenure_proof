# tenure_proof

## Project Title
tenure_proof

## Project Description
`tenure_proof` is a Soroban smart contract on Stellar that turns employment
history into a tamper-evident, on-chain credential. An employer (HR system
or manager wallet) issues a signed tenure attestation — role, start date,
and optional end date — directly against the worker's address. The worker
can then present any record index to a future employer or background-check
agent, who reads the contract to confirm the role, the issuer, the dates,
and whether the credential is still active or has been revoked, without
ever calling the previous employer.

## Project Vision
Hiring today relies on PDFs, phone calls, and trust in third-party
background-check vendors who themselves cannot prove a record has not been
forged. Our vision is a portable, worker-owned employment graph: every
engagement a person has ever had becomes a verifiable credential anchored
to Stellar, revocable only by the original issuer and verifiable by anyone
in milliseconds. Over time this becomes the universal substrate for
reference checks, gig-economy reputation, professional licensing, and
cross-border hiring — replacing opaque centralised HR-tech databases with
a transparent, user-controlled identity primitive.

## Key Features
- **Issuer-signed attestations** — `record_tenure` requires `require_auth()`
  from the employer, so only the real HR/manager wallet can mint a record.
- **Ongoing vs. closed engagements** — `end == 0` is treated as a current
  job; `is_current` lets verifiers distinguish present-tense employment.
- **Revocation with reason** — the original employer (and only them) can
  call `revoke_tenure` to invalidate a record, with the reason stored
  on-chain for audit.
- **Public verification** — `verify` returns a compact status code
  (0 not-found / 1 current / 2 past / 3 revoked) for cheap third-party
  background checks.
- **Worker-indexed registry** — `list_records` plus `get_employer` /
  `get_role` let any verifier enumerate a worker's full tenure history
  without needing the employer to be online.

## Contract

- **Network:** Stellar Testnet (Public)
- **Scope:** identity dApp — see `contracts/tenure_proof/src/lib.rs` for the full tenure_proof business logic.
- **Functions exposed:** see `Key Features` above and the `pub fn` list in `lib.rs`.
- **Contract ID:** `CB3YZ5V6IIGDAPPFDW4H76JY76PHX25BUNGHMIDNN2OZORYFJPAYQ5CV`
- **Explorer template:** `https://stellar.expert/explorer/testnet/tx/b7fa4eb6c50831b550b51c99cd2b9f2ff79e91a5f1f8ce973e93223ea1e58d02`

## Future Scope
- **Worker co-signature mode** — optional second `require_auth()` from the
  worker so attestations are mutually consented, enabling true
  self-sovereign identity semantics.
- **Structured role taxonomy** — replace the free-form `Symbol` role with
  an ISCO / O*NET code registry contract for machine-readable job titles.
- **Selective disclosure / ZK proofs** — let workers prove "I held a Senior
  Engineer role for ≥ 3 years" without revealing the employer's identity.
- **Reputation aggregator** — a companion contract that computes
  aggregate metrics (total years in industry, number of revocations,
  longest tenure) directly from the on-chain history.
- **Cross-chain attestation bridge** — mirror credentials to EVM L2s via
  Stellar's interoperability layer so the same worker identity is portable
  across ecosystems.
- **Frontend dApp** — wallet-based UI for employers to issue records and
  for workers to share a one-click verification link with recruiters.

## Profile

- **Name:** <!-- Fill github name -->
- **Project:** `tenure_proof` (identity)
- **Built with:** Soroban SDK 25, Rust, Stellar Testnet
