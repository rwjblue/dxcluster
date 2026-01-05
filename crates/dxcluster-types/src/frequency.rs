use crate::error::FrequencyError;
use crate::normalize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FrequencyHz(pub u64);

impl FrequencyHz {
    pub fn from_khz_str(input: &str) -> Result<Self, FrequencyError> {
        let cleaned = normalize::trim_units(input);
        if cleaned.is_empty() {
            return Err(FrequencyError::Missing);
        }

        let value: f64 = cleaned.parse().map_err(|_| FrequencyError::Invalid)?;
        let hz = (value * 1000.0).round();
        if hz.is_sign_negative() {
            return Err(FrequencyError::Invalid);
        }
        Ok(FrequencyHz(hz as u64))
    }
}
