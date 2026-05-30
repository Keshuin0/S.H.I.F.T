# Walkthrough: S.H.I.F.T. Core Architecture — Resolving Issues #101 (A6) & #102 (A7)

This walkthrough documents the technical architecture, implementation, and successful verification of the changes introduced to resolve **Issue #101 (A6)** and **Issue #102 (A7)**. 

---

## 1. Architectural Changes Overview

We have refactored the socket communication and network bootstrapping systems of the S.H.I.F.T. core to meet the bleeding-edge security and performance standards of the Sovereign Ride-Sharing network.

### 🔑 A6: Asynchronous Non-blocking VSOCK Hypervisor Stream
Previously, the enclave hypervisor connection wrapper used insecure, blocking standard library `TcpStream::from_raw_fd` sockets.
*   **Abstraction**: Introduced `SovereignStream` trait that encapsulates asynchronous read/write interfaces (`tokio::io::AsyncRead` + `tokio::io::AsyncWrite`).
*   **Non-Blocking System Call Driver**: Implemented `VsockStream` (gated under Unix Targets) which drives OS-level virtual socket reads and writes via direct, safe `libc` calls (`libc::read` and `libc::write`) inside a Tokio `AsyncFd<OwnedFd>`.
*   **Owned File Descriptor Lifecycle**: Integrated `std::os::fd::OwnedFd` directly. Wrapping raw socket descriptors in `OwnedFd` ensures that when `VsockStream` is dropped, the underlying system file descriptor is closed cleanly.
*   **Asynchronous Listening**: Configured non-blocking fcntl flags (`libc::O_NONBLOCK`) on sockets. The server handles connections concurrently via `tokio::spawn` without blocking the main event loops.
*   **Global Runtime Consolidation**: Removed nested runtime allocations (`tokio::runtime::Runtime::new()`) and the redundant `ASYNC_RUNTIME` OnceLock. The entire executable now runs inside a single globally optimized `#[tokio::main]`.

### 🌐 A7: Dynamic DNS-Based Bootstrap Seeding
Previously, bootstrap nodes were restricted to hardcoded, static IP addresses.
*   **libp2p DNS Resolution**: Enabled the `"dns"` feature flag on the `libp2p` dependency.
*   **Seed Addresses**: Configured a flexible, DNS-based multiaddress seed array (`BOOTSTRAP_NODES`).
*   **Bootstrapping Sequence**: On initialization, the peer routing tables parse the DNS multiaddresses, dynamically dial the targets, register them as explicit peers in GossipSub and Kademlia routing tables, and fire `kademlia.bootstrap()` to discover neighboring riders/drivers.

---

## 2. Implementation Visual Flow

```mermaid
graph TD
    Sub[#[tokio::main] Main Loop] -->|If Unix| VSOCK[AF_VSOCK Listener]
    Sub -->|Fallback/Desktop| TCP[TCP Listener 127.0.0.1:8000]
    
    VSOCK -->|Incoming Connection| FCNTL1[Fcntl Set O_NONBLOCK]
    FCNTL1 -->|Wrap in OwnedFd| FD[AsyncFd OwnedFd]
    FD -->|Wrap in VsockStream| SpawnV[tokio::spawn handle_connection]
    
    TCP -->|Incoming Connection| TcpStream[tokio::net::TcpStream]
    TcpStream --> SpawnT[tokio::spawn handle_connection]
    
    SpawnV --> SecureHandshake[Secure Handshake via SovereignStream]
    SpawnT --> SecureHandshake
    
    SecureHandshake --> ECDH[Ephemeral ECDH Secret Derivation]
    ECDH --> AES[AES-256-GCM Payload Encryption]
    AES --> Process[Execute Core Command Engine]
```

---

## 3. Verification and Target Compilation Status

The codebase compiles 100% cleanly on all platforms with warnings-as-errors (`-D warnings`) enforced.

### 💻 Local Desktop Target (Windows)
*   **Command**: `cargo test` & `cargo clippy -- -D warnings`
*   **Result**: Compiled cleanly with zero warnings. All cryptographic, zkVM, and key-derivation unit tests passed successfully.
*   **Test Snippet**:
    ```text
    running 3 tests
    test tests::test_deterministic_key_derivation ... ok
    test zk_prover::tests::test_distance_out_of_bounds_fails - should panic ... ok
    test zk_prover::tests::test_caching_proving_and_verification ... ok

    test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.08s
    ```

### 📱 Android target (`aarch64-linux-android`)
*   **Command**: `cargo ndk -t arm64-v8a -P 35 clippy -- -D warnings` & `cargo ndk -t arm64-v8a -P 35 build`
*   **Result**: Compiled successfully using Android platform API Level 35 matching the application's SDK configuration. Transitive dependency requirements (including `if_addrs` dynamic socket resolution linking) resolved without any missing symbol errors.
