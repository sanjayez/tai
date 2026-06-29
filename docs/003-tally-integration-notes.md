# Tally Integration Notes

## Current Assumptions

- Tally exposes a local HTTP XML interface.
- The first supported workflows are read-only discovery and listing data.
- Mutating workflows start as dry-runs before confirmed writes.

## Initial Tool Surface

- Detect Tally.
- List companies.
- List ledgers.
- Fetch vouchers.
- Prepare voucher draft.
- Write voucher after confirmation.

## Fixture Policy

Every Tally XML behavior that becomes product logic should have a fixture:

```text
fixtures/tally/
  requests/
  responses/
  normalized/
```

