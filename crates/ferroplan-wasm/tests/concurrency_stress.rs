//! fond-htn-29 (wave v26.9.17): host-side concurrency stress over the WASI
//! linear-memory dispatch surface.
//!
//! `src/wasi_abi.rs` is cfg-gated to `wasm32-wasip1` in `lib.rs` (the two
//! wasm targets are mutually exclusive at compile time:
//! `cfg(target_os = "wasi")` vs `cfg(target_os = "unknown")`), so on a
//! native host its `dispatch` function never compiles into the lib's own
//! unittest binary — without this file, the ABI surface would have ZERO
//! host-executable tests. This integration target pulls in the EXACT same
//! source file via `#[path]` — byte-for-byte the Rust that compiles into
//! the wasm32-wasip1 guest (a real file module, so its inner `//!` doc
//! comments stay legal, which a raw `include!` would not allow) — so the
//! 40-thread mixed-workload stress can drive the real wire path
//! (`dispatch`: JSON bytes in, JSON bytes out) with OS threads, which
//! wasip1 itself cannot provide (no OS threads in the guest;
//! `std::thread::spawn` there traps the instance). The full
//! host-side-dispatch rationale lives in the concurrency-stress module's
//! doc comment inside `src/wasi_abi.rs`.

// The pulled-in module carries its own `#[cfg(test)] mod tests` (the
// single-threaded op tests from wave-3 ticket 17) — compiled here too, so
// they get their only native-host execution alongside the stress suite.
// `wasi_abi.rs` resolves its shared probe guards as `crate::probe_guard`,
// so the host view pulls in that exact source file at the same crate path.
#[allow(dead_code)]
#[path = "../src/probe_guard.rs"]
mod probe_guard;

#[allow(dead_code)]
#[path = "../src/wasi_abi.rs"]
mod wasi_abi_host_view;
