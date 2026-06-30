# Project Instructions

This repository is for an offline-first Windows desktop companion for Tally.

## Product Constraints

- The customer runtime must work without internet after installation.
- Tally remains the accounting system of record.
- The desktop app must not require Node.js, Postgres, or a hosted command queue on customer machines.
- Local app storage must use encrypted SQLite through SQLCipher, with DB keys wrapped by Windows DPAPI.
- Tool, model, and runtime manifests must be signed and verified before use.
- Any write to Tally must require explicit user confirmation and must be audit logged.
- Audit logs must be append-only and tamper-evident with hash chaining.
- Cloud systems are out of scope for the customer runtime, except a future isolated license refresh path.

## CodeGraph

In repositories indexed by CodeGraph (a `.codegraph/` directory exists at the repo root), reach for it before grep/find or reading files when you need to understand or locate code:

- MCP tools, when available: `codegraph_explore` and `codegraph_node`.
- Shell fallback: `codegraph explore "<symbol names or question>"` and `codegraph node <symbol-or-file>`.

If there is no `.codegraph/` directory, skip CodeGraph entirely.

## Engineering Rules

- Rust owns product logic: Tally access, tools, licensing, persistence, model calls, OCR, and audit logging.
- TypeScript owns desktop UI presentation and calls into Tauri commands.
- Keep XML builders and parsers centralized.
- Prefer typed inputs and outputs for every tool.
- Use golden fixtures for Tally XML behavior.
- Do not store secrets, private API keys, or plaintext proprietary prompt bundles in local app files.
- Update `docs/lab-notebook.md` after meaningful work sessions.
- Record durable architecture decisions in `docs/006-decisions.md`.
