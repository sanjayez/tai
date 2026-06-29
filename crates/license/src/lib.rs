use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LicenseFile {
    pub customer_id: String,
    pub product: String,
    pub expires_on: String,
    pub signature: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LicenseStatus {
    Unchecked,
    Valid,
    Expired,
    InvalidDate,
    InvalidSignature,
    WrongProduct,
}

pub trait SignatureVerifier {
    fn verify(&self, payload: &str, signature: &str) -> bool;
}

pub struct LicenseVerifier<V> {
    product: String,
    signature_verifier: V,
}

impl<V: SignatureVerifier> LicenseVerifier<V> {
    pub fn new(product: impl Into<String>, signature_verifier: V) -> Self {
        Self {
            product: product.into(),
            signature_verifier,
        }
    }

    pub fn verify(&self, license: &LicenseFile, today: &str) -> LicenseStatus {
        if license.product != self.product {
            return LicenseStatus::WrongProduct;
        }

        if !self
            .signature_verifier
            .verify(&license.signing_payload(), &license.signature)
        {
            return LicenseStatus::InvalidSignature;
        }

        let Some(expires_on) = IsoDate::parse(&license.expires_on) else {
            return LicenseStatus::InvalidDate;
        };
        let Some(today) = IsoDate::parse(today) else {
            return LicenseStatus::InvalidDate;
        };

        if expires_on < today {
            return LicenseStatus::Expired;
        }

        LicenseStatus::Valid
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct IsoDate {
    year: u16,
    month: u8,
    day: u8,
}

impl IsoDate {
    fn parse(input: &str) -> Option<Self> {
        if input.len() != 10 {
            return None;
        }

        let bytes = input.as_bytes();
        if bytes[4] != b'-' || bytes[7] != b'-' {
            return None;
        }

        if !bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| index == 4 || index == 7 || byte.is_ascii_digit())
        {
            return None;
        }

        let year = input[0..4].parse::<u16>().ok()?;
        let month = input[5..7].parse::<u8>().ok()?;
        let day = input[8..10].parse::<u8>().ok()?;

        if !(1..=12).contains(&month) {
            return None;
        }

        let max_day = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 if is_leap_year(year) => 29,
            2 => 28,
            _ => return None,
        };

        if day == 0 || day > max_day {
            return None;
        }

        Some(Self { year, month, day })
    }
}

fn is_leap_year(year: u16) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

impl LicenseFile {
    pub fn signing_payload(&self) -> String {
        #[derive(Serialize)]
        struct SigningPayload<'a> {
            customer_id: &'a str,
            product: &'a str,
            expires_on: &'a str,
        }

        let payload = SigningPayload {
            customer_id: &self.customer_id,
            product: &self.product,
            expires_on: &self.expires_on,
        };

        serde_json::to_string(&payload).expect("license signing payload should serialize")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AlwaysValid;

    impl SignatureVerifier for AlwaysValid {
        fn verify(&self, _payload: &str, _signature: &str) -> bool {
            true
        }
    }

    #[test]
    fn marks_license_valid_when_signature_and_expiry_pass() {
        let license = LicenseFile {
            customer_id: "cust_1".into(),
            product: "tally-ai-companion".into(),
            expires_on: "2026-12-31".into(),
            signature: "sig".into(),
        };

        let verifier = LicenseVerifier::new("tally-ai-companion", AlwaysValid);

        assert_eq!(
            verifier.verify(&license, "2026-06-29"),
            LicenseStatus::Valid
        );
    }

    #[test]
    fn marks_license_expired() {
        let license = LicenseFile {
            customer_id: "cust_1".into(),
            product: "tally-ai-companion".into(),
            expires_on: "2026-01-01".into(),
            signature: "sig".into(),
        };

        let verifier = LicenseVerifier::new("tally-ai-companion", AlwaysValid);

        assert_eq!(
            verifier.verify(&license, "2026-06-29"),
            LicenseStatus::Expired
        );
    }

    #[test]
    fn signing_payload_uses_structured_json() {
        let license = LicenseFile {
            customer_id: "cust|evil-product".into(),
            product: "real\nproduct".into(),
            expires_on: "2026-12-31".into(),
            signature: "sig".into(),
        };

        let payload = license.signing_payload();

        assert!(payload.starts_with("{\"customer_id\":\"cust|evil-product\""));
        assert!(payload.contains("\\n"));
        assert!(!payload.contains("real\nproduct"));
    }

    #[test]
    fn rejects_invalid_expiry_dates() {
        let license = LicenseFile {
            customer_id: "cust_1".into(),
            product: "tally-ai-companion".into(),
            expires_on: "2026-6-29".into(),
            signature: "sig".into(),
        };

        let verifier = LicenseVerifier::new("tally-ai-companion", AlwaysValid);

        assert_eq!(
            verifier.verify(&license, "2026-06-29"),
            LicenseStatus::InvalidDate
        );
    }

    #[test]
    fn rejects_invalid_today_dates() {
        let license = LicenseFile {
            customer_id: "cust_1".into(),
            product: "tally-ai-companion".into(),
            expires_on: "2026-12-31".into(),
            signature: "sig".into(),
        };

        let verifier = LicenseVerifier::new("tally-ai-companion", AlwaysValid);

        assert_eq!(
            verifier.verify(&license, ""),
            LicenseStatus::InvalidDate
        );
    }
}
