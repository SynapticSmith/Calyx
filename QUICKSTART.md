# Calyx Quickstart

Welcome to Calyx! This guide will help you get the project built and show you the basic paths to start using the association-native database.

## Prerequisites

Calyx is a Rust workspace built on the `2024` edition. You will need:

1. **Rust**: Version `1.95` or higher. The easiest way to install it is via `rustup`. Our `rust-toolchain.toml` will automatically pin to the correct version.
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. **C Toolchain**: Required for compiling bundled dependencies like SQLite. On Linux/macOS, installing `build-essential` or Xcode command line tools is usually sufficient.

*Optional for GPU acceleration:* an NVIDIA `sm_120`-class GPU and CUDA `13.2`.

## Building the Project

Clone the repository and build the workspace. By default, Calyx builds for the CPU using SIMD math operations.

```bash
git clone https://github.com/ChrisRoyse/Calyx.git
cd Calyx

# Build the workspace in release mode (CPU-only by default)
cargo build --release --workspace

# Run the test suite to verify everything works
cargo test --workspace
```

If you have a compatible NVIDIA GPU and want to use the CUDA backend:

```bash
cargo build --release --workspace --features cuda
```

## Exploring the Hands-on Example

To see Calyx in action from a developer's perspective, check out the `hello_calyx` example. This example demonstrates how to set up the core engine traits, mock data into the system, and understand the flow of creating a constellation.

To run the example:

```bash
cargo run -p hello_calyx
```

## Next Steps

- **Deep Dive**: Read `docs/COMPREHENSIVE_GUIDE.md` to fully understand the architectural concepts (Constellations, Kernels, Guard, Ledger).
- **Core Abstractions**: Take a look at `crates/calyx-core/src/` to see the trait boundaries and data structures that hold the engine together.
- **The Intelligence Layer**: Explore `crates/calyx-ward` and `crates/calyx-oracle` to see how the system prevents out-of-distribution answers and predicts grounded outcomes.

*Note: The standalone CLI (`calyx`), daemon (`calyxd`), and MCP server (`calyx-mcp`) are in active development. As they stabilize, you will be able to run Calyx as a managed background service!*
