use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditAction {
    ProposedToolRun,
    ConfirmedToolRun,
    RejectedToolRun,
    CompletedToolRun,
    FailedToolRun,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditEvent {
    pub id: String,
    pub occurred_at: String,
    pub action: AuditAction,
    pub subject: String,
    pub details_json: String,
    pub previous_hash: String,
    pub event_hash: String,
}

impl AuditEvent {
    pub fn new(
        id: impl Into<String>,
        occurred_at: impl Into<String>,
        action: AuditAction,
        subject: impl Into<String>,
        details_json: impl Into<String>,
        previous_hash: impl Into<String>,
        event_hash: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            occurred_at: occurred_at.into(),
            action,
            subject: subject.into(),
            details_json: details_json.into(),
            previous_hash: previous_hash.into(),
            event_hash: event_hash.into(),
        }
    }

    pub fn action_label(&self) -> &'static str {
        match &self.action {
            AuditAction::ProposedToolRun => "ProposedToolRun",
            AuditAction::ConfirmedToolRun => "ConfirmedToolRun",
            AuditAction::RejectedToolRun => "RejectedToolRun",
            AuditAction::CompletedToolRun => "CompletedToolRun",
            AuditAction::FailedToolRun => "FailedToolRun",
        }
    }

    pub fn hash_payload(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}",
            self.id,
            self.occurred_at,
            self.action_label(),
            self.subject,
            self.details_json,
            self.previous_hash
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_audit_event() {
        let event = AuditEvent::new(
            "evt_1",
            "2026-06-29T00:00:00Z",
            AuditAction::ProposedToolRun,
            "tally.list_ledgers",
            "{}",
            "genesis",
            "hash_1",
        );

        assert_eq!(event.subject, "tally.list_ledgers");
        assert_eq!(event.action, AuditAction::ProposedToolRun);
        assert_eq!(event.previous_hash, "genesis");
        assert_eq!(event.event_hash, "hash_1");
    }

    #[test]
    fn hash_payload_includes_previous_hash() {
        let event = AuditEvent::new(
            "evt_1",
            "2026-06-29T00:00:00Z",
            AuditAction::CompletedToolRun,
            "tally.list_ledgers",
            "{}",
            "hash_0",
            "hash_1",
        );

        assert!(event.hash_payload().contains("hash_0"));
        assert!(event.hash_payload().contains("CompletedToolRun"));
    }

    #[test]
    fn action_label_is_explicit() {
        let event = AuditEvent::new(
            "evt_1",
            "2026-06-29T00:00:00Z",
            AuditAction::FailedToolRun,
            "tally.list_ledgers",
            "{}",
            "hash_0",
            "hash_1",
        );

        assert_eq!(event.action_label(), "FailedToolRun");
    }
}
