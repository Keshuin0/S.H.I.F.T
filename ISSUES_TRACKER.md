# S.H.I.F.T. — Issues Tracker & Gap Analysis

**Generated:** 2026-05-28 | **Updated:** 2026-05-30 | **Source:** https://github.com/Keshuin0/S.H.I.F.T/issues
**Total GitHub Items:** 123 (Issues + PRs) | **Open Issues:** 69 | **Closed Issues:** 37 | **PRs:** 17
**Audit Issues Filed:** ✅ All 20 (A1-A20) — [View on GitHub](https://github.com/Keshuin0/S.H.I.F.T/issues?q=label%3Aaudit)

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
| 123 | Hardware Limitation: pKVM (AVF) Hypervisor Access is Blocked by Samsung/OEMs | 🟢 OPEN | #90 | Tracks hardware compatibility matrix and SELinux fallback |

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
| 120 | [DESIGN] Define production genesis balance and tokenomics | 🟢 OPEN | #3 | Placeholder balance of 1M micro-units needs production definition |

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

## 2. Audit-Discovered Issues — ✅ All Filed on GitHub

These are **critical bugs and gaps** found during the codebase deep-dive. All 20 have been filed as GitHub issues:

### 🔴 CRITICAL — Filed as GitHub Issues

| ID | State | Severity | Title | File | Lines | Description |
|----|-------|----------|-------|------|-------|-------------|
| A1 | ✅ CLOSED | 🔴 CRITICAL | **Missing VSOCK Command Handlers** | `main.rs` | 445-467 | `MINT_GENESIS:`, `FIRE_LOCK:`, and `ISSUE_SBT:` are called from Kotlin `MainActivity.kt` but have **no handler** in the Rust Vault. They silently fail with "Unrecognized command." This means Block-Lattice genesis, OCC ride locking, and Soulbound Token issuance are completely non-functional. |
| A2 | ✅ CLOSED | 🔴 CRITICAL | **ZK Distance Bounding Circuit Missing Inequality Constraint** | `zk_engine.rs` | 65-101 | The `DistanceBoundingCircuit` computes `round_trip_distance` and `max_round_trip` but **never enforces** `round_trip_distance ≤ max_round_trip`. The Groth16 proof proves the arithmetic is correct but does NOT prove the node is within the allowed distance. The proof is valid regardless of physical distance. |
| A3 | ✅ CLOSED | 🔴 CRITICAL | **Ranging is Entirely Simulated** | `main.rs` | 411-420 | The cryptographic distance bounding creates a **dummy peer keypair locally** and fabricates the entire challenge-response. `simulated_rx_time` is artificially set to `tx_timestamp + compute_delay + 100ns`. No actual over-the-air BLE/UWB ranging occurs. |
| A4 | ✅ CLOSED | 🔴 CRITICAL | **Ephemeral libp2p Identity** | `main.rs` | 200-210 | A new Ed25519 keypair is generated on every `REGISTER_NODE` call. The node's PeerId changes every boot, destroying DHT routing tables, peer reputation history, and network identity continuity. |
| A5 | ✅ CLOSED | 🔴 CRITICAL | **Groth16 Trusted Setup Runs Per-PoL** | `main.rs` | 425-435 | `generate_tof_proof()` calls `Groth16::circuit_specific_setup()` on every Proof-of-Location generation. Trusted setup is computationally expensive (seconds on mobile). Should be done once and the proving/verification keys cached. |

### 🟡 MAJOR — Filed as GitHub Issues

| ID | State | Severity | Title | File | Lines | Description |
|----|-------|----------|-------|------|-------|-------------|
| A6 | ✅ CLOSED | 🟡 MAJOR | **VSOCK Uses TcpStream::from_raw_fd() on Non-TCP Socket** | `main.rs` | 155-175 | `std::net::TcpStream` is used for `AF_VSOCK`. This wraps file descriptor in `TcpStream` which works but TCP-specific options might cause unexpected behavior. |
| A7 | ✅ CLOSED | 🟡 MAJOR | **No Peer Bootstrapping** | `main.rs` | 240-250 | No bootstrap lists exist. Without mDNS, node cannot find peers on internet. |
| A8 | ✅ CLOSED | 🟡 MAJOR | **DefaultHasher for PoL Hashing** | `main.rs` | 390-405 | `SipHash` is used for Proof of Location cryptographic binding, but it is NOT cryptographically secure. |
| A9 | 🟢 OPEN | 🟡 MAJOR | **Block-Lattice Is Data Structure Only** | `main.rs` | 62-70 | `StateBlock` is defined but no functions to mint genesis block or handle ledger logic. |
| A10 | 🟢 OPEN | 🟡 MAJOR | **BLE Mesh Has No S.H.I.F.T. Service UUID** | `MainActivity.kt` | 333-353 | Kotlin BLE scanner collects all nearby devices instead of filtering for S.H.I.F.T. peers. |
| A11 | ✅ CLOSED | 🟡 MAJOR | **No VSOCK Authentication** | `main.rs` | 129-185 | Any process on the Android side can call the enclave. No HMAC or challenge-response. |
| A12 | ✅ CLOSED | 🟡 MAJOR | **SOULBOUND_TOKEN Never Enforced** | `main.rs` | 48-50 | Rust Vault processes `GENERATE_POL` without checking if Soulbound Token exists. |
| A13 | ✅ CLOSED | 🟡 MAJOR | **Thread Safety: nearbyNodes** | `MainActivity.kt` | 98 | `mutableSetOf` is not thread-safe. Scan callbacks write on background threads while UI reads. |

### 🟠 MINOR — Filed as GitHub Issues

| ID | State | Severity | Title | File | Description |
|----|-------|----------|-------|------|-------------|
| A14 | 🟢 OPEN | 🟠 MINOR | **Unused activity_main.xml** | `res/layout/` | Leftover "Hello World" layout from project creation. UI is built programmatically. |
| A15 | 🟢 OPEN | 🟠 MINOR | **Dead Code: native-lib.cpp** | `cpp/` | C++ JNI stub "Hello from C++" is never called. CMakeLists.txt still builds it. |
| A16 | 🟢 OPEN | 🟠 MINOR | **Vestigial jni Crate in Cargo.toml** | `Cargo.toml` | `jni = "0.21.1"` is still in dependencies but no JNI functions exist in the binary. |
| A17 | 🟢 OPEN | 🟠 MINOR | **Duplicate libif_watch .so Files** | `jniLibs/` | Three copies of `libif_watch-*.so` exist. Only one is needed. |
| A18 | 🟢 OPEN | 🟠 MINOR | **Hardcoded Fallback GPS Coordinates** | `MainActivity.kt` | Falls back to `46.2382, -63.1311` (Prince Edward Island) when GPS unavailable. Should be handled as an error, not a silent fallback. |
| A19 | 🟢 OPEN | 🟠 MINOR | **Gemini Gatekeeper Model Name May Be Invalid** | `gemini-gatekeeper.yml` | `gemini-3.1-pro-preview` may not be a real endpoint. Failure causes the catch block to post "REJECT: Neural Engine Failure" which blocks all PRs. |
| A20 | 🟢 OPEN | 🟠 MINOR | **Ranging Distance Calculation Off By Factor of 2** | `ranging.rs` | `distance_mm = t_flight * 300` reports round-trip distance, not one-way. The factor-of-2 division is handled separately in `zk_engine.rs`, but this makes `ranging.rs` output misleading in isolation. |

---

## 3. Audit Issue → GitHub Issue Mapping

✅ **All 20 audit issues have been filed on GitHub** (2026-05-28)

| Audit ID | State | Severity | GitHub Issue | Title |
|----------|-------|----------|--------------|-------|
| A1 | ✅ CLOSED | 🔴 CRITICAL | [#97](https://github.com/Keshuin0/S.H.I.F.T/issues/97) | VSOCK command handlers missing for MINT_GENESIS, FIRE_LOCK, ISSUE_SBT |
| A2 | ✅ CLOSED | 🔴 CRITICAL | [#98](https://github.com/Keshuin0/S.H.I.F.T/issues/98) | ZK Distance Bounding circuit missing inequality constraint |
| A3 | ✅ CLOSED | 🔴 CRITICAL | [#99](https://github.com/Keshuin0/S.H.I.F.T/issues/99) | Cryptographic ranging is entirely simulated |
| A4 | ✅ CLOSED | 🔴 CRITICAL | [#100](https://github.com/Keshuin0/S.H.I.F.T/issues/100) | libp2p PeerId regenerates on every boot |
| A5 | ✅ CLOSED | 🔴 CRITICAL | [#109](https://github.com/Keshuin0/S.H.I.F.T/issues/109) | Groth16 trusted setup runs on every PoL |
| A6 | ✅ CLOSED | 🟡 MAJOR | [#101](https://github.com/Keshuin0/S.H.I.F.T/issues/101) | VSOCK bridge uses TcpStream on non-TCP socket |
| A7 | ✅ CLOSED | 🟡 MAJOR | [#102](https://github.com/Keshuin0/S.H.I.F.T/issues/102) | No peer bootstrapping |
| A8 | ✅ CLOSED | 🟡 MAJOR | [#110](https://github.com/Keshuin0/S.H.I.F.T/issues/110) | PoL uses non-cryptographic DefaultHasher |
| A9 | 🟢 OPEN | 🟡 MAJOR | [#103](https://github.com/Keshuin0/S.H.I.F.T/issues/103) | Block-Lattice has no operational logic |
| A10 | 🟢 OPEN | 🟡 MAJOR | [#104](https://github.com/Keshuin0/S.H.I.F.T/issues/104) | BLE mesh scanner collects all Bluetooth devices |
| A11 | ✅ CLOSED | 🟡 MAJOR | [#111](https://github.com/Keshuin0/S.H.I.F.T/issues/111) | No VSOCK authentication |
| A12 | ✅ CLOSED | 🟡 MAJOR | [#112](https://github.com/Keshuin0/S.H.I.F.T/issues/112) | SOULBOUND_TOKEN never set or enforced |
| A13 | ✅ CLOSED | 🟡 MAJOR | [#105](https://github.com/Keshuin0/S.H.I.F.T/issues/105) | nearbyNodes MutableSet is not thread-safe |
| A14 | 🟢 OPEN | 🟠 MINOR | [#113](https://github.com/Keshuin0/S.H.I.F.T/issues/113) | activity_main.xml is unused |
| A15 | 🟢 OPEN | 🟠 MINOR | [#114](https://github.com/Keshuin0/S.H.I.F.T/issues/114) | native-lib.cpp is dead code |
| A16 | 🟢 OPEN | 🟠 MINOR | [#115](https://github.com/Keshuin0/S.H.I.F.T/issues/115) | jni crate in Cargo.toml is vestigial |
| A17 | 🟢 OPEN | 🟠 MINOR | [#116](https://github.com/Keshuin0/S.H.I.F.T/issues/116) | Duplicate libif_watch .so files |
| A18 | 🟢 OPEN | 🟠 MINOR | [#106](https://github.com/Keshuin0/S.H.I.F.T/issues/106) | Hardcoded fallback GPS coordinates |
| A19 | 🟢 OPEN | 🟠 MINOR | [#107](https://github.com/Keshuin0/S.H.I.F.T/issues/107) | Gemini Gatekeeper invalid model name |
| A20 | 🟢 OPEN | 🟠 MINOR | [#108](https://github.com/Keshuin0/S.H.I.F.T/issues/108) | ranging.rs distance off by factor of 2 |

---

## 4. Hardware Limitations & The Hybrid Path

**The Flaw: pKVM (AVF) Hypervisor Access is Blocked by Samsung/OEMs** (Tracked in GitHub Issue [#123](https://github.com/Keshuin0/S.H.I.F.T/issues/123))
During our deep dive, we discovered that the Android OS completely blocks the `virtualmachine` service on Samsung Galaxy devices (like the Fold 6). Google's pKVM (Android Virtualization Framework) is a strict hardware-level feature that relies on the OEM's bootloader.

**The Solution: Graceful Degradation (Native Fallback)**
We cannot force a Samsung phone to spawn a hardware VM if the Samsung firmware denies it. To resolve this without compromising the bleeding-edge architecture for supported devices, we implemented the **Hybrid Path (Dual-Bind Fallback)**:
1. **Tier 1 (AVF)**: On supported devices (like the Pixel 8), the Rust Vault binds to `AF_VSOCK` inside the Microdroid VM for maximum hardware-level isolation.
2. **Tier 2 (Native Fallback)**: On restricted devices (like the Samsung Fold 6), the app uses `ProcessBuilder` to spawn `libshift_core.so` binary as a highly isolated background daemon, communicating over standard TCP `127.0.0.1:8000`. It is securely sandboxed by Android's strict SELinux policies.

---

*This file is a living document. Update it as issues are resolved in code.*
