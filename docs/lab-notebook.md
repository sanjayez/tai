# Lab Notebook

## 2026-06-29

Goal:

- Establish the disciplined baseline for the offline Tally AI Companion project.

Changed:

- Added product architecture docs.
- Added Rust workspace layout for core product crates.
- Added Tauri desktop shell scaffold.
- Added fixture, model, script, and CI structure.

Learned:

- The repository started empty except for Git metadata.
- There is no `.codegraph/` index yet.

Decisions:

- Use SQLite for local embedded app storage.
- Keep the Tauri Rust package outside the workspace until packaging becomes a CI concern.
- Keep Tally as the accounting source of truth.

Risks:

- Real Tally XML samples are still needed to harden fixtures.
- OCR and model runtime choices need benchmarks on typical customer Windows machines.

Next:

- Implement Tally detection against a mock Tally server.
- Add first XML golden fixture for `list_ledgers`.

## 2026-06-29 Security Baseline Update

Goal:

- Integrate the local storage and tamper-evidence security posture into the scaffold.

Changed:

- Added `secure-storage` crate boundary for SQLCipher and Windows DPAPI key wrapping.
- Added `manifest` crate boundary for signed tool/model/runtime manifests.
- Updated audit events to include previous and current hash fields.
- Updated architecture, security, roadmap, test strategy, decisions, and coding style docs.
- Added a local threat model document.

Decisions:

- Plain SQLite is not acceptable for sensitive local app state.
- Postgres is not a security boundary for a local desktop app.
- Signed local artifacts and hash-chained audit logs are baseline requirements.

Next:

- Implement real SQLCipher database opening.
- Implement Windows DPAPI wrapper.
- Choose signature scheme for licenses and manifests.
