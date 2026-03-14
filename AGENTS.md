# AGENTS.md

## Scope
These instructions apply to the entire repository rooted here.

## Project Overview
- This repository is a Rust `no_std` hobby OS kernel based on the `blog_os` structure.
- The main kernel entry points live in `src/main.rs` and `src/lib.rs`.
- Integration tests live in `tests/` and use custom boot/test infrastructure.

## Working Style
- Keep changes small and targeted; avoid broad refactors unless explicitly requested.
- Preserve the existing low-level architecture and module layout.
- Prefer fixes at the actual failure point instead of layering workarounds on top.
- Do not edit generated artifacts in `target/` or disk images in `disk_image/`.

## Code Guidelines
- Match the existing Rust style and naming in nearby files.
- Avoid introducing new dependencies unless they are clearly necessary.
- Keep `no_std` and allocation constraints in mind when adding code.
- Be careful with `unsafe`; keep it minimal and justify it in code structure.
- Do not add inline comments unless the surrounding code already uses them or the logic is unusually subtle.

## Build And Test
- Use the repository root as the working directory for cargo commands.
- Preferred validation sequence for substantive Rust changes:
  - `cargo build`
  - `cargo test`
- Some tasks may also require:
  - `cargo run`
  - `cargo bootimage`
- This project expects a nightly Rust toolchain and the `bootimage` tool installed.

## Files To Check First
- `README.md` for project-specific build and run expectations.
- `Cargo.toml` for crate configuration and test targets.
- `src/` for kernel code and memory/interrupt/task subsystems.
- `tests/` for boot-time integration coverage.

## Agent Notes
- If a change affects boot flow, interrupts, memory management, allocation, or task execution, inspect adjacent tests and add or update coverage when there is an established pattern for it.
- Before finishing, verify that `.gitignore` still excludes build output and generated artifacts.
