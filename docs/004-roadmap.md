# Roadmap

## Milestone 0: Lab Setup

- Repository structure.
- Architecture docs.
- Rust workspace.
- Baseline CI.
- Fixture directories.
- Lab notebook.
- Secure-storage and manifest crate boundaries.

## Milestone 0.5: Local Security Foundation

- SQLCipher-backed database opening.
- Windows DPAPI key wrapping.
- Signed license verification using embedded public key.
- Signed tool/model/runtime manifest verification.
- Hash-chained audit event creation and verification.

## Milestone 1: Tally Read-Only Core

- Detect Tally.
- List companies.
- List ledgers.
- Fetch basic vouchers.
- Normalize XML to typed Rust structures.

## Milestone 2: Desktop Shell

- Tauri app launches.
- Shows license status.
- Shows Tally status.
- Runs read-only accounting queries.

## Milestone 3: Tool Router

- Deterministic routing for first tools.
- Typed inputs and outputs.
- Confirmation policies.
- Audit records for tool runs.

## Milestone 4: Local LLM Demo

- Bundle or locate llama.cpp sidecar.
- Load a small GGUF model.
- Route "list ledgers" through local model.
- Execute tool and summarize locally.

## Milestone 5: Safe Writes

- Voucher creation dry-run.
- User-confirmed write to Tally.
- Audit log for proposed and executed actions.

## Milestone 6: OCR Intake

- Local invoice image/PDF OCR.
- Extract fields.
- Propose voucher draft.
- Require confirmation.

## Milestone 7: Packaging

- Windows installer.
- App, sidecars, model placement, OCR runtime, migrations, and license verifier.
- Signed manifests for included sidecars, models, and tools.
- Encrypted database creation on install or first launch.
- Offline install/start smoke test.

## Milestone 8: Field Pilot

- Test with real Tally environments.
- Convert failures into fixtures.
- Harden UX and tool policies.
