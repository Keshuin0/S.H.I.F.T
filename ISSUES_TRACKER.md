# S.H.I.F.T. — Issues Tracker & Gap Analysis

**Generated:** 2026-05-28 | **Source:** https://github.com/Keshuin0/S.H.I.F.T/issues
**Total GitHub Items:** 96 (Issues + PRs) | **Open Issues:** 55 | **Closed Issues:** 20 | **PRs:** 8

---

## Table of Contents

1. [GitHub Issues — Full Inventory](#1-github-issues--full-inventory)
2. [Audit-Discovered Issues NOT on GitHub](#2-audit-discovered-issues-not-on-github)
3. [Gap Analysis — What's Missing](#3-gap-analysis--whats-missing)
4. [Recommended New GitHub Issues](#4-recommended-new-github-issues)

---

## 1. GitHub Issues — Full Inventory

### Phase 1: Hardware Abstraction & Identity (The Root of Trust)

| # | Title | State | Parent | Sub-Issues |
|---|-------|-------|--------|------------|
| 1 | Phase 1: Hardware Abstraction & Identity (The Root of Trust) | 🟢 OPEN | — | Top-level epic |
| 13 | 1.1 TEE Integration | 🟢 OPEN | #1 | #14 ✅, #15 ✅, #16 ✅, #17 🟢 |
| 14 | The Native Pipeline | ✅ CLOSED | #13 | — |
| 15 | The JNI Handshake | ✅ CLOSED | #13 | — |
| 16 | ARM TrustZone Connection | ✅ CLOSED | #13 | — |
| 17 | iOS / Apple Secure Enclave | 🟢 OPEN | #13 | Not started |
| 19 | 1.2 Sovereign Key Generation | ✅ CLOSED | #1 | — |
| 20 | 1.3 PoL (Proof of Location) Oracles | 🟢 OPEN | #1 | #21 ✅, #22 ✅, #23 ✅, #24 🟢, #25 🟢 |
| 21 | GPS & Cellular Data | ✅ CLOSED | #20 | — |
| 22 | Cryptographic Binding | ✅ CLOSED | #20 | — |
| 23 | Missing BLE Telemetry | ✅ CLOSED | #20 | — |
| 24 | Not "Zero-Knowledge" | 🟢 OPEN | #20 | PoL hashing uses DefaultHasher, not ZK |
| 25 | Not Routed "Directly" into the TEE (The Hardware Limitation) | 🟢 OPEN | #20 | — |
| 26 | 1.4 Biometric ZK Identity (Soulbound Tokens) | 🟢 OPEN | #1 | #27 ✅, #28 ✅, #29 🟢 |
| 27 | Biometric Prompting | ✅ CLOSED | #26 | — |
| 28 | Biometrics are in the OS, NOT the TEE | ✅ CLOSED | #26 | — |
| 29 | Missing the Soulbound Token (KYC State) | 🟢 OPEN | #26 | SBT never written to OnceLock |
| 30 | 1.5 Proximity Triangulation (Anti-Spoofing) | ✅ CLOSED | #1 | #31 ✅, #32 ✅, #33 ✅ |
| 31 | The Kotlin Pipe | ✅ CLOSED | #30 | — |
| 32 | The Rust Consensus Integration | ✅ CLOSED | #30 | — |
| 33 | The Mathematical Rejection Engine | ✅ CLOSED | #30 | — |
| 86 | TEE Hypervisor Passthrough & ZK Distance Bounding | 🟢 OPEN | #25 | #87 🟢, #88 🟢, #89 🟢 |
| 87 | Implement AVF/pKVM Peripheral Passthrough | 🟢 OPEN | #86 | — |
| 88 | Develop TEE-to-TEE Cryptographic Ranging Protocol | 🟢 OPEN | #86 | — |
| 89 | Build ZK-SNARK Circuit for ToF Aggregation (Sub-50ms) | 🟢 OPEN | #86 | — |
| 90 | 1.6: SELinux Zero-Trust Enclave (Isolated Process) | 🟢 OPEN | #1 | Alternative to pKVM for non-AVF devices |

### Phase 2: Mesh Networking & Spatial Indexing (The P2P Layer)

| # | Title | State | Parent | Notes |
|---|-------|-------|--------|-------|
| 2 | Phase 2: Mesh | 🟢 OPEN | — | Top-level epic |
| 34 | 2.1 Radio Mesh Deployment | 🟢 OPEN | #2 | #35 ✅, #36 ✅, #37 ✅, #38 🟢, #39 🟢, #40 🟢, #41 🟢 |
| 35 | GossipSub Protocol | ✅ CLOSED | #34 | — |
| 36 | QUIC (UDP) | ✅ CLOSED | #34 | — |
| 37 | The Asynchronous Chasm | ✅ CLOSED | #34 | Tokio runtime injected |
| 38 | The Network Standard | 🟢 OPEN | #34 | — |
| 39 | Wi-Fi Aware (Device-to-Device) | 🟢 OPEN | #34 | No code exists |
| 40 | True NAT Traversal | 🟢 OPEN | #34 | DCUtR configured but untested |
| 41 | libp2p Rendezvous Protocol | 🟢 OPEN | #34 | — |
| 42 | 2.2 Kademlia DHT Setup | ✅ CLOSED | #2 | — |
| 43 | 2.3 H3 Hexagonal Spatial Indexing | ✅ CLOSED | #2 | — |
| 44 | 2.4 Sub-50ms Proximity Queries | ✅ CLOSED | #2 | #45 ✅, #46 ✅, #47 ✅, #48 ✅ |
| 45 | The Spatial Shard (Zero-Hop Architecture) | ✅ CLOSED | #44 | — |
| 46 | The K-Ring Expansion (The Radar) | ✅ CLOSED | #44 | — |
| 47 | The 50ms Strike | ✅ CLOSED | #44 | — |
| 48 | Optimistic Concurrency Control (OCC) | ✅ CLOSED | #44 | — |
| 49 | 2.5 Dead Zone Architecture | 🟢 OPEN | #2 | #50-#56 all OPEN |
| 50 | The Multi-Sig Anchor | 🟢 OPEN | #49 | — |
| 51 | The BLE Beacon (The Handshake) | 🟢 OPEN | #49 | — |
| 52 | The Wi-Fi Aware Tunnel (The Payload) | 🟢 OPEN | #49 | — |
| 53 | The Direct Tether (In the Dark) | 🟢 OPEN | #49 | — |
| 54 | The Fallback | 🟢 OPEN | #49 | — |
| 55 | Hardware Co-Signing | 🟢 OPEN | #49 | — |
| 56 | The Asynchronous Sling-Shot (Settlement) | 🟢 OPEN | #49 | — |
| 91 | 2.6 Async Hardware-Attested State Channels | 🟢 OPEN | #2 | — |
| 92 | Build BLE Handshake to trigger Wi-Fi Aware (NAN) | 🟢 OPEN | #49 | — |
| 93 | Implement Wi-Fi Direct tether | 🟢 OPEN | #49 | — |
| 94 | Code TrustZone co-signing for Final State Receipts | 🟢 OPEN | #49 | — |

### Phase 3: The Block-Lattice & Ledger (Financial Settlement)

| # | Title | State | Parent | Notes |
|---|-------|-------|--------|-------|
| 3 | Phase 3: Block-Lattice & Ledger | 🟢 OPEN | — | Top-level epic |
| 57 | 3.1 Block-Lattice Architecture | 🟢 OPEN | #3 | #62 ✅, #63 🟢, #64 🟢 |
| 58 | 3.5 S.H.I.F.T. Name Service (SNS) | 🟢 OPEN | #3 | — |
| 59 | 3.2 Nova/SuperNova ZK Folding | 🟢 OPEN | #3 | — |
| 60 | 3.3 Tx-PoW (Spam Protection) | 🟢 OPEN | #3 | — |
| 61 | 3.4 The Burn Mechanism | 🟢 OPEN | #3 | — |
| 62 | The Sovereign Lattice (The Data Structure) | ✅ CLOSED | #57 | StateBlock struct defined |
| 63 | Verkle Trees (The Stateless State) | 🟢 OPEN | #57 | — |
| 64 | Nova IVC Folding (The Compression) | 🟢 OPEN | #57 | — |

### Phase 4: Execution, Economics, and AI Governance (The Brain)

| # | Title | State | Parent | Notes |
|---|-------|-------|--------|-------|
| 4 | Phase 4: Execution, Economics, AI Governance | 🟢 OPEN | — | Top-level epic |
| 65 | 4.1 On-Device zkVM | ✅ CLOSED | #4 | — |
| 66 | 4.2 The AI Brain (zkML Integration) | 🟢 OPEN | #4 | — |
| 67 | 4.3 Hybrid Market-Maker Pricing | 🟢 OPEN | #4 | #80 ✅, #81 ✅, #82 ✅, #83 🟢, #84 🟢 |
| 68 | 4.4 "Airport Mafia" Slashing | 🟢 OPEN | #4 | — |
| 69 | 4.5 DeFi Insurance Treasury | 🟢 OPEN | #4 | — |
| 80 | The zkVM is Ignited | ✅ CLOSED | #67 | — |
| 81 | System 1 is Defined (The AI Surge) | ✅ CLOSED | #67 | — |
| 82 | Flaw 1 (The Algorithmic Floor) | ✅ CLOSED | #67 | — |
| 83 | Flaw 2: Automated Slippage | 🟢 OPEN | #67 | — |
| 84 | Flaw 3: The Phantom Demand Exploit | 🟢 OPEN | #67 | — |

### Phase 5: Real-World Safety & App UX

| # | Title | State | Parent | Notes |
|---|-------|-------|--------|-------|
| 5 | Phase 5: Real-World Safety & App UX | 🟢 OPEN | — | Top-level epic |
| 70 | 5.1 The Decentralized SOS Button | 🟢 OPEN | #5 | — |
| 71 | 5.2 Trustless Dispute Arbitration | 🟢 OPEN | #5 | — |
| 72 | 5.3 Web2-Style UI/UX Abstraction | 🟢 OPEN | #5 | — |
| 95 | Web2-Style UI/UX | 🟢 OPEN | #72 | Remove manual "Execute" buttons |

### DevOps & Infrastructure

| # | Title | State | Parent | Notes |
|---|-------|-------|--------|-------|
| 75 | DevOps & Infra: Autonomous AI Matrix | 🟢 OPEN | — | — |
| 76 | Component 1: The Builder (Copilot Workspace) | 🟢 OPEN | #75 | — |
| 77 | Component 2: The Gatekeeper (Gemini 3.1 Pro) | 🟢 OPEN | #75 | — |

---

## 2. Audit-Discovered Issues NOT on GitHub

These are **critical bugs and gaps** found during the codebase deep-dive that have **NO corresponding GitHub issue**:

### 🔴 CRITICAL — Not Tracked Anywhere

| ID | Severity | Title | File | Lines | Description |
|----|----------|-------|------|-------|-------------|
| A1 | 🔴 CRITICAL | **Missing VSOCK Command Handlers** | `main.rs` | 445-467 | `MINT_GENESIS:`, `FIRE_LOCK:`, and `ISSUE_SBT:` are called from Kotlin `MainActivity.kt` but have **no handler** in the Rust Vault. They silently fail with "Unrecognized command." This means Block-Lattice genesis, OCC ride locking, and Soulbound Token issuance are completely non-functional. |
| A2 | 🔴 CRITICAL | **ZK Distance Bounding Circuit Missing Inequality Constraint** | `zk_engine.rs` | 65-101 | The `DistanceBoundingCircuit` computes `round_trip_distance` and `max_round_trip` but **never enforces** `round_trip_distance ≤ max_round_trip`. The Groth16 proof proves the arithmetic is correct but does NOT prove the node is within the allowed distance. The proof is valid regardless of physical distance. |
| A3 | 🔴 CRITICAL | **Ranging is Entirely Simulated** | `main.rs` | 411-420 | The cryptographic distance bounding creates a **dummy peer keypair locally** and fabricates the entire challenge-response. `simulated_rx_time` is artificially set to `tx_timestamp + compute_delay + 100ns`. No actual over-the-air BLE/UWB ranging occurs. |
| A4 | 🔴 CRITICAL | **Ephemeral libp2p Identity** | `main.rs` | 200-210 | A new Ed25519 keypair is generated on every `REGISTER_NODE` call. The node's PeerId changes every boot, destroying DHT routing tables, peer reputation history, and network identity continuity. |
| A5 | 🔴 CRITICAL | **Groth16 Trusted Setup Runs Per-PoL** | `main.rs` | 425-435 | `generate_tof_proof()` calls `Groth16::circuit_specific_setup()` on every Proof-of-Location generation. Trusted setup is computationally expensive (seconds on mobile). Should be done once and the proving/verification keys cached. |

### 🟡 MAJOR — Not Tracked Anywhere

| ID | Severity | Title | File | Lines | Description |
|----|----------|-------|------|-------|-------------|
| A6 | 🟡 MAJOR | **VSOCK Uses TcpStream::from_raw_fd() on Non-TCP Socket** | `main.rs` | 155-175 | The VSOCK bridge wraps an `AF_VSOCK` file descriptor in `std::net::TcpStream`. This works for read/write but is semantically incorrect. TCP-specific socket options (Nagle, keepalive, timeouts) may cause unexpected behavior. |
| A7 | 🟡 MAJOR | **No Peer Bootstrapping** | `main.rs` | 240-250 | The mobile client has no hardcoded relay/bootnode multiaddrs. Without mDNS (LAN-only) or a bootstrap list, the node cannot discover peers on the public internet. The deleted bootnode is the only reference implementation, but it's been archived. |
| A8 | 🟡 MAJOR | **DefaultHasher for PoL Hashing** | `main.rs` | 390-405 | `std::hash::DefaultHasher` (SipHash) is used for PoL "cryptographic binding." SipHash is fast for hash tables but is NOT cryptographically secure. The README promises cryptographic binding but any node can trivially replicate the hash. |
| A9 | 🟡 MAJOR | **Block-Lattice Is Data Structure Only** | `main.rs` | 62-70 | `StateBlock` and `LOCAL_LEDGER` are defined but no functions for minting genesis blocks, creating send/receive blocks, validating chains, or computing balances exist. |
| A10 | 🟡 MAJOR | **BLE Mesh Has No S.H.I.F.T. Service UUID** | `MainActivity.kt` | 333-353 | The BLE scanner collects MAC addresses from **all** nearby Bluetooth devices (smartwatches, headphones, TVs). Without a custom GATT service UUID in the advertisement, there's no way to distinguish S.H.I.F.T. peers from random BLE devices. |
| A11 | 🟡 MAJOR | **No VSOCK Authentication** | `main.rs` | 129-185 | Any process that can connect to VSOCK port 8000 can issue commands to the Rust Vault. There's no challenge-response, HMAC, or session token to verify the caller is the legitimate Kotlin host. |
| A12 | 🟡 MAJOR | **SOULBOUND_TOKEN Never Enforced** | `main.rs` | 48-50 | The Rust Vault processes `GENERATE_POL` commands without checking if `SOULBOUND_TOKEN` has been set. Any unverified node can generate location proofs without KYC clearance. |
| A13 | 🟡 MAJOR | **Thread Safety: nearbyNodes** | `MainActivity.kt` | 98 | `mutableSetOf<String>()` is not thread-safe. BLE scan callback writes from a background thread while the UI thread reads, creating a potential ConcurrentModificationException. |

### 🟠 MINOR — Not Tracked Anywhere

| ID | Severity | Title | File | Description |
|----|----------|-------|------|-------------|
| A14 | 🟠 MINOR | **Unused activity_main.xml** | `res/layout/` | Leftover "Hello World" layout from project creation. UI is built programmatically. |
| A15 | 🟠 MINOR | **Dead Code: native-lib.cpp** | `cpp/` | C++ JNI stub "Hello from C++" is never called. CMakeLists.txt still builds it. |
| A16 | 🟠 MINOR | **Vestigial jni Crate in Cargo.toml** | `Cargo.toml` | `jni = "0.21.1"` is still in dependencies but no JNI functions exist in the binary. |
| A17 | 🟠 MINOR | **Duplicate libif_watch .so Files** | `jniLibs/` | Three copies of `libif_watch-*.so` exist. Only one is needed. |
| A18 | 🟠 MINOR | **Hardcoded Fallback GPS Coordinates** | `MainActivity.kt` | L391 | Falls back to `46.2382, -63.1311` (Prince Edward Island) when GPS unavailable. Should be handled as an error, not a silent fallback. |
| A19 | 🟠 MINOR | **Gemini Gatekeeper Model Name May Be Invalid** | `gemini-gatekeeper.yml` | `gemini-3.1-pro-preview` may not be a real endpoint. Failure causes the catch block to post "REJECT: Neural Engine Failure" which blocks all PRs. |
| A20 | 🟠 MINOR | **Ranging Distance Calculation Off By Factor of 2** | `ranging.rs` | L78 | `distance_mm = t_flight * 300` reports round-trip distance, not one-way. The factor-of-2 division is handled separately in `zk_engine.rs`, but this makes `ranging.rs` output misleading in isolation. |

---

## 3. Gap Analysis — What's Missing

### Cross-Reference: Audit Issues vs. Existing GitHub Issues

| Audit Issue | Related GitHub Issue | Gap Status |
|-------------|---------------------|------------|
| A1: Missing VSOCK Handlers | #29 (Missing SBT) partially covers ISSUE_SBT | **MINT_GENESIS and FIRE_LOCK have NO issue** |
| A2: ZK Circuit Missing Constraint | #89 (Build ZK-SNARK for ToF) | Issue exists but **doesn't mention the missing inequality constraint** — it's a task for building the circuit, not fixing the existing broken one |
| A3: Simulated Ranging | #88 (TEE-to-TEE Ranging Protocol) | Issue exists for the **target** protocol but **doesn't acknowledge current simulation** |
| A4: Ephemeral libp2p Identity | None | **NO ISSUE EXISTS** |
| A5: Trusted Setup Per-PoL | None | **NO ISSUE EXISTS** |
| A6: VSOCK TcpStream Hack | None | **NO ISSUE EXISTS** |
| A7: No Peer Bootstrapping | #41 (Rendezvous Protocol) covers dynamic discovery but **no bootstrap list issue** | **PARTIALLY COVERED** |
| A8: DefaultHasher | #24 (Not "Zero-Knowledge") | **COVERED** — issue acknowledges PoL isn't ZK |
| A9: Block-Lattice Stub Only | #57 (Block-Lattice Architecture) | **COVERED** — epic is open with sub-issues |
| A10: BLE No Service UUID | None | **NO ISSUE EXISTS** |
| A11: No VSOCK Authentication | None | **NO ISSUE EXISTS** |
| A12: SBT Never Enforced | #29 (Missing SBT KYC State) | **COVERED** |
| A13: Thread Safety nearbyNodes | None | **NO ISSUE EXISTS** |
| A14-A20: Minor Cleanup | None | **NO ISSUES EXIST** |

### Summary: Issues That Need to Be Filed on GitHub

**🔴 Critical (must file):**
- A1 (partial) — MINT_GENESIS and FIRE_LOCK missing handlers
- A4 — Ephemeral libp2p identity
- A5 — Groth16 trusted setup runs per-PoL

**🟡 Major (should file):**
- A6 — VSOCK TcpStream semantic mismatch
- A10 — BLE missing S.H.I.F.T. service UUID
- A11 — No VSOCK authentication
- A13 — Thread-unsafe nearbyNodes set

**🟠 Minor (nice to file):**
- A14-A20 — Cleanup and dead code removal

**Already covered by existing issues (no action needed):**
- A2 → partially by #89
- A3 → partially by #88
- A7 → partially by #41
- A8 → by #24
- A9 → by #57
- A12 → by #29

---

## 4. Recommended New GitHub Issues

### Issue Template: A1 — MINT_GENESIS & FIRE_LOCK Missing Handlers

**Title:** `[BUG] VSOCK command handlers missing for MINT_GENESIS and FIRE_LOCK`
**Labels:** `Phase 1`, `Phase 3`, `bug`
**Body:**

> The Kotlin UI (MainActivity.kt) sends `MINT_GENESIS:` and `FIRE_LOCK:` commands via VSOCK to the Rust Vault, but `process_vault_command()` in `main.rs` has no handler for either command.
>
> Both fall through to the `else` clause and return `"Unrecognized or deprecated command."`, silently breaking Block-Lattice genesis and OCC ride locking.
>
> **Files affected:** `shift_core/src/main.rs` (L445-467), `MainActivity.kt` (L130, L140)
>
> **Fix:** Implement `MINT_GENESIS:` handler to create a genesis StateBlock in LOCAL_LEDGER, and `FIRE_LOCK:` handler to broadcast a LOCK_REQUEST via GossipSub.

---

### Issue Template: A4 — Ephemeral libp2p Identity

**Title:** `[BUG] libp2p PeerId changes on every boot — breaks DHT routing`
**Labels:** `Phase 2`, `bug`
**Body:**

> In `main.rs` line ~200, `REGISTER_NODE` generates a fresh `identity::Keypair::generate_ed25519()` on every call. This means the node's PeerId changes on every app restart.
>
> **Impact:** DHT routing tables become invalid, peer reputation is lost, relay reservations expire, and the node is treated as a brand-new participant after every reboot.
>
> **Fix:** Derive the libp2p keypair deterministically from the TEE's StrongBox ECDSA key, OR persist the Ed25519 key in the pKVM encrypted filesystem.

---

### Issue Template: A5 — Groth16 Setup Per-PoL

**Title:** `[PERF] Groth16 trusted setup runs on every Proof-of-Location`
**Labels:** `Phase 1`, `performance`
**Body:**

> `generate_tof_proof()` in `zk_engine.rs` calls `Groth16::<Bls12_381>::circuit_specific_setup()` every time a PoL is generated. Trusted setup involves elliptic curve pairings and is extremely expensive on mobile hardware (multi-second delays).
>
> **Fix:** Run trusted setup once at node initialization, cache the `ProvingKey` and `VerifyingKey` in a global `OnceLock`, and reuse them for all subsequent proofs.

---

### Issue Template: A10 — BLE Missing Service UUID

**Title:** `[BUG] BLE mesh scanner collects all nearby Bluetooth devices, not just S.H.I.F.T. nodes`
**Labels:** `Phase 1`, `bug`
**Body:**

> `startBleMesh()` in `MainActivity.kt` starts BLE scanning and advertising without a custom GATT service UUID or manufacturer-specific data. The scan collects MAC addresses from ALL nearby BLE devices (headphones, smartwatches, TVs, etc.).
>
> **Impact:** The zk-PSI rejection engine and peer count are polluted with non-S.H.I.F.T. devices, making proximity triangulation unreliable.
>
> **Fix:** Define a S.H.I.F.T. custom 128-bit service UUID. Add it to `AdvertiseData` and filter `ScanSettings` to only match that UUID.

---

### Issue Template: A11 — No VSOCK Authentication

**Title:** `[SECURITY] No authentication on VSOCK bridge — any process can control the Vault`
**Labels:** `Phase 1`, `security`
**Body:**

> The VSOCK listener in `main.rs` accepts any connection to port 8000 and processes commands without verifying the caller's identity. On a rooted device, any process can open a VSOCK connection and send commands like `REGISTER_NODE` or `GENERATE_POL`.
>
> **Fix:** Implement a session handshake where the Kotlin host proves possession of the StrongBox key before the Vault accepts commands. Consider HMAC-signed command packets.

---

*This file is a living document. Update it as issues are filed on GitHub and resolved in code.*
