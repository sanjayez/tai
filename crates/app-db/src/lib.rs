pub const INITIAL_SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS audit_events (
  id TEXT PRIMARY KEY NOT NULL,
  occurred_at TEXT NOT NULL,
  action TEXT NOT NULL,
  subject TEXT NOT NULL,
  details_json TEXT NOT NULL,
  previous_hash TEXT NOT NULL,
  event_hash TEXT NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_audit_events_event_hash
  ON audit_events(event_hash);

CREATE TABLE IF NOT EXISTS app_settings (
  key TEXT PRIMARY KEY NOT NULL,
  value TEXT NOT NULL,
  updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS signed_manifest_state (
  kind TEXT PRIMARY KEY NOT NULL,
  version TEXT NOT NULL,
  signature TEXT NOT NULL,
  verified_at TEXT NOT NULL
);
"#;

pub fn migration_names() -> &'static [&'static str] {
    &["0001_initial_schema"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_schema_contains_audit_events() {
        assert!(INITIAL_SCHEMA.contains("CREATE TABLE IF NOT EXISTS audit_events"));
        assert!(INITIAL_SCHEMA.contains("idx_audit_events_event_hash"));
        assert!(INITIAL_SCHEMA.contains("previous_hash"));
        assert!(INITIAL_SCHEMA.contains("event_hash"));
    }

    #[test]
    fn migrations_are_ordered() {
        assert_eq!(migration_names(), &["0001_initial_schema"]);
    }
}
