# Test Strategy

## Required Gates

```powershell
cargo fmt --all --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
npm --prefix apps/desktop run typecheck
```

## Test Layers

Unit tests:

- XML builders.
- XML parsers.
- License verification.
- Manifest signature verification.
- SQLCipher key validation and DPAPI wrapper boundaries.
- Hash-chained audit event creation.
- Tool routing.
- Database repositories.

Golden tests:

- Known Tally XML input to expected domain output.
- Known tool request to expected XML envelope.

Integration tests:

- Mock Tally server.
- Detect Tally.
- List companies.
- List ledgers.
- Fetch vouchers.
- Dry-run voucher creation.
- Encrypted database creation and migration.
- Manifest verification before sidecar/model loading.
- Audit chain verification after tool runs.

Desktop smoke tests:

- App launches.
- License status renders.
- Tally status renders.
- "list ledgers" works against mock server.

Release tests:

- App installs.
- App starts offline.
- Encrypted database is created.
- Plain SQLite tools cannot inspect app database contents.
- Wrapped DB key is present, raw DB key is not stored.
- Model loads.
- Model hash matches signed manifest.
- OCR executable is found.
- Runtime hash matches signed manifest.
- Mock Tally flow works.
