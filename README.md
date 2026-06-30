# Tally AI Companion

Offline-first Windows desktop companion for TallyPrime and Tally ERP 9.

The customer runtime is a packaged desktop app:

```text
Tally AI Companion.exe
  Tauri desktop shell
  Rust backend
  SQLCipher encrypted local app data
  Windows DPAPI key wrapping
  Local LLM runtime
  Local OCR runtime
  Tally HTTP/XML tools
  Offline license verifier
  Signed tool/model manifests
  Hash-chained audit log
        |
        v
  TallyPrime / ERP 9 localhost XML API
```

No cloud service, hosted queue, customer dashboard, Postgres service, or Node.js runtime is required on customer machines after installation. Local state is encrypted and tamper-evident by default.

## Repository Layout

```text
apps/desktop/          Tauri customer app shell
crates/                Rust product logic
docs/                  Architecture, decisions, roadmap, notebook
fixtures/              Tally, Excel, and OCR test fixtures
models/                Local model placement docs; real models are not committed
scripts/               Development, test, and packaging helpers
```

## Development Runtime

- Rust stable with `clippy` and `rustfmt`.
- Node.js 20 or newer for desktop UI development and CI frontend checks.
- Windows for full Tauri packaging.

Node.js is a development and packaging dependency only. Tauri uses `npm run dev` to serve the React/Vite UI during development and `npm run build` to compile static frontend assets before packaging. The installed customer `.exe` does not run Node.js.

## Development Gates

```powershell
cargo fmt --all --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
npm --prefix apps/desktop run typecheck
```

The desktop shell is scaffolded separately from the Rust workspace so product crates remain quick to test while packaging work evolves.
