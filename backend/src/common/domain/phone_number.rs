use phonenumber::{Mode, parse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhoneNumber {
    pub e164: String,
    pub country_code: String,
    pub national_number: String,
}

#[derive(Debug, thiserror::Error)]
pub enum PhoneError {
    #[error("Invalid phone number format")]
    InvalidFormat,

    #[error("Invalid phone number")]
    InvalidNumber,
}

impl PhoneNumber {
    pub fn normalize(raw: &str, country_code: Option<&str>) -> Result<Self, PhoneError> {
        let sanitized_raw = raw.trim();
        if sanitized_raw.is_empty() {
            return Err(PhoneError::InvalidFormat);
        }

        let country_hint = country_code.unwrap_or("NG");
        let parsed_country_hint = country_hint
            .parse()
            .map_err(|_| PhoneError::InvalidFormat)?;

        // Try with country hint first, then without (for numbers with + prefix)
        let parsed = parse(Some(parsed_country_hint), raw)
            .or_else(|_| parse(None, raw))
            .map_err(|_| PhoneError::InvalidFormat)?;

        if !phonenumber::is_valid(&parsed) {
            return Err(PhoneError::InvalidNumber);
        }

        Ok(PhoneNumber {
            e164: parsed.format().mode(Mode::E164).to_string(),
            country_code: country_hint.to_uppercase(),
            national_number: parsed.format().mode(Mode::National).to_string(),
        })
    }
}
