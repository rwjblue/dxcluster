use crate::error::CallsignError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Callsign(String);

impl Callsign {
    pub fn new(raw: String) -> Result<Self, CallsignError> {
        Callsign::validate(&raw, false)?;
        Ok(Callsign(raw.to_uppercase()))
    }

    pub fn parse_loose(input: &str) -> Result<Self, CallsignError> {
        let trimmed = input.trim();
        Callsign::validate(trimmed, false)?;
        Ok(Callsign(trimmed.to_uppercase()))
    }

    pub fn parse_strict(input: &str) -> Result<Self, CallsignError> {
        let trimmed = input.trim();
        Callsign::validate(trimmed, true)?;
        Ok(Callsign(trimmed.to_uppercase()))
    }

    fn validate(value: &str, strict: bool) -> Result<(), CallsignError> {
        if value.is_empty() {
            return Err(CallsignError::Empty);
        }

        if strict || cfg!(feature = "strict_callsign") {
            let has_digit = value.chars().any(|c| c.is_ascii_digit());
            let all_valid = value.chars().all(|c| c.is_ascii_alphanumeric() || c == '/');
            if !has_digit || !all_valid {
                return Err(CallsignError::InvalidFormat);
            }
        }

        Ok(())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
