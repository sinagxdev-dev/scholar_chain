# ScholarChain PH

> On-chain scholarship disbursement and credential verification for Filipino students — built on Stellar Soroban.

**Developer:** John Ray Cacananta 
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

In the Philippines, the "State University and College (SUC)" system serves millions of students, many of whom rely entirely on government stipends (CHED, DOST, or LGU grants) to survive. However, the path from enrollment to disbursement is broken by a manual verification bottleneck:
- The 1.7 Billion Peso Gap: According to Commission on Audit (COA) reports, over ₱1.748 billion in scholarship funds often remains unreleased due to administrative delays.
- Manual Validation: SUC registrars must manually verify thousands of physical "Certificates of Enrollment" before funds can be moved. This process takes 1 to 12 months.
- The Student Toll: For scholars in state universities, these delays lead to "delayed degrees," forced student loans, and increased dropout rates as students lose the financial focus needed to stay in school.

---

## Solution

ScholarChain PH provides a decentralized infrastructure that automates the trust between State Universities, Government Agencies, and the Student. By leveraging Soroban Smart Contracts on the Stellar Network, we replace months of paperwork with seconds of code:
- Digital Fingerprinting: SUC Registrars upload a SHA-256 hash (a digital fingerprint) of a student's validated enrollment directly to the Stellar blockchain.
- Instant Smart Verification: The funding agency (CHED/DOST) no longer needs to wait for physical mail. The smart contract automatically verifies the student’s status against the on-chain hash.
- Programmable Disbursement: Upon successful verification, the contract releases funds in USDC (or a Philippine Peso stablecoin) directly to the student's Freighter Wallet.
- Public Accountability: Every peso moved is recorded on the public ledger (Stellar Expert), ensuring that the ₱1.7B "funding gap" is closed through real-time, transparent auditing.

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
stellar contract deploy `
  --wasm target\wasm32v1-none\release\scholarchain_ph.wasm `
  --source sinag `
  --network testnet

# Initialize (run once after deploy)
stellar contract invoke `
  --id <YOUR_NEW_CONTRACT_ID> `
  --source sinag `
  --network testnet `
  -- initialize `
  --admin <YOUR_G_ADDRESS_FROM_FREIGHTER> `
  --usdc_token CB763353B0D2AA246B5C5C5C5C5C5C5C5C5C5C5C5C5C5C5C5C5C5C5C5C5C5C5C `
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
<img width="1838" height="867" alt="image" src="https://github.com/user-attachments/assets/47205e0e-d212-4e69-8bc0-4707525a0864" />

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
