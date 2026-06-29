use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TallyEndpoint {
    pub host: String,
    pub port: u16,
}

impl TallyEndpoint {
    pub fn localhost(port: u16) -> Self {
        Self {
            host: "127.0.0.1".into(),
            port,
        }
    }

    pub fn url(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TallyToolRequest {
    ListLedgers,
}

pub fn default_discovery_ports() -> &'static [u16] {
    &[9000]
}

pub fn request_xml(request: &TallyToolRequest) -> String {
    match request {
        TallyToolRequest::ListLedgers => tally_xml::list_ledgers_request(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_localhost_url() {
        assert_eq!(TallyEndpoint::localhost(9000).url(), "http://127.0.0.1:9000");
    }

    #[test]
    fn maps_list_ledgers_to_xml() {
        let xml = request_xml(&TallyToolRequest::ListLedgers);

        assert!(xml.contains("<TYPE>Ledger</TYPE>"));
    }
}

