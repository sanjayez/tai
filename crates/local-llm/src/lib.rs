use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LlmRuntimeConfig {
    pub executable_path: String,
    pub model_path: String,
    pub context_tokens: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LlmRequest {
    pub system_prompt: String,
    pub user_prompt: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LlmResponse {
    pub text: String,
}

pub trait LocalLlm {
    fn generate(&self, request: &LlmRequest) -> Result<LlmResponse, LlmError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LlmError {
    RuntimeUnavailable,
    ModelUnavailable,
    GenerationFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    struct EchoLlm;

    impl LocalLlm for EchoLlm {
        fn generate(&self, request: &LlmRequest) -> Result<LlmResponse, LlmError> {
            Ok(LlmResponse {
                text: request.user_prompt.clone(),
            })
        }
    }

    #[test]
    fn local_llm_trait_supports_generation() {
        let response = EchoLlm
            .generate(&LlmRequest {
                system_prompt: "route tools".into(),
                user_prompt: "list ledgers".into(),
            })
            .unwrap();

        assert_eq!(response.text, "list ledgers");
    }
}

