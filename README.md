# ScholarChain PH

> On-chain scholarship disbursement and credential verification for Filipino students — built on Stellar Soroban.

**Developer:** John Ray Cacananta · National University Fairview  
**Event:** Stellar Philippines UniTour 2026 · Rise In Bootcamp  
**Network:** Stellar Testnet

---

## Live Links

| Resource | Link |
|---|---|
| 🌐 Live Frontend | [sinagxdev-dev.github.io/scholar_chain](https://sinagxdev-dev.github.io/scholar_chain) |
| 📦 Contract on Stellar Expert | [CCYASD3...EL5S](https://stellar.expert/explorer/testnet/contract/CCYASD3OJXWDSUMCLOQ246JB62DJAR6CABZZOLIHR7VG5YOQ3NPLEL5S) |
| 💻 GitHub Repo | [sinagxdev-dev/scholar_chain](https://github.com/sinagxdev-dev/scholar_chain) |

---

## Problem

A fourth-year BS Computer Science student at Polytechnic University of the Philippines receives a CHED/DOST scholarship stipend — but the ₱7,000/month disbursement is delayed **1 to 12 months** because universities manually verify enrollment status before releasing funds. The Commission on Audit found **₱1.748 billion** in scholarship funds unreleased, forcing scholars to skip meals, take on debt, and lose academic focus.

---

## Solution

ScholarChain PH replaces the manual bursar process with a Soroban smart contract on Stellar. A university registrar registers the student's enrollment credential (as a SHA-256 hash) on-chain. Upon successful verification, the contract releases USDC directly to the student's Freighter wallet — in under 5 seconds, with near-zero fees.

---

## Stellar Features Used

| Feature | Purpose |
|---|---|
| **Soroban smart contracts** | Enrollment gating + automatic USDC disbursement logic |
| **USDC transfers** | Stable, peso-equivalent scholarship payouts direct to student wallet |
| **Trustlines** | Student wallet opts in to receive scholarship USDC asset |
| **Custom tokens** | School-issued enrollment credential token for on-chain verification |

---

## Contract Details

| Field | Value |
|---|---|
| **Contract ID** | `CCYASD3OJXWDSUMCLOQ246JB62DJAR6CABZZOLIHR7VG5YOQ3NPLEL5S` |
| **Network** | Stellar Testnet |
| **Language** | Rust / Soroban SDK 21.0.0 |

---

## Contract Functions

| Function | Description |
|---|---|
| `initialize(admin, usdc_token, reward_amount)` | Sets up the contract — run once after deploy |
| `register_certificate(admin, student, doc_hash)` | Registers a student's enrollment credential on-chain |
| `reward_student(admin, student, doc_hash)` | Transfers USDC reward to student wallet |
| `verify_certificate(doc_hash)` | Returns `true` if credential is valid, emits event |
| `link_payment(employer, student, doc_hash, amount)` | Employer-triggered direct payment to verified student |
| `get_certificate(doc_hash)` | Read-only — returns full credential record |

---

## Repo Structure

```
scholar_chain/
├── src/
│   ├── lib.rs         ← Soroban smart contract
│   └── test.rs        ← 3 unit tests
├── Cargo.toml
├── index.html         ← Frontend (Freighter + Soroban)
└── README.md
```

---

## Prerequisites

- [Rust](https://rustup.rs/) with `wasm32-unknown-unknown` target
- [Stellar CLI](https://developers.stellar.org/docs/tools/stellar-cli) v21+
- [Freighter Wallet](https://freighter.app) browser extension — set to **Testnet**

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked stellar-cli --features opt
```

---

## Build & Test

```bash
# Build the Wasm binary
stellar contract build

# Run unit tests
cargo test
```

Expected test output:
```
running 3 tests
test tests::test_happy_path_register_and_reward_student ... ok
test tests::test_duplicate_certificate_rejected ... ok
test tests::test_state_verification_after_registration ... ok

test result: ok. 3 passed; 0 failed
```

---

## Deploy to Testnet

```bash
# Create and fund identity
stellar keys generate --global my-key --network testnet
stellar keys fund my-key --network testnet

# Deploy
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/scholar_chain_ph.wasm \
  --source my-key \
  --network testnet

# Initialize (run once after deploy)
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source my-key \
  --network testnet \
  -- initialize \
  --admin <YOUR_G_ADDRESS> \
  --usdc_token <USDC_TOKEN_ADDRESS> \
  --reward_amount 500000000
```

---

## Sample CLI Invocations

```bash
# Register a student certificate
stellar contract invoke \
  --id CCYASD3OJXWDSUMCLOQ246JB62DJAR6CABZZOLIHR7VG5YOQ3NPLEL5S \
  --source my-key \
  --network testnet \
  -- register_certificate \
  --admin <YOUR_G_ADDRESS> \
  --student <STUDENT_G_ADDRESS> \
  --doc_hash abababababababababababababababababababababababababababababababababab

# Verify a certificate
stellar contract invoke \
  --id CCYASD3OJXWDSUMCLOQ246JB62DJAR6CABZZOLIHR7VG5YOQ3NPLEL5S \
  --source my-key \
  --network testnet \
  -- verify_certificate \
  --doc_hash abababababababababababababababababababababababababababababababababab
```

---

## References

| Source | Finding |
|---|---|
| COA Audit Report | ₱1.748B in DOST-SEI scholarship funds unreleased; 1–12 month delays documented |
| Cebu City (2026) | Thousands of city scholars received zero allowance for an entire semester |
| UP Diliman Scholar | 2-month stipend delay forced students to skip meals and borrow from friends |
| CHED Citizen's Charter | HEIs legally required to credit scholarship funds within 15 working days |

---

## License

MIT — free to use, fork, and build on.
