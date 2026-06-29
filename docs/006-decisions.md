# Architecture Decisions

## ADR-001: Use SQLCipher-Backed SQLite For Local App Storage

Status: Accepted

Decision:

Use SQLite as the embedded local database for the customer desktop app, encrypted at rest with SQLCipher. Generate a random 256-bit database key and wrap that key with Windows DPAPI.

Reason:

The app is a single-machine desktop product. SQLite keeps installation simple and avoids service management, ports, credentials, backups, and support overhead from running Postgres on customer machines. Plain SQLite would expose sensitive local state to casual inspection, so the product uses SQLCipher and DPAPI key wrapping.

Revisit When:

We support shared LAN deployments, coordinated multi-user local servers, or unusually heavy local analytics.

## ADR-002: Keep Tauri Runtime Separate From Core Rust Workspace Initially

Status: Accepted

Decision:

Scaffold the Tauri app under `apps/desktop`, but keep `apps/desktop/src-tauri` out of the root Rust workspace for the initial lab setup.

Reason:

Core product crates should remain fast to build and test while desktop packaging dependencies evolve.

Revisit When:

The desktop command surface stabilizes and packaging becomes part of the normal CI gate.

## ADR-003: Treat Tally As The Source Of Truth

Status: Accepted

Decision:

The companion stores assistant state, cache, history, metadata, and audit records. Tally remains the accounting source of truth.

Reason:

This reduces data integrity risk and keeps the product aligned with existing accountant workflows.

Revisit When:

The product intentionally adds independent reporting or cross-company analytics that require a separate analytical store.

## ADR-004: Verify Signed Local Artifacts

Status: Accepted

Decision:

Verify signed manifests for packaged tools, models, and runtime sidecars before loading them.

Reason:

The app runs offline and ships executable/model artifacts locally. Signed manifests make accidental replacement and simple tampering detectable without requiring a cloud service.

Revisit When:

We introduce an online update channel with a stronger signed update framework.

## ADR-005: Use Hash-Chained Audit Logs

Status: Accepted

Decision:

Store audit events as append-only records containing the previous event hash and the current event hash.

Reason:

The app proposes and may execute accounting actions. Hash chaining makes local edits or deleted event history detectable during audit verification.

Revisit When:

We add external notarization or customer-controlled audit exports.
