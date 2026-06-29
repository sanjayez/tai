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
        let kind = match &self.kind {
            ManifestKind::Tool => "tool",
            ManifestKind::Model => "model",
            ManifestKind::Runtime => "runtime",
        };

        let artifacts = self
            .artifacts
            .iter()
            .map(|artifact| {
                format!(
                    "{}|{}|{}|{}",
                    artifact.name, artifact.version, artifact.path, artifact.sha256
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        format!("{}\n{}", kind, artifacts)
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
}
