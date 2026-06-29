# Product Brief

## One Sentence

Tally AI Companion is an offline Windows desktop app that helps accountants and business operators query, prepare, review, and safely write accounting work in Tally.

## Target Runtime

```text
Downloaded Windows installer
  -> installs one local desktop app
  -> contains LLM + OCR + Tally tools + local agent
  -> talks only to localhost / Tally
  -> no cloud dependency after download
  -> encrypts local state and verifies signed local artifacts
```

## Non Goals

- No customer-facing hosted dashboard.
- No hosted command queue.
- No requirement for Node.js on customer machines.
- No requirement for a local Postgres service.
- No replacement for Tally as the source of truth.
- No plaintext local database containing sensitive accounting or assistant state.

## First Useful Demo

The app opens offline, detects local Tally, accepts "list ledgers", routes that request to a local tool, executes it against Tally, and summarizes the result locally.
