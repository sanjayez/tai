# Security And Privacy

## Principles

- Accounting data stays on the customer machine.
- The desktop app must not make general internet calls.
- Any future license refresh path must be isolated and auditable.
- Local app state is encrypted at rest with SQLCipher.
- The SQLCipher key is random and wrapped by Windows DPAPI.
- Tool, model, and runtime manifests are signed and verified locally.
- Mutating Tally tools require explicit user confirmation.
- Proposed and executed accounting actions are written to an append-only hash-chained local audit log.
- No secrets, private API keys, server credentials, or plaintext proprietary prompt bundles are stored in local app files.

## Local Data

Local storage may contain settings, cache, conversation history, extracted document metadata, license state, manifest verification state, and audit logs. It must not be treated as the accounting source of truth.

Local storage should use:

```text
SQLite + SQLCipher
Windows DPAPI for key wrapping
minimal Tally data cache
append-only audit events
```

## Licensing

The customer app verifies a signed local license file or serial using an embedded public key. Expiry is checked locally.

## Manifests

Packaged artifacts should be described by signed manifests:

```text
tools.manifest.json
models.manifest.json
runtimes.manifest.json
```

Each manifest includes artifact names, versions, paths, SHA-256 hashes, and a signature. The app verifies the manifest before loading the artifact.

## Audit Log

Audit events must include:

```text
event id
timestamp
action
subject
details
previous event hash
event hash
```

The event hash is computed from the previous hash plus the current event payload, making local edits detectable.

## Threat Model

We defend against casual local inspection, copied database files, accidental data exposure, and simple tampering. We do not claim perfect secrecy against a determined reverse engineer with full control of the machine.

## Risk Register

- Tally XML write behavior may differ between TallyPrime and ERP 9.
- Local models may hallucinate tool intent, so tool execution must remain typed and permissioned.
- OCR output must be treated as a draft, never an automatic accounting entry.
- DPAPI protects keys at the Windows user/machine boundary; malware running as the same user remains a serious risk.
