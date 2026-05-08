use phonenumber::{Mode, PhoneNumber as BasePhoneNumber, parse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum CountryCode {
    NG, // Nigeria (primary)
    GH, // Ghana
    SN, // Senegal
    KE, // Kenya
    ZA, // South Africa
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhoneNumber {
    pub e164: String, // +2348012345678
    pub country_code: CountryCode,
    pub national_number: String,
}

#[derive(Debug, thiserror::Error)]
pub enum PhoneError {
    #[error("Invalid phone number format")]
    InvalidFormat,

    #[error("Invalid phone number")]
    InvalidNumber,
}

impl CountryCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            CountryCode::NG => "NG",
            CountryCode::GH => "GH",
            CountryCode::SN => "SN",
            CountryCode::KE => "KE",
            CountryCode::ZA => "ZA",
        }
    }

    pub fn default() -> Self {
        CountryCode::NG
    }
}

impl PhoneNumber {
    pub fn normalize(raw: &str, code: Option<CountryCode>) -> Result<Self, PhoneError> {
        let country_code = code.unwrap_or(CountryCode::NG);

        let parsed: BasePhoneNumber = parse(Some(country_code.as_str().parse().unwrap()), raw)
            .or_else(|_| parse(None, raw))
            .map_err(|_| PhoneError::InvalidFormat)?;

        if !phonenumber::is_valid(&parsed) {
            return Err(PhoneError::InvalidNumber);
        }

        let e164 = parsed.format().mode(Mode::E164).to_string();
        let national_number = parsed.format().mode(Mode::National).to_string();

        Ok(PhoneNumber {
            e164,
            country_code,
            national_number,
        })
    }

    pub fn is_nigerian(&self) -> bool {
        self.country_code == CountryCode::NG
    }
}
