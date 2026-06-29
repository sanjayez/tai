# Architecture

## Customer Runtime

```text
Tally AI Companion.exe / installer
  Tauri desktop shell
  Rust local backend
  SQLCipher encrypted SQLite/app data
  Windows DPAPI key wrapping
  Embedded or sidecar llama.cpp runtime
  Packaged GGUF model
  Packaged OCR engine/model
  Tally HTTP/XML module
  Tool router + confirmations
  Offline signed license verifier
  Signed tool/model/runtime manifests
  Hash-chained append-only audit log
        |
        v
  TallyPrime / ERP 9 local HTTP XML API
```

## Boundaries

- Rust owns app state, Tally communication, tools, persistence, licensing, LLM/OCR invocation, and audit logging.
- TypeScript owns the desktop interface and calls Tauri commands.
- SQLCipher-backed SQLite stores local app state, cache, settings, history, extracted document metadata, and audit logs.
- Windows DPAPI wraps the local database key; the unwrapped key exists only at runtime.
- Signed manifests verify packaged tools, models, and sidecar runtimes before use.
- Tally remains the accounting system of record.

## Crates

- `tally-xml`: XML builders, parser helpers, and golden fixture tests.
- `tally-client`: localhost discovery and Tally HTTP transport.
- `tool-router`: typed tool definitions, routing, confirmation policies.
- `local-llm`: llama.cpp sidecar or linked runtime abstraction.
- `local-ocr`: OCR abstraction and extracted field contracts.
- `license`: offline signed license verification.
- `manifest`: signed model, tool, and runtime manifest verification.
- `secure-storage`: SQLCipher database opening, Windows DPAPI key wrapping boundary, and migration execution.
- `app-db`: SQLite schema, migrations, and repositories.
- `audit-log`: immutable hash-chained local event records.
- `app-core`: orchestration across product crates.

## Local Security Flow

First launch:

```text
generate random 256-bit DB key
wrap key with Windows DPAPI
store wrapped key in app data
create encrypted SQLCipher DB
verify signed manifests
```

Later launches:

```text
read wrapped key
unwrap key with Windows DPAPI
open SQLCipher DB
run migrations
verify signed manifests
```

## Offline Rule

Every milestone should pass this test:

```text
Disconnect internet.
Launch app.
Connect to local Tally.
Perform useful accounting work.
```
