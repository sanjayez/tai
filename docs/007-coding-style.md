# Coding Style

## Ownership

- Rust owns product logic and side effects.
- TypeScript owns UI presentation.
- Tally XML creation stays in `tally-xml`.
- Tally HTTP behavior stays in `tally-client`.
- Tool permission and confirmation behavior stays in `tool-router`.
- SQLCipher and DPAPI key handling stay in `secure-storage`.
- Signed artifact verification stays in `manifest`.
- Audit hash-chain behavior stays in `audit-log`.

## Rust

- Prefer typed domain structs over unstructured JSON.
- Keep side effects behind traits when they touch Tally, local models, OCR, or the filesystem.
- Keep cryptographic verification and key handling behind small, testable boundaries.
- Use `Result` for fallible behavior.
- Keep fixture-driven tests close to the crate that owns the behavior.
- Avoid panics in product code.

## TypeScript

- Keep React components focused on state, layout, and invoking Tauri commands.
- Do not duplicate accounting or Tally behavior in the UI.
- Keep UI state typed.
- Prefer small components once a screen becomes hard to scan.

## Tally Tools

Every tool should define:

- name
- typed input
- typed output
- confirmation policy
- audit behavior
- manifest/hash coverage when the tool depends on a packaged local artifact

## Local Storage

- Do not open the app database outside `secure-storage`.
- Do not store raw database keys.
- Do not store private API keys or server credentials.
- Do not store proprietary prompt bundles as plaintext files.
- Keep cached Tally data minimal and purposeful.
- fixture coverage

## Documentation

- Update `docs/lab-notebook.md` after meaningful work sessions.
- Add an ADR to `docs/006-decisions.md` when a decision affects architecture, security, data storage, or packaging.
