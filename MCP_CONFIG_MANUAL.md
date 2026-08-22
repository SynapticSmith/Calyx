# Calyx MCP Configuration Manual

This manual provides a comprehensive example of an MCP client configuration file for Calyx.
Per the requirements, the entire JSON structure of all available tools and arguments has been included directly within the configuration example.

## Complete Server Configuration Example

```json
{
  "mcpServers": {
    "calyx": {
      "command": "calyx-mcp",
      "args": [],
      "_exposed_tools": [
        {
          "description": "DDA abundance report",
          "inputSchema": {
            "properties": {
              "vault": {
                "type": "string"
              }
            },
            "required": [
              "vault"
            ],
            "type": "object"
          },
          "name": "calyx.abundance",
          "use_when": "DDA report: N, C(N,2), materialized, n_eff, DPI ceiling"
        },
        {
          "description": "add one frozen lens to a vault panel",
          "inputSchema": {
            "properties": {
              "endpoint": {
                "type": "string"
              },
              "modality": {
                "enum": [
                  "text",
                  "code",
                  "image",
                  "audio",
                  "video",
                  "structured",
                  "mixed"
                ],
                "type": "string"
              },
              "name": {
                "type": "string"
              },
              "runtime": {
                "enum": [
                  "tei-http",
                  "onnx",
                  "candle",
                  "algorithmic"
                ],
                "type": "string"
              },
              "shape": {
                "type": "string"
              },
              "vault": {
                "type": "string"
              },
              "weights": {
                "type": "string"
              }
            },
            "required": [
              "vault",
              "name",
              "runtime"
            ],
            "type": "object"
          },
          "name": "calyx.add_lens",
          "use_when": "add a measurement axis - the one call that replaces a whole pipeline"
        },
        {
          "description": "find constellations consistent with a stored constellation",
          "inputSchema": {
            "properties": {
              "cx_id": {
                "type": "string"
              },
              "slot": {
                "type": "integer"
              },
              "vault": {
                "type": "string"
              }
            },
            "required": [
              "vault",
              "cx_id"
            ],
            "type": "object"
          },
          "name": "calyx.agree",
          "use_when": "find constellations consistent with this one on a given lens"
        },
        {
          "description": "attach a grounded outcome",
          "inputSchema": {
            "properties": {
              "confidence": {
                "type": "number"
              },
              "cx_id": {
                "type": "string"
              },
              "kind": {
                "enum": [
                  "test_pass",
                  "thumbs_up",
                  "thumbs_down",
                  "speaker_match",
                  "style_hold",
                  "label"
                ],
                "type": "string"
              },
              "label": {
                "type": "string"
              },
              "source": {
                "type": "string"
              },
              "value": {
                "oneOf": [
                  {
                    "type": "boolean"
                  },
                  {
                    "type": "number"
                  }
                ]
              },
              "vault": {
                "type": "string"
              }
            },
            "required": [
              "vault",
              "cx_id",
              "kind",
              "value"
            ],
            "type": "object"
          },
          "name": "calyx.anchor",
          "use_when": "attach a grounded outcome (test pass, thumbs, label)"
        },
        {
          "description": "inspect self-optimization state",
          "inputSchema": {
            "properties": {
              "vault": {
                "type": "string"
              }
            },
            "required": [
              "vault"
            ],
            "type": "object"
          },
          "name": "calyx.anneal.status",
          "use_when": "self-optimization state, tripwires, proposals"
        },
        {
          "description": "full lineage of a kernel answer or search result",
          "inputSchema": {
            "properties": {
              "answer_id": {
                "type": "string"
              }
            },
            "required": [
              "answer_id"
            ],
            "type": "object"
          },
          "name": "calyx.answer_trace",
          "use_when": "full lineage of a kernel answer or search result"
        },
        {
          "description": "per-lens signal and panel sufficiency",
          "inputSchema": {
            "properties": {
              "anchor": {
                "type": "string"
              },
              "explain": {
                "type": "boolean"
              },
              "vault": {
                "type": "string"
              }
            },
            "required": [
              "vault",
              "anchor"
            ],
            "type": "object"
          },
          "name": "calyx.bits",
          "use_when": "per-lens signal + panel sufficiency + deficit attribution"
        },
        {
          "description": "create a durable Calyx vault",
          "inputSchema": {
            "properties": {
              "name": {
                "type": "string"
              },
              "panel_template": {
                "enum": [
                  "text-default",
                  "code-default",
                  "civic-default",
                  "media-default"
                ],
                "type": "string"
              }
            },
            "required": [
              "name"
            ],
            "type": "object"
          },
          "name": "calyx.create_vault",
          "use_when": "start a new database; picks text/code/civic/media-default panel"
        },
        {
          "description": "return the cross-lens definition for a lens coordinate",
          "inputSchema": {
            "properties": {
              "index": {
                "type": "integer"
              },
              "lens": {
                "type": "integer"
              },
              "vault": {
                "type": "string"
              }
            },
            "required": [
              "vault",
              "lens",
              "index"
            ],
            "type": "object"
          },
          "name": "calyx.define",
          "use_when": "get a term's grounded definition across the other lenses"
        },
        {
          "description": "find constellations anomalous relative to a stored constellation",
          "inputSchema": {
            "properties": {
              "cx_id": {
                "type": "string"
              },
              "slot": {
                "type": "integer"
              },
              "vault": {
                "type": "string"
              }
            },
            "required": [
              "vault",
              "cx_id"
            ],
            "type": "object"
          },
          "name": "calyx.disagree",
          "use_when": "find constellations that are anomalous relative to this one"
        },
        {
          "description": "calibrate a Gtau boundary",
          "inputSchema": {
            "properties": {
              "domain": {
                "type": "string"
              },
              "set": {
                "type": "string"
              },
              "target_far": {
                "type": "number"
              },
              "vault": {
                "type": "string"
              }
            },
            "required": [
              "vault",
              "domain",
              "set",
              "target_far"
            ],
            "type": "object"
          },
          "name": "calyx.guard.calibrate",
          "use_when": "calibrate the G\u03c4 boundary for a domain"
        },
        {
          "description": "apply a calibrated Gtau boundary",
          "inputSchema": {
            "properties": {
              "cx_id": {
                "type": "string"
              },
              "text": {
                "type": "string"
              },
              "vault": {
                "type": "string"
              }
            },
            "required": [
              "vault"
            ],
            "type": "object"
          },
          "name": "calyx.guard.check",
          "use_when": "apply the G\u03c4 boundary to a constellation or text"
        },
        {
          "description": "identity-locked generation gate",
          "inputSchema": {
            "properties": {
              "candidate_text": {
                "type": "string"
              },
              "identity_cx": {
                "type": "string"
              },
              "vault": {
                "type": "string"
              }
            },
            "required": [
              "vault",
              "candidate_text"
            ],
            "type": "object"
          },
          "name": "calyx.guard_generate",
          "use_when": "accept generated text only if it stays inside calibrated Gtau slots"
        },
        {
          "description": "ingest text into a Calyx vault",
          "inputSchema": {
            "properties": {
              "batch": {
                "items": {
                  "type": "string"
                },
                "type": "array"
              },
              "input": {
                "type": "string"
              },
              "vault": {
                "type": "string"
              }
            },
            "required": [
              "vault"
            ],
            "type": "object"
          },
          "name": "calyx.ingest",
          "use_when": "store data -> constellation (auto multi-lens, idempotent)"
        },
        {
          "description": "ingest retained image/audio/video bytes into a Calyx vault",
          "inputSchema": {
            "properties": {
              "file": {
                "type": "string"
              },
              "modality": {
                "enum": [
                  "image",
                  "audio",
                  "video"
                ],
                "type": "string"
              },
              "vault": {
                "type": "string"
              }
            },
            "required": [
              "vault",
              "file",
              "modality"
            ],
            "type": "object"
          },
          "name": "calyx.ingest_media",
          "use_when": "store raw media bytes -> derived text -> linked constellations"
        },
        {
          "description": "build or read the grounding kernel",
          "inputSchema": {
            "properties": {
              "anchor": {
                "type": "string"
              },
              "rebuild": {
                "type": "boolean"
              },
              "vault": {
                "type": "string"
              }
            },
            "required": [
              "vault"
            ],
            "type": "object"
          },
          "name": "calyx.kernel",
          "use_when": "build/get the grounding kernel + recall + grounding gaps"
        },
        {
          "description": "answer via the grounded kernel skeleton",
          "inputSchema": {
            "properties": {
              "anchor": {
                "type": "string"
              },
              "explain": {
                "type": "boolean"
              },
              "query": {
                "type": "string"
              },
              "vault": {
                "type": "string"
              }
            },
            "required": [
              "vault",
              "query"
            ],
            "type": "object"
          },
          "name": "calyx.kernel_answer",
          "use_when": "answer via the grounded kernel skeleton"
        },
        {
          "description": "list panel slots",
          "inputSchema": {
            "properties": {
              "vault": {
                "type": "string"
              }
            },
            "required": [
              "vault"
            ],
            "type": "object"
          },
          "name": "calyx.list_panel",
          "use_when": "see lenses, their bits signal, and state"
        },
        {
          "description": "measure text without storing it",
          "inputSchema": {
            "properties": {
              "input": {
                "type": "string"
              },
              "vault": {
                "type": "string"
              }
            },
            "required": [
              "vault",
              "input"
            ],
            "type": "object"
          },
          "name": "calyx.measure",
          "use_when": "get the constellation without storing (for guarding a candidate)"
        },
        {
          "description": "return per-lens neighbors of a stored constellation",
          "inputSchema": {
            "properties": {
              "cx_id": {
                "type": "string"
              },
              "k": {
                "maximum": 1000,
                "minimum": 1,
                "type": "integer"
              },
              "slot": {
                "type": "integer"
              },
              "vault": {
                "type": "string"
              }
            },
            "required": [
              "vault",
              "cx_id"
            ],
            "type": "object"
          },
          "name": "calyx.neighbors",
          "use_when": "per-lens neighborhood of a known constellation"
        },
        {
          "description": "park a panel slot",
          "inputSchema": {
            "properties": {
              "slot": {
                "type": "integer"
              },
              "vault": {
                "type": "string"
              }
            },
            "required": [
              "vault",
              "slot"
            ],
            "type": "object"
          },
          "name": "calyx.park_lens",
          "use_when": "sideline a lens without deleting its data"
        },
        {
          "description": "profile a candidate lens",
          "inputSchema": {
            "properties": {
              "endpoint": {
                "type": "string"
              },
              "modality": {
                "enum": [
                  "text",
                  "code",
                  "image",
                  "audio",
                  "video",
                  "structured",
                  "mixed"
                ],
                "type": "string"
              },
              "probe": {
                "type": "string"
              },
              "runtime": {
                "enum": [
                  "tei-http",
                  "onnx",
                  "candle",
                  "algorithmic"
                ],
                "type": "string"
              },
              "weights": {
                "type": "string"
              }
            },
            "required": [
              "runtime"
            ],
            "type": "object"
          },
          "name": "calyx.profile_lens",
          "use_when": "get a capability card before committing to a lens"
        },
        {
          "description": "propose a lens to close an intelligence gap",
          "inputSchema": {
            "properties": {
              "anchor": {
                "type": "string"
              },
              "max_ms_per_input": {
                "type": "number"
              },
              "max_ram_mb": {
                "type": "number"
              },
              "max_vram_mb": {
                "type": "number"
              },
              "vault": {
                "type": "string"
              }
            },
            "required": [
              "vault",
              "anchor"
            ],
            "type": "object"
          },
          "name": "calyx.propose_lens",
          "use_when": "ask Calyx what lens would close a sufficiency gap"
        },
        {
          "description": "full lineage of a constellation",
          "inputSchema": {
            "properties": {
              "cx_id": {
                "type": "string"
              },
              "vault": {
                "type": "string"
              }
            },
            "required": [
              "vault",
              "cx_id"
            ],
            "type": "object"
          },
          "name": "calyx.provenance",
          "use_when": "full lineage of a constellation"
        },
        {
          "description": "replay a claim to verify bit-parity",
          "inputSchema": {
            "properties": {
              "answer_id": {
                "type": "string"
              },
              "vault": {
                "type": "string"
              }
            },
            "required": [
              "vault",
              "answer_id"
            ],
            "type": "object"
          },
          "name": "calyx.reproduce",
          "use_when": "replay a claim to verify bit-parity"
        },
        {
          "description": "retire a panel slot",
          "inputSchema": {
            "properties": {
              "slot": {
                "type": "integer"
              },
              "vault": {
                "type": "string"
              }
            },
            "required": [
              "vault",
              "slot"
            ],
            "type": "object"
          },
          "name": "calyx.retire_lens",
          "use_when": "drop a low-signal lens permanently"
        },
        {
          "description": "search a Calyx vault",
          "inputSchema": {
            "properties": {
              "explain": {
                "type": "boolean"
              },
              "filter": {
                "type": "object"
              },
              "fresh": {
                "type": "boolean"
              },
              "fusion": {
                "enum": [
                  "rrf",
                  "weighted_rrf",
                  "single_lens",
                  "kernel_first",
                  "pipeline"
                ],
                "type": "string"
              },
              "guard": {
                "enum": [
                  "off",
                  "in_region"
                ],
                "type": "string"
              },
              "k": {
                "maximum": 1000,
                "minimum": 1,
                "type": "integer"
              },
              "query": {
                "type": "string"
              },
              "vault": {
                "type": "string"
              }
            },
            "required": [
              "vault",
              "query"
            ],
            "type": "object"
          },
          "name": "calyx.search",
          "use_when": "the everyday multi-lens search (RRF default, provenance attached)"
        },
        {
          "description": "search inside a named skill scope",
          "inputSchema": {
            "properties": {
              "query": {
                "type": "string"
              },
              "skill": {
                "type": "string"
              },
              "vault": {
                "type": "string"
              }
            },
            "required": [
              "vault",
              "skill",
              "query"
            ],
            "type": "object"
          },
          "name": "calyx.search_skill",
          "use_when": "search within a specific skill scope"
        },
        {
          "description": "return the hierarchical skill tree for a vault",
          "inputSchema": {
            "properties": {
              "vault": {
                "type": "string"
              }
            },
            "required": [
              "vault"
            ],
            "type": "object"
          },
          "name": "calyx.skills",
          "use_when": "hierarchical-skill navigation"
        },
        {
          "description": "walk the vault association graph from a constellation",
          "inputSchema": {
            "properties": {
              "cx_id": {
                "type": "string"
              },
              "direction": {
                "enum": [
                  "forward",
                  "backward",
                  "both"
                ],
                "type": "string"
              },
              "hops": {
                "maximum": 10,
                "minimum": 1,
                "type": "integer"
              },
              "vault": {
                "type": "string"
              }
            },
            "required": [
              "vault",
              "cx_id",
              "direction",
              "hops"
            ],
            "type": "object"
          },
          "name": "calyx.traverse",
          "use_when": "causal/asymmetric walk from a constellation"
        },
        {
          "description": "verify the ledger hash-chain",
          "inputSchema": {
            "properties": {
              "from_seq": {
                "type": "integer"
              },
              "to_seq": {
                "type": "integer"
              },
              "vault": {
                "type": "string"
              }
            },
            "required": [
              "vault"
            ],
            "type": "object"
          },
          "name": "calyx.verify_chain",
          "use_when": "tamper check: verify the Ledger hash-chain over a range"
        }
      ]
    }
  }
}
```
