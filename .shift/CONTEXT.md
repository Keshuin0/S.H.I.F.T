# S.H.I.F.T. — AI Session Context

> **INSTRUCTION FOR AI:** Read this entire file at the start of every session. It contains the full project context, history, and current state. Update this file at the end of every session.

---

## Quick Start

**Project:** S.H.I.F.T. — A sovereign Layer-1 blockchain for decentralized, middleman-free ride-sharing
**Repo:** https://github.com/Keshuin0/S.H.I.F.T
**Project Board:** https://github.com/users/Keshuin0/projects/8
**Local Path:** D:\Project\Project S.H.I.F.T
**Owner:** Keshuin0
**CLI Auth:** `gh` CLI authenticated as Keshuin0 (scopes: repo, read:project, project)

---

## Architecture Overview

### Rust Core (`shift_core/src/`)
- **main.rs** — VSOCK listener (port 8000), identity management (Ed25519), PoL generation, proximity triangulation, BLE/UWB ranging, libp2p networking (GossipSub + Kademlia + QUIC)
- **zk_engine.rs** — Arkworks ZK circuits (Groth16), DistanceBoundingCircuit, RideCircuit (pricing)
- **zk_prover.rs** — Global static parameter OnceLocks and zero-overhead proving routines
- **ranging.rs** — Time-of-flight distance calculation
- **Block-Lattice** — StateBlock struct + LOCAL_LEDGER (OnceLock) — data structure only, no operations yet

### Android App (`android_app/app/src/main/java/.../`)
- **MainActivity.kt** — Jetpack Compose UI, BLE scanning, biometric gating, VSOCK client, GPS/cellular data collection

### Communication
- Kotlin ↔ Rust via VSOCK (AF_VSOCK port 8000)
- Commands: REGISTER_NODE, GENERATE_POL, GET_LOCATION_SIGNATURE, INITIATE_RANGE, etc.

---

## Session History

### Session 1 (2026-05-28, Conversation: a8d4cba2-e872-4325-8c9b-c615f9dd5855)
**What was done:**
1. **Full codebase audit** — Read every file, every line of Rust + Kotlin
2. **Filed 20 audit issues** (A1-A20) as GitHub issues #97-#116
3. **Complete GitHub reorganization:**
   - Deleted 20 old labels, created 32 new labels (7 axes: type, priority, component, phase, status, platform, lang)
   - Created 6 milestones (M0-M5) with due dates and weekly plans
   - Created 3 issue templates (bug_report.yml, feature_request.yml, config.yml)
   - Relabeled all 76 open issues with new taxonomy
   - Assigned all issues to milestones
   - Added cross-references to 13 critical/major audit issues
   - Created and pinned #117 (Roadmap), #118 (Audit Checklist)
   - Added all 76 issues to project board
4. **Created ISSUES_TRACKER.md** — Full inventory of all issues
5. **Verified everything** — Labels, milestones, project board, cross-refs, body integrity all confirmed clean

**Artifacts saved:**
- `D:\Project\Project S.H.I.F.T\.shift\CONTEXT.md` (this file)
- `D:\Project\Project S.H.I.F.T\.shift\walkthrough.md` (detailed change log)
- `D:\Project\Project S.H.I.F.T\.shift\audit_report.md` (full audit findings)
- `D:\Project\Project S.H.I.F.T\ISSUES_TRACKER.md` (issue inventory)

### Session 2 (2026-05-28)
**What was done:**
1. **Implemented 3 missing VSOCK handlers** (`ISSUE_SBT`, `MINT_GENESIS`, `FIRE_LOCK`) in `main.rs`.
2. **Auto-init boot sequence** added to `MainActivity.kt` so the node fully initializes autonomously on boot.
3. **SBT KYC enforcement** added to `GENERATE_POL` (fixing #112).
4. **Resolved Issue #97 (A1) and Issue #112 (A12)**.
5. **Closed GitHub Issues #97 and #112** with detailed closing comments.
6. **Created design Issue #120** on GitHub for genesis balance placeholder and tokenomics.

### Session 3 (2026-05-28)
**What was done:**
1. **Implemented Hybrid Path (Dual-Bind Fallback)** to bypass Samsung pKVM limitations.
   - Tier 1: Hardware-isolated AVF/pKVM via VSOCK port 8000 inside Microdroid VM (Pixel 8, etc.).
   - Tier 2: SELinux-sandboxed native daemon via TCP port 8000 on `127.0.0.1` (Galaxy Z Fold6, etc.).
2. **Created & Labeled GitHub Issue #123**: Documented the hypervisor hardware limitations, setting tags (`platform: android`, `component: tee-vault`, `phase: 1`, `type: research`, `P2: medium`) and assigning the `M1: Root of Trust` milestone.
3. **Resolved TCP Bridge Connection Error (`ECONNREFUSED`)**:
   - Rebuilt Rust core and replaced the stale `libshift_core.so` file in `jniLibs` with the latest build.
   - Added a `delay(500)` in `igniteNativeFallback` to give the native daemon time to boot and bind to port 8000.
   - Added connection retries (5 attempts, 100ms intervals) in `TeeBridge.sendCommand` to eliminate race conditions.
   - Enabled Logcat output routing of the daemon's stdout/stderr under the tag `SHIFT_VAULT_DAEMON`.
4. **Deployed and Installed Updated App**: Compiled and successfully installed the updated APK directly on the connected phone via command-line ADB.
5. **Fixed Log Auto-Scroll & Spawn Guard (Verification Stage)**:
   - Replaced `addTextChangedListener` with `addOnLayoutChangeListener` on `statusText` to trigger console auto-scrolling after layout sizing passes, resolving the race condition clipping Phase 4.3 smart contract logs.
   - Guarded `igniteHypervisor()` to check if the hypervisor or native fallback daemon is already active, preventing duplicate spawns and address-binding panics.
   - Successfully compiled, packaged, and reinstalled the debug APK on `RFCX61EJAHR`.

### Session 4 (2026-05-29, Conversation: 9a00fded-699d-4d46-accd-22ccbcf199e9)
**What was done:**
1. **Resolved Issue #100 (A4)** — Ephemeral libp2p Identity (P0 Critical).
2. **Zero-Storage Key Derivation**: Integrated ECDH `KeyAgreement` inside Android KeyStore using `SHIFT_SOVEREIGN_AGREEMENT_KEY` against a static public key salt to derive `S_classical` dynamically.
3. **Hybrid Post-Quantum Identity (HPQI)**: Combined the hardware ECDH classical secret with an encrypted PQC software seed using `HKDF-SHA256` inside `main.rs` to generate the libp2p Ed25519 keypair in RAM on boot, requiring 0 private key files on disk.
4. **Biometric Boot Gating**: Added a user fingerprint authentication prompt (`authenticateForBoot`) required to run key agreements and spin up the P2P swarm.
5. **Hardware Compatibility Fallbacks**: Implemented fallback exceptions to standard TEE storage if a StrongBox chip is unavailable.
6. **Desktop Compilation Gating**: Conditionally gated Unix sockets behind `#[cfg(unix)]` in `main.rs` and added a TCP listener under `#[cfg(not(unix))]` to enable desktop compilation checks and tests.
7. **Unit Test Verification**: Implemented deterministic key derivation unit tests which passed successfully.

### Session 5 (2026-05-30, Conversation: bef1657e-52b4-4a8a-a2be-271e2af5ddb1)
**What was done:**
1. **Resolved Issue #109 (A5)** — Groth16 trusted setup runs on every PoL (P0 Critical).
2. **Deadlock-Free File Separation**: Restructured the ZK system, separating pure circuit constraints (`zk_engine.rs`) from proving execution cache (`zk_prover.rs`). This allows `build.rs` to compile the circuit without causing compile-time circular dependency deadlocks.
3. **Preserved Standalone Binary target**: Retained `shift_core` as a pure binary crate (required to compile the executable for the AVF hypervisor) instead of splitting it into a library crate.
4. **Deterministic Compile-Time Key setup**: Integrated `build.rs` using a seed-stabilized `rand_chacha::ChaCha20Rng` to run the trusted setup ceremony at compile-time and output serialized keys directly into `OUT_DIR`.
5. **Baked static keys**: Baked the pre-generated proving and verification keys directly into the executable's read-only memory footprint (`&[u8]`) using `include_bytes!`.
6. **OnceLock RAM structure caching**: Implemented Tier 2 caching in `zk_prover.rs` to deserialize the baked keys exactly once on node boot (`REGISTER_NODE`) into global `OnceLock` structures (`PROVING_KEY` and `VERIFYING_KEY`) using fast unchecked deserialization, achieving zero heap allocations and sub-millisecond key lookups during proof generation.
7. **Benchmark Verification**: Proving latency dropped from seconds/minutes to **30.92 ms** on local CPU. All unit tests successfully compiled and passed cleanly.
8. **Target Compilation Verification**: Cross-compiled the ARM64 release binary for Android using `cargo ndk` to confirm compilation footprint with zero warnings.
9. **Physical Device Verification**: Discovered a JNI library directory packaging discrepancy (where `cargo ndk` only updates library targets automatically). Resolved it by manually syncing the `shift_vault` executable target to `libshift_core.so` in `jniLibs`. Deployed to a physical test phone and verified successful boot key loading and Proof of Location ZK-proving with an outstanding **7ms** latency (exceeding target metrics).

### Session 6 (2026-05-30, Conversation: a0b246fc-1c02-4353-bfb2-f48f673151ff)
**What was done:**
1. **Resolved Issue #111 (A11)** — Sovereign Tunnel: VSOCK Challenge-Response Authentication.
2. **Hybrid Authenticated Key Exchange (AKE)**: Implemented Ephemeral Curve25519 (P-256) ECDH key agreement with Perfect Forward Secrecy.
3. **Digital Signature Transcript Verification**: Verified the digital signature transcript using the device's hardware-backed private key (`SHIFT_SOVEREIGN_NODE_ID`) in Android KeyStore, checking it against the expected node identity in Rust.
4. **Military-grade AEAD Encryption**: Used `AES-GCM-256` to encrypt all telemetry commands and payloads over the local socket connection.
5. **Resolved Biometric Authorization Deadlock**: Added 15-second validity duration (`setUserAuthenticationValidityDurationSeconds(15)`) to the signing key to prevent deadlocks when prompt triggers before the handshake transcript exists.
6. **Fixed Signed Byte Hex Mismatch**: Standardized hex conversion of public key bytes using unsigned lowercase formatting in Kotlin.
7. **End-to-End Verification**: Compiled and successfully verified on physical test device `SM-F956W`.

### Session 7 (2026-05-30, Conversation: 54081ed0-b36a-4b31-b444-b5a3893a2117, Current)
**What was done:**
1. **Resolved Issue #98 (A2)** — ZK Distance Bounding circuit missing inequality constraint.
2. **Difference Range Proof (DRP) Optimization**: Implemented an optimized 32-bit range proof of the algebraic difference ($max\_round\_trip - round\_trip\_dist$) using finite field wrap-around properties. This reduces the constraint count to exactly 33 (a 75% reduction over standard `UInt64` comparison gadgets).
3. **Test Suite Hardening**: Corrected simulated coordinates in `test_caching_proving_and_verification` to use valid parameters. Added a negative test case `test_distance_out_of_bounds_fails` with `#[should_panic]` to verify out-of-bounds rejection.
4. **End-to-End Verification**: Cross-compiled the Rust core, packaged the APK, and deployed to physical Galaxy Z Fold 6 (SM-F956W). Verified that valid telemetry successfully generates a 192-byte ZK-SNARK PoL proof in **9 ms**.
5. **GitHub Reorganization**: Updated `ISSUES_TRACKER.md` and closed remote Issue #98 on GitHub via CLI.
6. **Resolved Pull Request #128 CI Check Failures**:
   - Fixed 6 Clippy errors under warnings-as-errors (`-D warnings`) in `shift_core/src/main.rs`.
   - Modified `gemini-gatekeeper.yml` to use `gemini-2.5-pro` instead of the invalid model endpoint `gemini-3.1-pro-preview`.
   - Compiled the target `aarch64-linux-android` release binary with platform level 24 (`-P 24`) to correctly link network functions (`getifaddrs`/`freeifaddrs`), updating `libshift_core.so` in `android_app/app/src/main/jniLibs/arm64-v8a/`.

---

## Current State

### GitHub Organization
- **71 open issues**, 35 closed, 106 total
- **32 labels** across 7 axes (type, priority, component, phase, status, platform, lang)
- **6 milestones:** M0 (Jul 10) → M5 (Jul 9, 2027)
- **3 pinned issues:** #117 Roadmap, #118 Audit Checklist, #1 Phase 1 Epic
- **3 issue templates** on main branch (blank issues disabled)
- **76/76 issues** in project board

### Milestone Status
| Milestone | Issues | Due | Status |
|-----------|--------|-----|--------|
| M0: Audit Fixes | 24 | Jul 10, 2026 | 🟡 In Progress (6 closed) |
| M1: Root of Trust | 14 | Oct 2, 2026 | 🟡 In Progress (Fallback added) |
| M2: P2P Mesh MVP | 18 | Dec 25, 2026 | 🔴 Not started |
| M3: Ledger & Settlement | 8 | Mar 5, 2027 | 🔴 Not started |
| M4: Economics & AI | 7 | May 14, 2027 | 🔴 Not started |
| M5: Production UX | 5 | Jul 9, 2027 | 🔴 Not started |

### Priority Issues (Fix Order)
**P0 Critical (4):** #99 (A3), #101 (A6), #102 (A7), #110 (A8)
**P1 High (27):** Most Phase 1-2 features + 6 major audit issues
**P2 Medium (18):** Phase 2-3 features + Issue #123 (Hardware Limitation)
**P3 Low (10):** Cleanup + minor audit issues

### Next Work
Start M0: Audit Fixes. Recommended order:
1. ~~A1 (#97) — Add MINT_GENESIS, FIRE_LOCK, ISSUE_SBT handlers to main.rs~~ (Completed Session 2)
2. ~~Hardware Fallback (#123) — Implement Native Fallback for non-AVF devices~~ (Completed Session 3)
3. ~~A4 (#100) — Persist libp2p identity to TEE storage~~ (Completed Session 4)
4. ~~A5 (#109) — Cache Groth16 proving/verification keys~~ (Completed Session 5)
5. ~~A11 (#111) — Add VSOCK challenge-response authentication~~ (Completed Session 6)
6. ~~A2 (#98) — Add enforce_less_than constraint to ZK circuit~~ (Completed Session 7)
7. A3 (#99) — Replace simulated ranging with real BLE/UWB

---

## Key Technical Details

### Critical Code Locations
| What | File | Lines | Status |
|------|------|-------|--------|
| VSOCK listener | main.rs | 129-185 | Closed: ECDH + Hardware Signature + AES-GCM Encrypted Tunnel |
| Command handler | main.rs | 445-560 | Missing commands implemented |
| Identity generation | main.rs | 200-230 | Persistent via ECDH & PQC HKDF |
| PoL generation | main.rs | 390-435 | Uses SipHash (not crypto), setup per-call |
| Prover Cache | zk_prover.rs | 10-69 | OnceLock key cache + benchmark unit test |
| ZK circuit | zk_engine.rs | 42-140 | Closed: Enforced 32-bit DRP inequality constraint |
| Ranging | main.rs | 411-420 | Entirely simulated |
| BLE scanner | MainActivity.kt | 333-353 | No UUID filter |
| nearbyNodes | MainActivity.kt | 98 | Not thread-safe |
| StateBlock | main.rs | 62-70 | Data structure only |

---

## User Preferences
- **Tone:** Lead Systems Architect, ruthlessly logical
- **Naming:** Consistent style (no mixed formats)
- **Templates:** Blank issues disabled, force structured templates
- **Planning:** Prefers thorough planning before execution
- **Timeline:** Flexible, no hard deadlines, ~15 hrs/week

---

*Last updated: 2026-05-30 Session 7*
