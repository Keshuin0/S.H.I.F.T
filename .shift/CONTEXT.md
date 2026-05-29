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

### Session 3 (2026-05-28, Current)
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

---

## Current State

### GitHub Organization
- **75 open issues**, 31 closed, 106 total
- **32 labels** across 7 axes (type, priority, component, phase, status, platform, lang)
- **6 milestones:** M0 (Jul 10) → M5 (Jul 9, 2027)
- **3 pinned issues:** #117 Roadmap, #118 Audit Checklist, #1 Phase 1 Epic
- **3 issue templates** on main branch (blank issues disabled)
- **76/76 issues** in project board

### Milestone Status
| Milestone | Issues | Due | Status |
|-----------|--------|-----|--------|
| M0: Audit Fixes | 24 | Jul 10, 2026 | 🔴 Not started |
| M1: Root of Trust | 14 | Oct 2, 2026 | 🟡 In Progress (Fallback added) |
| M2: P2P Mesh MVP | 18 | Dec 25, 2026 | 🔴 Not started |
| M3: Ledger & Settlement | 8 | Mar 5, 2027 | 🔴 Not started |
| M4: Economics & AI | 7 | May 14, 2027 | 🔴 Not started |
| M5: Production UX | 5 | Jul 9, 2027 | 🔴 Not started |

### Priority Issues (Fix Order)
**P0 Critical (6):** #97 (A1 - Closed), #98 (A2), #99 (A3), #100 (A4 - Closed), #109 (A5), #111 (A11)
**P1 High (27):** Most Phase 1-2 features + 7 major audit issues
**P2 Medium (18):** Phase 2-3 features + Issue #123 (Hardware Limitation)
**P3 Low (10):** Cleanup + minor audit issues

### Next Work
Start M0: Audit Fixes. Recommended order:
1. ~~A1 (#97) — Add MINT_GENESIS, FIRE_LOCK, ISSUE_SBT handlers to main.rs~~ (Completed Session 2)
2. ~~Hardware Fallback (#123) — Implement Native Fallback for non-AVF devices~~ (Completed Session 3)
3. ~~A4 (#100) — Persist libp2p identity to TEE storage~~ (Completed Session 4)
4. A5 (#109) — Cache Groth16 proving/verification keys
5. A11 (#111) — Add VSOCK challenge-response authentication
6. A2 (#98) — Add enforce_less_than constraint to ZK circuit
7. A3 (#99) — Replace simulated ranging with real BLE/UWB

---

## Key Technical Details

### Critical Code Locations
| What | File | Lines | Status |
|------|------|-------|--------|
| VSOCK listener | main.rs | 129-185 | Working but unauthenticated |
| Command handler | main.rs | 445-560 | Missing commands implemented |
| Identity generation | main.rs | 200-230 | Persistent via ECDH & PQC HKDF |
| PoL generation | main.rs | 390-435 | Uses SipHash (not crypto), setup per-call |
| ZK circuit | zk_engine.rs | 65-101 | Missing inequality constraint |
| Ranging | main.rs | 411-420 | Entirely simulated |
| BLE scanner | MainActivity.kt | 333-353 | No UUID filter |
| nearbyNodes | MainActivity.kt | 98 | Not thread-safe |
| StateBlock | main.rs | 62-70 | Data structure only |

### Dependency Chains
```
A13 (thread safety) → A10 (BLE UUID) → #51 (BLE Beacon)
A11 (VSOCK auth) → #90 (SELinux) + #87 (pKVM)
A1 (handlers) → #57 (Block-Lattice) + #29 (SBT)
A2 + A5 → #89 (ZK-SNARK for ToF)
A4 (PeerId) → #42 (DHT) → #44 (Proximity)
A7 (bootstrap) → #34 (Radio Mesh)
```

---

## User Preferences
- **Tone:** Lead Systems Architect, ruthlessly logical
- **Naming:** Consistent style (no mixed formats)
- **Templates:** Blank issues disabled, force structured templates
- **Planning:** Prefers thorough planning before execution
- **Timeline:** Flexible, no hard deadlines, ~15 hrs/week

---

*Last updated: 2026-05-29 Session 4*
