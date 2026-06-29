use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TallyEnvelope {
    pub request_type: String,
    pub body: String,
}

impl TallyEnvelope {
    pub fn export_collection(collection_name: impl Into<String>) -> Self {
        let collection_name = collection_name.into();
        let body = format!(
            "<DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT></STATICVARIABLES><TDL><TDLMESSAGE><COLLECTION NAME=\"{}\" ISMODIFY=\"No\"><TYPE>Ledger</TYPE><FETCH>Name</FETCH></COLLECTION></TDLMESSAGE></TDL></DESC>",
            escape_xml_attr(&collection_name)
        );

        Self {
            request_type: "Export".into(),
            body,
        }
    }

    pub fn to_xml(&self) -> String {
        format!(
            "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>{}</TALLYREQUEST><TYPE>Collection</TYPE><ID>ListLedgers</ID></HEADER><BODY>{}</BODY></ENVELOPE>",
            escape_xml_text(&self.request_type),
            self.body
        )
    }
}

pub fn list_ledgers_request() -> String {
    TallyEnvelope::export_collection("Ledgers").to_xml()
}

fn escape_xml_attr(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_xml_text(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_list_ledgers_envelope() {
        let xml = list_ledgers_request();

        assert!(xml.contains("<TALLYREQUEST>Export</TALLYREQUEST>"));
        assert!(xml.contains("<COLLECTION NAME=\"Ledgers\""));
        assert!(xml.contains("<TYPE>Ledger</TYPE>"));
    }

    #[test]
    fn escapes_collection_name() {
        let xml = TallyEnvelope::export_collection("A&B").to_xml();

        assert!(xml.contains("A&amp;B"));
    }
}
