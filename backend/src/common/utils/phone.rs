use phonenumber::{Mode, parse};

/// Normalizes any international phone number to E.164 format.
/// Falls back to Nigerian normalization for local formats without country code.
pub fn normalize_phone(raw: &str, default_country: Option<&str>) -> Option<String> {
    let country = default_country.unwrap_or("NG").parse().ok()?;
   
    let parsed = parse(Some(country), raw).ok()?;
   
    if !phonenumber::is_valid(&parsed) {
        return None;
    }

    Some(parsed.format().mode(Mode::E164).to_string())
}
