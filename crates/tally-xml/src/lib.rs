use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TallyEnvelope {
    request_type: String,
    report_type: String,
    request_id: String,
    body: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TallyCollection {
    Ledgers,
}

impl TallyCollection {
    fn collection_name(self) -> &'static str {
        match self {
            TallyCollection::Ledgers => "Ledgers",
        }
    }

    fn object_type(self) -> &'static str {
        match self {
            TallyCollection::Ledgers => "Ledger",
        }
    }

    fn request_id(self) -> &'static str {
        match self {
            TallyCollection::Ledgers => "ListLedgers",
        }
    }
}

impl TallyEnvelope {
    pub fn export_collection(collection: TallyCollection) -> Self {
        let body = format!(
            "<DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT></STATICVARIABLES><TDL><TDLMESSAGE><COLLECTION NAME=\"{}\" ISMODIFY=\"No\"><TYPE>{}</TYPE><FETCH>Name</FETCH></COLLECTION></TDLMESSAGE></TDL></DESC>",
            escape_xml_attr(collection.collection_name()),
            escape_xml_text(collection.object_type())
        );

        Self {
            request_type: "Export".into(),
            report_type: "Collection".into(),
            request_id: collection.request_id().into(),
            body,
        }
    }

    pub fn to_xml(&self) -> String {
        format!(
            "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>{}</TALLYREQUEST><TYPE>{}</TYPE><ID>{}</ID></HEADER><BODY>{}</BODY></ENVELOPE>",
            escape_xml_text(&self.request_type),
            escape_xml_text(&self.report_type),
            escape_xml_text(&self.request_id),
            self.body
        )
    }
}

pub fn list_ledgers_request() -> String {
    TallyEnvelope::export_collection(TallyCollection::Ledgers).to_xml()
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
        assert_eq!(escape_xml_attr("A&B"), "A&amp;B");
    }
}
