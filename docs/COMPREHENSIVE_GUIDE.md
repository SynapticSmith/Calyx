# Calyx Comprehensive Guide

Welcome to the comprehensive guide to Calyx. This document explains the high-level concepts, the architectural layers, and how the many components (crates) work together to create an association-native database.

## 1. Core Concepts

At its heart, Calyx does not just store a row (like a relational DB) or a vector (like a traditional vector DB). It stores **constellations**.

### Constellations
A constellation is the fundamental record in Calyx. It represents a single input that has been measured through many independent "lenses." Instead of flattening these measurements into one opaque blob, Calyx keeps them separate. This means an input is represented as a cluster of typed vectors—a constellation—anchored to real outcomes.

### Lenses
A lens is an embedder or feature extractor. You can think of it as a specific viewpoint on your data. Examples include a dense embedding model (like BERT or an ONNX model), a sparse lexical extractor (like BM25), or a temporal feature extractor. In Calyx, you use multiple lenses simultaneously to get a rich, multifaceted understanding of an input.

### Slots
When an input passes through a lens, the output is stored in a "slot." A slot is a typed container for the output of a specific lens. Keeping slots separate is a core principle of Calyx ("no-flatten"). It ensures signals stay independent, allowing the system to know exactly *which* lens drove a specific result.

### The Grounding Kernel (Lodestar)
Most datasets contain redundant information. The "kernel" is the small subset of records that actually carries the structure of the entire corpus. Discovered by the Lodestar subsystem, the kernel acts as a central hub for navigation. It doubles as a fast index and an answer path—routing queries through the kernel first, then walking its edges to find answers.

### The Guard (Ward)
Calyx refuses to answer when it doesn't know. The Ward subsystem is a fail-closed boundary. Every required slot is scored independently against a calibrated threshold. If a query or generated content falls outside the trusted, grounded region, it is refused or quarantined. There are no silent wrong answers.

### The Ledger
Provenance is built-in. Every measurement, kernel discovery, and guard verdict is recorded in an append-only, hash-chained ledger. This ensures that you can always trace how an answer was produced and verify that the data hasn't been tampered with.

---

## 2. Architecture & Crates

Calyx is not a service mesh; it is an embedded engine built as a stack of focused Rust crates. It is structured into four main layers:

### Layer 1: Entry Points
These are the thin interfaces on top of the engine that you interact with directly.
- **`calyx-cli` (`calyx`)**: The command-line interface for direct interaction.
- **`calyxd`**: The background daemon for running Calyx as a service.
- **`calyx-mcp`**: The Model Context Protocol server, designed for AI agents to interact with Calyx.

### Layer 2: The Intelligence Layer
This layer turns basic retrieval into grounded intelligence.
- **`calyx-oracle` (Oracle)**: Predicts the grounded consequences of actions by mining recurrence patterns. Includes an honesty gate that refuses to guess when data is insufficient.
- **`calyx-anneal` (Anneal)**: Reversible self-optimization. It safely tunes index parameters, quantization, and fusion weights, rolling back if quality regresses.
- **`calyx-lodestar` (Lodestar)**: Discovers the grounding kernel of the corpus for routing and navigation.

### Layer 3: The Association Engine
This layer manages the lenses, signals, and how data points relate to one another.
- **`calyx-sextant` (Sextant)**: The search and navigation component. It handles vector indexes, BM25, and multi-signal fusion.
- **`calyx-loom` (Loom)**: Derives associations *between* slots (agreement, delta, interaction) and weaves them into a queryable graph.
- **`calyx-assay` (Assay)**: Measures the unique information (in bits) each lens contributes to outcomes, pruning redundant lenses.
- **`calyx-ward` (Ward)**: The fail-closed guard that independently scores slots to prevent out-of-distribution answers.
- **`calyx-registry` (Registry)**: Manages the lifecycle of lenses. Lenses are versioned, frozen, and content-addressed.

### Layer 4: Storage & Math (The Foundation)
The lowest layer handles disk I/O, heavy computation, and foundational data structures.
- **`calyx-aster` (Aster)**: Embedded LSM storage. It provides the write-ahead log, MVCC snapshots, and hot/cold tiering.
- **`calyx-forge` (Forge)**: The numeric runtime. It implements math operations twice—using CPU SIMD and CUDA—for fast, bit-near parity calculations and quantization.
- **`calyx-ledger` (Ledger)**: The append-only, hash-chained provenance log.
- **`calyx-core` (Core)**: The dependency-free foundation holding identifiers, error catalogs, the data model, and engine traits.

---

## 3. The Calculus of Association

Calyx organizes its operations around four main verbs:
1. **Measure**: Assemble a constellation by viewing one input through every lens in a panel (Registry, Aster).
2. **Count**: Derive associations between slots (Loom).
3. **Differentiate**: Quantify the unique information each lens adds (Assay).
4. **Compose**: Find the explanatory kernel, guard generation, and answer with provenance (Lodestar, Ward, Ledger).

By baking this machinery directly into the storage engine, Calyx eliminates the need to glue together separate vector DBs, keyword indexes, and graph databases. It provides one unified system where meaning is grounded, signals are typed, and answers are always accompanied by proof.
