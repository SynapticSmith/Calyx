# Calyx CLI Manual

Welcome to the Calyx CLI manual. Calyx is an association-native database designed for AI workloads. Instead of storing traditional rows or bare vectors, it stores **Constellations**—inputs measured across multiple frozen **Lenses** (embedding models and structured extractors) into a unified **Panel**. It utilizes GPU linear algebra to facilitate highly complex search, reasoning, and synthesis (often referred to as **Annealing**).

This manual serves as a comprehensive guide for developers, administrators, and users to understand the CLI tools available for building, managing, and querying Calyx databases.

## Glossary of Core Concepts

- **Vault:** The primary database unit in Calyx (similar to a table or database in relational systems). A Vault contains configuration, a ledger of operations, and the raw data.
- **Constellation (`cx`):** A core data record in Calyx. It contains the raw input data measured through multiple lenses. Instead of a traditional row, it's a multi-dimensional measurement of an input.
- **Lens:** An embedding model, extractor, or other transformation function. A lens maps raw data (text, images, audio, etc.) into vectors (or other shapes). Examples include algorithmic functions, local models (Candle, ONNX), or external HTTP endpoints.
- **Slot:** A specific placeholder within a Panel that is bound to a single Lens. It dictates the shape, modality, and resource constraints of the measurement.
- **Panel:** A specific versioned arrangement of Slots (and thus Lenses) applied to a Vault. It defines *how* a Vault "looks" at its data.
- **Annealing:** The process of optimizing, synthesizing, or organizing the data within a Vault. In Calyx, this often involves background optimization, deduplication, and refining the multi-lens representations.
- **FSV (File System Vault):** The raw bytes and layout of the Vault on disk. Calyx is often used to manipulate, verify, or recover this low-level representation.
- **A38 / A37:** Calyx internal terminologies for specific bundles, admission gates, or panel/lens configurations used for advanced validation and search (often seen in experimental/research commands).
- **Oracle / Hypothesis / Intelligence:** Advanced reasoning sub-systems that use the Vault's data to predict, expand, and evaluate hypotheses based on stored evidence.

## Quick Start & Common Workflows

Here are examples of the most common ways to interact with Calyx from start to finish.

### 1. Creating and Configuring a Vault

First, create a new vault. You can use an optional template to automatically set up a default Panel (a set of lenses appropriate for the domain).

```bash
calyx create-vault ./my-vault --panel-template text-default
```

### 2. Adding Lenses to the Vault

If you didn't use a template, or if you want to add more capabilities, you add Lenses to the Vault. This configures the models Calyx will use when ingesting data.

```bash
calyx add-lens ./my-vault --name text-embed --runtime candle-local --modality text
```

### 3. Ingesting Data

Add data to your Vault. You can insert a single string of text or point to a batch file. Calyx will process this input through the active Panel (all active Lenses) and store the resulting Constellation.

```bash
calyx ingest ./my-vault --text "Calyx is an association-native database."
calyx ingest ./my-vault --batch ./my-data.jsonl
```

### 4. Searching

Search the Vault for similarities using a query. Calyx uses the lenses to embed the query and find the nearest Constellations.

```bash
calyx search ./my-vault "What kind of database is Calyx?" --k 5
```

## Core Commands Reference

### Vault Management

Commands related to the lifecycle and health of a Vault database directory.

*   **`calyx create-vault <name> [--panel-template <template>]`**
    Creates a new Vault directory at `<name>`. You can optionally initialize it with a pre-configured set of lenses using `--panel-template` (options: `text-default`, `code-default`, `civic-default`, `legal-default`, `medical-default`, `bio-default`, `media-default`).
*   **`calyx retire-vault <vault> --reason <text> [args]`**
    Tombstones or gracefully retires a Vault, preventing further modification. You must provide a reason. Optionally, you can link it to a superseding Vault or corpus, reference a source issue, or attach FSV readback metadata.
*   **`calyx resource-status --vault <dir> [--metrics]`**
    Prints the current resource consumption (e.g., storage, disk pressure, memory) for the specified Vault. Use `--metrics` to output Prometheus-compatible metrics or detailed JSON.
*   **`calyx healthcheck [args]`**
    Runs a health check on the Calyx daemon or a specific vault. It verifies dependencies (like CUDA and VRAM) and vault readability. You can point it to a `calyx.toml` config, a specific vault, or ask it to wait for readiness.
*   **`calyx erase <vault> --cx-id <cx_id> [--fsv-out <json>]`**
    Hard-deletes a specific Constellation (identified by `<cx_id>`) from the Vault.

### Lens & Panel Management

Lenses are the embedding models or extractors, and Panels are the active configuration of those Lenses for a Vault.

*   **`calyx add-lens <vault> --name <n> --runtime <type> [args]`**
    Adds a new Lens to the Vault's active Panel. Requires a name and a runtime type (e.g., `algorithmic`, `tei-http`, `candle-local`, `onnx`). You can specify the `--endpoint`, `--weights` path, output `--shape` (e.g., `Dense(<dim>)`), and expected `--modality` (e.g., `text`, `image`).
*   **`calyx retire-lens <vault> --slot <u16>`**
    Permanently retires a Lens slot from the Vault's Panel. Future ingestions will no longer process data through this Lens.
*   **`calyx park-lens <vault> --slot <u16>`**
    Temporarily parks a Lens slot. It becomes inactive for new ingestions but remains in the panel definition and can potentially be unparked.
*   **`calyx list-panel <vault>`**
    Displays the current active Panel configuration for the Vault, listing all active and parked slots/lenses.
*   **`calyx profile-lens [args]`**
    Profiles the resource cost and latency of a specific Lens configuration without necessarily adding it to a Vault. Useful for benchmarking models before admission.
*   **`calyx propose-lens <vault> --anchor <kind>`**
    (Experimental) Proposes a new lens to improve the Vault's performance based on a specific anchor kind (e.g., targeting a specific evaluation metric).
*   **Panel Template Commands:**
    *   `calyx panel template seed [--home <dir>]`: Seeds default templates into the Calyx home directory.
    *   `calyx panel template save --name <name> ...`: Saves the current Lens configuration (or specified subset) as a reusable template.
    *   `calyx panel template list`: Lists available Panel templates.
    *   `calyx panel template fork --from <name-or-id> --name <name>`: Clones an existing template.
    *   `calyx panel template profile ...`: Benchmarks a template against assay cards.
    *   `calyx panel template swap --template <name-or-id> --vault <vault>`: Replaces a Vault's active Panel with the specified template.
*   **Panel Resident Commands:**
    Manages resident panels (keeping models loaded in GPU/CPU memory as a daemon for fast, repeated inference).
    *   `calyx panel resident serve ...`: Starts the resident panel server.
    *   `calyx panel resident ready`: Checks if the resident server is ready.
    *   `calyx panel resident measure ...`: Manually requests a measurement (inference) from the resident server.
    *   `calyx panel resident stop`: Shuts down the resident server.

### Data Ingestion & Anchoring

Commands for adding data to the Vault and providing feedback.

*   **`calyx ingest <vault> (args) [--idempotent]`**
    Processes input data through the Vault's Panel and stores the resulting Constellation. Provide input via `--text <s>`, `--batch <jsonl-path>`, or `--file <path> --modality <type>`.
*   **`calyx ingest-status <vault> --session <id>`**
    Checks the progress and status of a batch ingestion session.
*   **`calyx anchor <vault> <cx_id> --kind <kind> --value <v> [args]`**
    Attaches a feedback anchor to a specific Constellation. Anchors provide ground-truth or human feedback (e.g., `--kind thumbs-up`, `test-pass`, `speaker-match`) which is used during Annealing and search optimization.
*   **`calyx measure <vault> --text <s>`**
    A dry-run ingestion. It processes the text through the Vault's Panel and outputs the resulting vectors (measurements) without saving them to the Vault.

### Search & Query

Commands for retrieving and exploring data.

*   **`calyx search <vault> <query> [args]`**
    The primary search command. Finds Constellations similar to the `<query>`. You can specify the number of results (`--k`), the fusion strategy for combining multiple lens scores (`--fusion <rrf|weighted-rrf|pipeline>`), and apply metadata filters (`--filter`).
*   **`calyx probe-matrix <vault> --frontier <text> [args]`**
    An advanced search tool that probes the Vault using variations of a frontier text (adjusting phrasing, length, etc.) to comprehensively map the surrounding conceptual space.
*   **`calyx kernel-answer <vault> <query> [--anchor <kind>] [--explain]`**
    Queries the Vault's "Kernel" (an optimized, grounded summary or reasoning structure built over the data) to provide a synthesized answer rather than just returning raw Constellations.
*   **`calyx bits <vault> <anchor-kind> [--explain]`**
    Retrieves the raw bits/data associated with a specific anchor kind within the Vault.
*   **`calyx kernel <vault> [--anchor <kind>] [--rebuild]`**
    Manages or rebuilds the reasoning Kernel for a Vault.
*   **`calyx readback cx-list --vault <dir> [args]`**
    Lists all Constellation IDs in the Vault. Can be bounded by limits, include slot data, or write to a JSONL file. Useful for extracting the entire dataset.
*   **`calyx summarize --vault <dir> --scope <json|@file> --out <json> [args]`**
    Generates a summary of Constellations matching a specific scope or graph criteria.
*   **`calyx readback as-of --vault <dir> --t-millis <ms>`**
    (Time-Travel) Allows you to query or read the state of the Vault exactly as it was at a specific Unix timestamp in milliseconds.


### Annealing & Optimization

Annealing in Calyx is the background process of optimizing the Vault—tuning weights, removing duplicates, identifying structural holes, and building the kernel.

*   **`calyx anneal status --vault <dir> [args]`**
    Reports the current status of the background Annealer. Can be filtered by health (`--health`), faults (`--faults`), or specific tuners (e.g., `--tuner bw_postcutoff`).
*   **`calyx anneal soak ...` & `calyx anneal soak-report ...`**
    Runs and reports on a soak test (a sustained workload) to evaluate the Annealer's stability and performance over time.
*   **`calyx anneal autotune-report --scope <forge|index|storage> ...`**
    Prints reports generated by the Annealer's autotuning processes (optimizing index structures, storage layouts, or the "forge" synthesis pipeline).
*   **`calyx anneal goodhart-check ...`**
    Evaluates whether the current optimizations have fallen victim to Goodhart's Law (over-optimizing for a metric at the expense of actual quality), using a provided JSON fixture.
*   **`calyx anneal deficit-map ...` & `calyx anneal propose-preview ...`**
    Tools for analyzing where the Vault's current Lenses are failing to distinguish important data (deficits) and previewing proposed solutions.
*   **`calyx readback config <tripwire|budget> --vault <dir>`**
    Configures operational limits, such as memory budgets or tripwires (safety stops) for the Annealer and resident panels.
*   **Deduplication Commands:**
    *   `calyx dedup-check ...`: Dry-run check for duplicates against a specific Constellation using cosine similarity thresholds.
    *   `calyx dedup-audit ...`: Audits the deduplication decisions made for a Constellation.
    *   `calyx dedup-undo ...`: Reverses a deduplication action using a token.

### Benchmarking & Validation (Assay & Sextant)

These tools are used to rigorously test the recall, accuracy, and performance of Vault configurations (Lenses, search strategies) against ground-truth datasets.

*   **`calyx bench search ...` & `calyx bench recall ...`**
    Runs basic search performance and recall benchmarks against the Vault.
*   **`calyx bench partitioned-search ...` & `calyx bench partitioned-rrf ...`**
    Advanced benchmarks that evaluate search strategies across partitioned index spaces and complex fusion plans (RRF - Reciprocal Rank Fusion).
*   **`calyx build-bench-vault ...` & `calyx build-partitioned-vault ...`**
    Constructs synthetic or highly controlled Vaults specifically for running these benchmarks.
*   **`calyx assay corpus-build ...`**
    Builds a testing corpus from JSONL rows, categorizing them for validation tasks.
*   **`calyx assay ensemble-card ...` & `calyx assay i8bin-ensemble-card ...`**
    Generates an "ensemble card"—a detailed report analyzing how well a combination of lenses (an ensemble) performs on a specific dataset or target class.
*   **`calyx sextant recall-validate ...` & `calyx lodestar kernel-validate ...`**
    Validates the recall of standard searches (Sextant) or the accuracy of Kernel-based answers (Lodestar) against provided query-relevance (qrels) datasets.
*   **Media Validation:** `calyx media image-validate`, `calyx media emotion-validate`, `calyx media video-validate` run specific validation suites for non-text modalities.

### Hypothesis, Discovery & Oracle (Advanced AI)

These commands interact with Calyx's higher-level reasoning engines, allowing the system to propose hypotheses, validate associations, and traverse knowledge graphs.

*   **`calyx discovery-chain <vault> ...` & `calyx chain-walks <vault> ...`**
    Initiates a random walk or directed exploration through the Vault's constellation graph to discover novel associations or build evidence chains starting from a seed.
*   **Hypothesis Pipeline:**
    *   `calyx assemble-hypothesis-evidence ...`: Gathers evidence for a generated chain.
    *   `calyx hypothesis-evaluator-driver ...` & `calyx hypothesis-evaluate ...`: Evaluates the strength of a hypothesis using external models (via HTTP endpoints) or internal metrics.
    *   `calyx hypothesis-rank ...`: Ranks evaluated hypotheses.
    *   `calyx hypothesis-falsification-sweep ...`: Attempts to falsify hypotheses against external databases (like PubTator or ClinicalTrials).
*   **Oracle Commands:**
    *   `calyx readback oracle_predict ...`, `calyx readback oracle_expand ...`, `calyx readback oracle_self_consistency ...`, `calyx readback oracle_sufficiency ...`: Interfaces with the Oracle sub-system to predict outcomes, expand contexts, and check the internal consistency and sufficiency of the knowledge base.
    *   `calyx readback reverse_query ...`: Given an answer and a domain, attempts to reconstruct the query that would yield that answer.
*   **`calyx reproduce <vault> <answer_id>`**
    Re-runs the exact steps taken to generate a specific answer or hypothesis to verify reproducibility.

### Leapable (SQLite Migration & Dual-Write)

Leapable is the subsystem for migrating data from traditional SQLite databases into Calyx Vaults and managing dual-write states during transition.

*   **`calyx migrate vault <sqlite.db> <vault.calyx> [args]`**
    Migrates an entire SQLite database into a Calyx Vault.
*   **`calyx migrate backfill ...` & `calyx migrate verify ...` & `calyx migrate status ...`**
    Commands to backfill missing data, verify the integrity of the migration, and check progress.
*   **`calyx leapable dual-write ...` & `calyx leapable read-flip ...` & `calyx leapable remove-shadow ...`**
    Manages the transitional state where data is written to both SQLite and Calyx, eventually flipping reads to Calyx, and finally removing the SQLite "shadow".
*   **`calyx leapable ask ...` & `calyx leapable recall-compare ...`**
    Tools to query the Leapable interface and compare recall performance between the legacy SQLite system and the new Calyx Vault.

### Low-Level Debugging & FSV Management

Commands meant for developers and administrators to inspect the raw file system state of a Vault or recover from crashes.

*   **`calyx readback [options]`**
    A powerful, multifaceted command for inspecting the raw state. It can read Vault trees, specific column families (`--cf-row`), Write-Ahead Logs (`--wal`), ledgers, or temporal/dedup statuses. It essentially prints source-of-truth bytes or listings for manual FSV inspection.
*   **`calyx verify-chain [args]` & `calyx verify-restore ...`**
    Verifies the cryptographic integrity of the Vault's ledger chain and tests restoration procedures.
*   **`calyx fsv corpus-readback ...` & `calyx fsv vault-health ...`**
    Low-level checks on the File System Vault (FSV) layout and data integrity.
*   **`calyx scan --cf ledger --vault <dir>`**
    Scans and prints the contents of a specific Column Family (like the ledger).
*   **`calyx compact --vault <dir> --cf <name>`**
    Manually triggers compaction on a specific Column Family to reclaim disk space.
*   **`calyx recover --vault <dir>`**
    Attempts to recover a Vault that has crashed or been corrupted by replaying logs.
*   **`calyx wal-replay <wal-dir>` & `calyx wal-drill ...`**
    Tools for inspecting and replaying the Write-Ahead Log.
*   **`calyx crash-drill ...` & `calyx corrupt-shard ...`**
    Chaos engineering tools used by developers to intentionally crash the system at specific points or corrupt data to test recovery mechanisms.
*   **`calyx build-info`**
    Prints build information (version, git commit, rustc version, etc.) for the Calyx binary.


### Graph Navigation & Skills

These commands allow traversing and analyzing the constellation graph to find neighbors, define skills, and explore relationships based on a JSON specification.

*   **`calyx navigate neighbors --spec <json> --cx <cx> ...`**
    Finds nearest neighbors for a specific constellation based on the navigation spec.
*   **`calyx navigate define --spec <json> --cx <cx> ...`**
    Defines or maps out a region around a constellation.
*   **`calyx navigate agree --spec <json> --anchor <cx> ...`**
    Finds constellations that "agree" with or align closely with an anchor constellation across specified slots.
*   **`calyx navigate disagree --spec <json> --anchor <cx> ...`**
    Finds constellations that diverge or disagree with the anchor.
*   **`calyx navigate traverse --spec <json> --anchor <cx> ...`**
    Traverses the graph step-by-step from an anchor, discovering multi-hop associations.
*   **`calyx navigate skills --spec <json> ...`**
    Analyzes the graph to identify distinct clusters or "skills" represented within the data.
*   **`calyx navigate search-skill --spec <json> --skill <name> ...`**
    Searches within a specifically identified skill cluster.

### Advanced Lens & Manifest Operations

Detailed commands for managing lens models, their configurations (manifests), and commissioning them into the ecosystem.

*   **`calyx lens add --manifest <manifest.json> [--home <dir>]`**
    Adds a new lens definition to the local Calyx environment using a JSON manifest.
*   **`calyx lens card --manifest <manifest.json> ...`**
    Displays the capability card for a specific lens manifest, optionally testing it against an input text.
*   **`calyx lens list [--home <dir>]`**
    Lists all lens manifests registered in the local environment.
*   **`calyx lens remove (--name <name>|--lens-id <id>) ...`**
    Removes a lens manifest from the local environment.
*   **`calyx lens commission --hf <id> --runtime <type> ...`**
    Downloads and commissions a new model from Hugging Face or FastEmbed, compiling/optimizing it for the specified runtime (e.g., `onnx-int8`, `candle-fp16`).
*   **`calyx lens explain --manifest <manifest.json> ...`**
    Provides detailed debugging output explaining how a specific lens transforms an input into a vector.
*   **`calyx lens scale-audit --manifest <manifest.json> ...`**
    Audits the performance scaling and batching capabilities of one or more lens manifests across available hardware.
*   **`calyx readback vault-manifest --field <name> --vault <dir>`**
    (via `readback`) Extracts a specific field from a Vault's internal manifest file.

### Materialization & Graph Substrates

Commands used to ingest specialized external datasets (often medical/scientific) and convert them into the Calyx graph structure.

*   **`calyx materialize-bridge-corpus <name> --rows <jsonl> ...`**
    Materializes a general bridge corpus from JSONL.
*   **`calyx materialize-molecular-vault <vault> --rows <jsonl> ...`**
    Specifically imports molecular structure data into a vault.
*   **`calyx materialize-evidence-substrate ...`**
    Builds a massive evidence graph by ingesting databases like PubTator, ClinicalTrials, and DGIdb.
*   **`calyx materialize-lincs-reversal <vault> --root <dir> ...`**
    Materializes LINCS (Library of Integrated Network-Based Cellular Signatures) transcriptomic data for drug reversal analysis.
*   **`calyx graph-collection-generations <vault> ...`**
    Lists the version history (generations) of a specialized graph collection.
*   **`calyx graph-collection-state <vault> ...`**
    Modifies or reports the state (`writing`, `accepted`, `failed`, `tombstoned`) of a specific graph collection generation.

### Assays, Associations & Validation Pipelines

Tools for rigorously validating associations discovered in the graph against external knowledge bases.

*   **`calyx association-validation-gates ...`**
    Evaluates discovered associations against multiple external roots (Open Targets, PubTator, etc.) to gate/approve them.
*   **`calyx typed-association-miner ...`**
    Mines specific typed associations (e.g., Gene -> Disease) using a validation report.
*   **`calyx weave-loom <vault> ...`**
    An experimental command that attempts to connect disconnected subgraphs or "weave" missing edges in the Vault's internal representation.
*   **`calyx novelty-calibration-split ...`**
    Splits datasets to calibrate the "novelty" scoring of discoveries (ensuring the system prefers genuinely new insights over redundant ones).
*   **`calyx bridge-falsification-evaluate ...` & `calyx bridge-evaluate-rank ...`**
    Evaluates and ranks "bridges" (multi-hop hypotheses) specifically checking for falsification evidence.

### Additional Advanced Commands

*   **`calyx guard <vault> <calibrate|check|generate> [args]`**
    Manages semantic safety guards. `calibrate` sets boundaries based on the current panel, `check` verifies if an input violates them, and `generate` creates adversarial test cases.
*   **`calyx get-provenance --vault <dir> --cx <cx-id>`**
    Retrieves the exact cryptographic and logical history (provenance) of a specific Constellation, showing how and when it was derived.
*   **`calyx get-answer-trace --vault <dir> --answer <answer-id-or-hex>`**
    Retrieves the execution trace used to generate a specific Oracle answer.
*   **`calyx rebuild-search-index <vault>`**
    Manually forces a complete rebuild of the vector search indices (usually handled automatically by Anneal).
*   **`calyx kernel-build <vault> [args]`**
    Forces an immediate build of the reasoning Kernel, bypassing the background Annealer schedule.
*   **`calyx panel batch-limit --vault <vault> --set <name-or-id>=<max_batch> ...`**
    Overrides the auto-detected maximum batch sizes for specific lenses in the Vault's panel.
*   **`calyx panel registry-audit --vault <vault>` & `calyx panel registry-repair ...`**
    Audits or repairs the Panel registry within the Vault if corruption is suspected.
*   **`calyx panel manifest-restore --vault <vault> ...`**
    Restores a specific panel configuration from raw asset files in the Vault's FSV.
*   **`calyx panel warm --template <name-or-id> ...`**
    Pre-loads a panel template into memory (warming the models) without starting a persistent server, useful for benchmarking load times.
*   **`calyx panel a38-bundle ...` (save, list)**
    Manages specific A38 capability bundles (highly curated sets of lenses and evidence requirements).
*   **`calyx discovery-run (seal | reproduce | verify) ...`**
    Manages reproducible discovery runs. `seal` locks down a run, `reproduce` re-executes it, and `verify` checks it against the ledger.
*   **`calyx audit --vault <dir> --kind <kind>`**
    Runs a specialized integrity audit on the Vault data.
*   **`calyx compact-watch --vault <dir> --duration <time>`**
    Monitors the background compaction process in real-time.
*   **`calyx tier --vault <dir> --cf <name> --output <hot|cold>`**
    Manages data tiering, moving specific column families between hot (fast/expensive) and cold (slow/cheap) storage.
*   **`calyx open-check --vault <dir> --index <n>`**
    A low-level diagnostic to verify if specific internal indices can be successfully opened.
*   **`calyx vault-demo`, `calyx arrow-demo`, `calyx cf-demo`, `calyx mvcc-demo`, `calyx wal-batch-demo`**
    Developer commands that run specific internal technology demonstrations or micro-benchmarks on a test Vault.
*   **`calyx merkle-root (--ledger | --vault) ...`**
    Computes the Merkle root hash for a specific range of the Vault's ledger to cryptographically verify data integrity.


### Additional Readback Commands

The `calyx readback` command is a multi-tool for inspecting various low-level states. In addition to the ones mentioned earlier, it supports these sub-topics:

*   **`calyx readback temporal_search ...`**
    Explains the internal parameters used for time-weighted search queries.
*   **`calyx readback kernel-health ...`**
    Checks the health of a specific kernel ID in the raw ledger.
*   **`calyx readback recurrence-series ...`**
    Reads the recurrence series data for a specific constellation.
*   **`calyx readback periodic-recall ...`**
    Tests the recall of data at specific periodic boundaries (e.g., specific hours or days).
*   **`calyx readback time-index ...`**
    Dumps the contents of the time-series index.
