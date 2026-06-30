use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OcrInput {
    pub file_path: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OcrTextBlock {
    pub text: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OcrResult {
    pub blocks: Vec<OcrTextBlock>,
}

pub trait LocalOcr {
    fn extract_text(&self, input: &OcrInput) -> Result<OcrResult, OcrError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OcrError {
    RuntimeUnavailable,
    UnsupportedFileType,
    ExtractionFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ocr_result_can_hold_blocks() {
        let result = OcrResult {
            blocks: vec![OcrTextBlock {
                text: "Invoice No. 42".into(),
                confidence: 0.91,
            }],
        };

        assert_eq!(result.blocks[0].text, "Invoice No. 42");
    }
}
