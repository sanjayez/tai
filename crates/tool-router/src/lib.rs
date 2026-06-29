use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConfirmationPolicy {
    Never,
    BeforeExternalRead,
    BeforeTallyWrite,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub confirmation_policy: ConfirmationPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolRoute {
    TallyListLedgers,
    Unknown,
}

pub fn core_tools() -> Vec<ToolDefinition> {
    vec![ToolDefinition {
        name: "tally.list_ledgers".into(),
        description: "List ledgers from the currently open Tally company.".into(),
        confirmation_policy: ConfirmationPolicy::Never,
    }]
}

pub fn route_text(input: &str) -> ToolRoute {
    let normalized = input.trim().to_lowercase();

    if normalized.contains("list") && normalized.contains("ledger") {
        return ToolRoute::TallyListLedgers;
    }

    ToolRoute::Unknown
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routes_list_ledgers() {
        assert_eq!(route_text("list ledgers"), ToolRoute::TallyListLedgers);
    }

    #[test]
    fn exposes_core_tool_definitions() {
        let tools = core_tools();

        assert_eq!(tools[0].name, "tally.list_ledgers");
        assert_eq!(tools[0].confirmation_policy, ConfirmationPolicy::Never);
    }
}

