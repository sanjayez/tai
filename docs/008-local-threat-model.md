# Local Threat Model

## Posture

The app is offline and runs on a customer-controlled Windows machine. Security should protect against casual inspection, copied files, accidental exposure, and simple tampering. It should not pretend to defeat a determined reverse engineer with full local control.

## Protected Assets

- Local assistant state.
- Conversation history, if enabled.
- Cached Tally metadata.
- License state.
- Tool run history.
- Audit logs.
- Proprietary prompts and policies.
- Packaged tool/model/runtime integrity.

## Controls

```text
SQLite + SQLCipher
Windows DPAPI key wrapping
signed license files
signed tool/model/runtime manifests
append-only audit logs with hash chaining
no secrets or proprietary prompts stored in plaintext
no server credentials in the app
minimal local cache of Tally data
```

## Database Key Flow

First launch:

```text
generate random 256-bit DB key
wrap key using Windows DPAPI
store wrapped key in app data
open SQLCipher database
```

Later launches:

```text
read wrapped key from app data
unwrap key using Windows DPAPI
open SQLCipher database
```

## Non Goals

- Perfect protection against malware running as the same Windows user.
- Perfect protection against reverse engineering.
- Keeping distributable local models secret.
- Using Postgres as a local security boundary.

## Review Checklist

- Does this change write sensitive data locally?
- Is the data necessary to store?
- Is it covered by SQLCipher storage?
- Does it need audit logging?
- Does it rely on a packaged artifact that needs manifest verification?
- Does it introduce any local plaintext secret?
