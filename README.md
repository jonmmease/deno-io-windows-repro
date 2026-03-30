# deno-io-windows-repro

Minimal reproduction of a Windows compile failure in [`deno_io`](https://crates.io/crates/deno_io) 0.148.0 through at least 0.152.0.

## What

`cargo check` succeeds on Linux/macOS and fails on Windows with `rustc >= 1.94.0`.

## Root cause

`deno_io` uses the `winapi` crate, which defines its own `winapi::ctypes::c_void` type. On modern Rust + `libc >= 0.2.183`, `libc::c_void` re-exports `core::ffi::c_void` — a distinct type from `winapi::ctypes::c_void`. Code in `deno_io` that passes handles between winapi functions and std's `RawHandle`/`FromRawHandle` hits `E0308` type mismatches.

## Error locations in `deno_io`

- `winpipe.rs:115` — `create_named_pipe_inner()` returns `(RawHandle, RawHandle)` but pipe handles from winapi are `*mut winapi::ctypes::c_void`
- `lib.rs:212,217,222` — `GetStdHandle()` returns a winapi handle, passed to `StdFile::from_raw_handle()` which expects `*mut std::ffi::c_void`
- `lib.rs:1321,1323` — `file.as_raw_handle()` returns `*mut std::ffi::c_void`, passed to functions expecting `*mut winapi::ctypes::c_void`

## Affected versions

`deno_io` 0.148.0 through at least 0.152.0 (all versions using `winapi`).

## Why Deno CI passes

Deno pins to Rust 1.80.1 via `rust-toolchain.toml`, which predates the `libc` change.

## Fix

Migrate from `winapi` to `windows-sys` (tracked in [denoland/deno#27158](https://github.com/denoland/deno/issues/27158)), or add explicit `as *mut _` casts between the `c_void` types.

## Discovered in

https://github.com/vega/vl-convert/pull/264 — updating Deno crate dependencies caused `libc` to resolve to 0.2.183 (forced by `mio >= 1.2.0` and `rustix >= 1.1.4`), triggering the mismatch on Windows CI with Rust 1.94.1.
