# Calyx MCP Skill for AI Agents

You are an AI agent with access to the Calyx database via the Model Context Protocol (MCP). Calyx is an association-native database that stores 'Constellations' (inputs measured across multiple 'Lenses').

This document is your instruction manual on how to interact with the Calyx system. It provides details on all available tools and how to chain them together.

## Workflows and Tool Chaining

As an AI, you should understand how to combine tools to accomplish complex tasks:

### 1. Database Setup Workflow
- **Goal:** Initialize a new database for a user.
- **Steps:**
  1. Call `calyx.create_vault` to initialize the directory.
  2. If a default panel wasn't sufficient, call `calyx.add_lens` to add specific embedding models or extractors.

### 2. Data Ingestion Workflow
- **Goal:** Add knowledge to the database.
- **Steps:**
  1. Call `calyx.ingest` with the user's text or batch data.
  2. If the user provides a ground-truth label for the data, immediately call `calyx.anchor` on the resulting `cx_id`.

### 3. Reasoning and Search Workflow
- **Goal:** Answer a user's question using stored data.
- **Steps:**
  1. Call `calyx.search` to retrieve raw, multi-lens results.
  2. For a synthesized, high-level answer, call `calyx.kernel_answer`.
  3. To explore relationships around a specific constellation, use `calyx.neighbors` or `calyx.traverse`.

## Complete Tool Reference

### calyx.abundance

**Description:** DDA abundance report

**When to use (`use_when`):** DDA report: N, C(N,2), materialized, n_eff, DPI ceiling

**Arguments:**
- `vault` **(Required)** `string`

### calyx.add_lens

**Description:** add one frozen lens to a vault panel

**When to use (`use_when`):** add a measurement axis - the one call that replaces a whole pipeline

**Arguments:**
- `endpoint` (Optional) `string`
- `modality` (Optional) `string [enum: text, code, image, audio, video, structured, mixed]`
- `name` **(Required)** `string`
- `runtime` **(Required)** `string [enum: tei-http, onnx, candle, algorithmic]`
- `shape` (Optional) `string`
- `vault` **(Required)** `string`
- `weights` (Optional) `string`

### calyx.agree

**Description:** find constellations consistent with a stored constellation

**When to use (`use_when`):** find constellations consistent with this one on a given lens

**Arguments:**
- `cx_id` **(Required)** `string`
- `slot` (Optional) `integer`
- `vault` **(Required)** `string`

### calyx.anchor

**Description:** attach a grounded outcome

**When to use (`use_when`):** attach a grounded outcome (test pass, thumbs, label)

**Arguments:**
- `confidence` (Optional) `number`
- `cx_id` **(Required)** `string`
- `kind` **(Required)** `string [enum: test_pass, thumbs_up, thumbs_down, speaker_match, style_hold, label]`
- `label` (Optional) `string`
- `source` (Optional) `string`
- `value` **(Required)** `unknown`
- `vault` **(Required)** `string`

### calyx.anneal.status

**Description:** inspect self-optimization state

**When to use (`use_when`):** self-optimization state, tripwires, proposals

**Arguments:**
- `vault` **(Required)** `string`

### calyx.answer_trace

**Description:** full lineage of a kernel answer or search result

**When to use (`use_when`):** full lineage of a kernel answer or search result

**Arguments:**
- `answer_id` **(Required)** `string`

### calyx.bits

**Description:** per-lens signal and panel sufficiency

**When to use (`use_when`):** per-lens signal + panel sufficiency + deficit attribution

**Arguments:**
- `anchor` **(Required)** `string`
- `explain` (Optional) `boolean`
- `vault` **(Required)** `string`

### calyx.create_vault

**Description:** create a durable Calyx vault

**When to use (`use_when`):** start a new database; picks text/code/civic/media-default panel

**Arguments:**
- `name` **(Required)** `string`
- `panel_template` (Optional) `string [enum: text-default, code-default, civic-default, media-default]`

### calyx.define

**Description:** return the cross-lens definition for a lens coordinate

**When to use (`use_when`):** get a term's grounded definition across the other lenses

**Arguments:**
- `index` **(Required)** `integer`
- `lens` **(Required)** `integer`
- `vault` **(Required)** `string`

### calyx.disagree

**Description:** find constellations anomalous relative to a stored constellation

**When to use (`use_when`):** find constellations that are anomalous relative to this one

**Arguments:**
- `cx_id` **(Required)** `string`
- `slot` (Optional) `integer`
- `vault` **(Required)** `string`

### calyx.guard.calibrate

**Description:** calibrate a Gtau boundary

**When to use (`use_when`):** calibrate the Gτ boundary for a domain

**Arguments:**
- `domain` **(Required)** `string`
- `set` **(Required)** `string`
- `target_far` **(Required)** `number`
- `vault` **(Required)** `string`

### calyx.guard.check

**Description:** apply a calibrated Gtau boundary

**When to use (`use_when`):** apply the Gτ boundary to a constellation or text

**Arguments:**
- `cx_id` (Optional) `string`
- `text` (Optional) `string`
- `vault` **(Required)** `string`

### calyx.guard_generate

**Description:** identity-locked generation gate

**When to use (`use_when`):** accept generated text only if it stays inside calibrated Gtau slots

**Arguments:**
- `candidate_text` **(Required)** `string`
- `identity_cx` (Optional) `string`
- `vault` **(Required)** `string`

### calyx.ingest

**Description:** ingest text into a Calyx vault

**When to use (`use_when`):** store data -> constellation (auto multi-lens, idempotent)

**Arguments:**
- `batch` (Optional) `array`
- `input` (Optional) `string`
- `vault` **(Required)** `string`

### calyx.ingest_media

**Description:** ingest retained image/audio/video bytes into a Calyx vault

**When to use (`use_when`):** store raw media bytes -> derived text -> linked constellations

**Arguments:**
- `file` **(Required)** `string`
- `modality` **(Required)** `string [enum: image, audio, video]`
- `vault` **(Required)** `string`

### calyx.kernel

**Description:** build or read the grounding kernel

**When to use (`use_when`):** build/get the grounding kernel + recall + grounding gaps

**Arguments:**
- `anchor` (Optional) `string`
- `rebuild` (Optional) `boolean`
- `vault` **(Required)** `string`

### calyx.kernel_answer

**Description:** answer via the grounded kernel skeleton

**When to use (`use_when`):** answer via the grounded kernel skeleton

**Arguments:**
- `anchor` (Optional) `string`
- `explain` (Optional) `boolean`
- `query` **(Required)** `string`
- `vault` **(Required)** `string`

### calyx.list_panel

**Description:** list panel slots

**When to use (`use_when`):** see lenses, their bits signal, and state

**Arguments:**
- `vault` **(Required)** `string`

### calyx.measure

**Description:** measure text without storing it

**When to use (`use_when`):** get the constellation without storing (for guarding a candidate)

**Arguments:**
- `input` **(Required)** `string`
- `vault` **(Required)** `string`

### calyx.neighbors

**Description:** return per-lens neighbors of a stored constellation

**When to use (`use_when`):** per-lens neighborhood of a known constellation

**Arguments:**
- `cx_id` **(Required)** `string`
- `k` (Optional) `integer`
- `slot` (Optional) `integer`
- `vault` **(Required)** `string`

### calyx.park_lens

**Description:** park a panel slot

**When to use (`use_when`):** sideline a lens without deleting its data

**Arguments:**
- `slot` **(Required)** `integer`
- `vault` **(Required)** `string`

### calyx.profile_lens

**Description:** profile a candidate lens

**When to use (`use_when`):** get a capability card before committing to a lens

**Arguments:**
- `endpoint` (Optional) `string`
- `modality` (Optional) `string [enum: text, code, image, audio, video, structured, mixed]`
- `probe` (Optional) `string`
- `runtime` **(Required)** `string [enum: tei-http, onnx, candle, algorithmic]`
- `weights` (Optional) `string`

### calyx.propose_lens

**Description:** propose a lens to close an intelligence gap

**When to use (`use_when`):** ask Calyx what lens would close a sufficiency gap

**Arguments:**
- `anchor` **(Required)** `string`
- `max_ms_per_input` (Optional) `number`
- `max_ram_mb` (Optional) `number`
- `max_vram_mb` (Optional) `number`
- `vault` **(Required)** `string`

### calyx.provenance

**Description:** full lineage of a constellation

**When to use (`use_when`):** full lineage of a constellation

**Arguments:**
- `cx_id` **(Required)** `string`
- `vault` **(Required)** `string`

### calyx.reproduce

**Description:** replay a claim to verify bit-parity

**When to use (`use_when`):** replay a claim to verify bit-parity

**Arguments:**
- `answer_id` **(Required)** `string`
- `vault` **(Required)** `string`

### calyx.retire_lens

**Description:** retire a panel slot

**When to use (`use_when`):** drop a low-signal lens permanently

**Arguments:**
- `slot` **(Required)** `integer`
- `vault` **(Required)** `string`

### calyx.search

**Description:** search a Calyx vault

**When to use (`use_when`):** the everyday multi-lens search (RRF default, provenance attached)

**Arguments:**
- `explain` (Optional) `boolean`
- `filter` (Optional) `object`
- `fresh` (Optional) `boolean`
- `fusion` (Optional) `string [enum: rrf, weighted_rrf, single_lens, kernel_first, pipeline]`
- `guard` (Optional) `string [enum: off, in_region]`
- `k` (Optional) `integer`
- `query` **(Required)** `string`
- `vault` **(Required)** `string`

### calyx.search_skill

**Description:** search inside a named skill scope

**When to use (`use_when`):** search within a specific skill scope

**Arguments:**
- `query` **(Required)** `string`
- `skill` **(Required)** `string`
- `vault` **(Required)** `string`

### calyx.skills

**Description:** return the hierarchical skill tree for a vault

**When to use (`use_when`):** hierarchical-skill navigation

**Arguments:**
- `vault` **(Required)** `string`

### calyx.traverse

**Description:** walk the vault association graph from a constellation

**When to use (`use_when`):** causal/asymmetric walk from a constellation

**Arguments:**
- `cx_id` **(Required)** `string`
- `direction` **(Required)** `string [enum: forward, backward, both]`
- `hops` **(Required)** `integer`
- `vault` **(Required)** `string`

### calyx.verify_chain

**Description:** verify the ledger hash-chain

**When to use (`use_when`):** tamper check: verify the Ledger hash-chain over a range

**Arguments:**
- `from_seq` (Optional) `integer`
- `to_seq` (Optional) `integer`
- `vault` **(Required)** `string`
