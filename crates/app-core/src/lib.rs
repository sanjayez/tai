use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppStatus {
    pub offline_ready: bool,
    pub tally_endpoint: Option<tally_client::TallyEndpoint>,
    pub license_state: String,
}

impl AppStatus {
    pub fn initial() -> Self {
        Self {
            offline_ready: true,
            tally_endpoint: None,
            license_state: "unchecked".into(),
        }
    }
}

pub fn route_user_text(input: &str) -> tool_router::ToolRoute {
    tool_router::route_text(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_status_is_offline_ready() {
        let status = AppStatus::initial();

        assert!(status.offline_ready);
        assert_eq!(status.license_state, "unchecked");
    }

    #[test]
    fn routes_user_text_through_tool_router() {
        assert_eq!(
            route_user_text("Please list ledgers"),
            tool_router::ToolRoute::TallyListLedgers
        );
    }
}

