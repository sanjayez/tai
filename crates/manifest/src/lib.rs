use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManifestKind {
    Tool,
    Model,
    Runtime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestArtifact {
    pub name: String,
    pub version: String,
    pub path: String,
    pub sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedManifest {
    pub kind: ManifestKind,
    pub artifacts: Vec<ManifestArtifact>,
    pub signature: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManifestStatus {
    Trusted,
    InvalidSignature,
    Empty,
}

pub trait ManifestSignatureVerifier {
    fn verify(&self, payload: &str, signature: &str) -> bool;
}

impl SignedManifest {
    pub fn signing_payload(&self) -> String {
        #[derive(Serialize)]
        struct SigningPayload<'a> {
            kind: &'static str,
            artifacts: &'a [ManifestArtifact],
        }

        let payload = SigningPayload {
            kind: self.kind.label(),
            artifacts: &self.artifacts,
        };

        serde_json::to_string(&payload).expect("manifest signing payload should serialize")
    }

    pub fn verify<V: ManifestSignatureVerifier>(&self, verifier: &V) -> ManifestStatus {
        if self.artifacts.is_empty() {
            return ManifestStatus::Empty;
        }

        if verifier.verify(&self.signing_payload(), &self.signature) {
            ManifestStatus::Trusted
        } else {
            ManifestStatus::InvalidSignature
        }
    }
}

impl ManifestKind {
    pub fn label(&self) -> &'static str {
        match self {
            ManifestKind::Tool => "tool",
            ManifestKind::Model => "model",
            ManifestKind::Runtime => "runtime",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AlwaysTrusted;

    impl ManifestSignatureVerifier for AlwaysTrusted {
        fn verify(&self, _payload: &str, _signature: &str) -> bool {
            true
        }
    }

    #[test]
    fn verifies_non_empty_manifest() {
        let manifest = SignedManifest {
            kind: ManifestKind::Model,
            artifacts: vec![ManifestArtifact {
                name: "local-model".into(),
                version: "0.1.0".into(),
                path: "models/llm/model.gguf".into(),
                sha256: "abc".into(),
            }],
            signature: "sig".into(),
        };

        assert_eq!(manifest.verify(&AlwaysTrusted), ManifestStatus::Trusted);
    }

    #[test]
    fn rejects_empty_manifest() {
        let manifest = SignedManifest {
            kind: ManifestKind::Tool,
            artifacts: vec![],
            signature: "sig".into(),
        };

        assert_eq!(manifest.verify(&AlwaysTrusted), ManifestStatus::Empty);
    }

    #[test]
    fn signing_payload_uses_structured_json() {
        let manifest = SignedManifest {
            kind: ManifestKind::Runtime,
            artifacts: vec![ManifestArtifact {
                name: "legit-model\nmalicious-model".into(),
                version: "0.1.0|2.0.0".into(),
                path: "models/llm/model.gguf".into(),
                sha256: "abc".into(),
            }],
            signature: "sig".into(),
        };

        let payload = manifest.signing_payload();

        assert!(payload.starts_with("{\"kind\":\"runtime\",\"artifacts\":["));
        assert!(payload.contains("\\n"));
        assert!(!payload.contains("legit-model\nmalicious-model"));
        assert!(!payload.contains("runtime\n"));
    }
}
