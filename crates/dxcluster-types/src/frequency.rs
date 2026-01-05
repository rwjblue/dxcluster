use std::{fmt, str::FromStr};

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

    pub fn to_khz_string(&self) -> String {
        let khz = self.0 / 1000;
        let remainder = self.0 % 1000;
        if remainder == 0 {
            khz.to_string()
        } else {
            format!("{khz}.{remainder:03}")
        }
    }
}

impl fmt::Display for FrequencyHz {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_khz_string())
    }
}

impl FromStr for FrequencyHz {
    type Err = FrequencyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        FrequencyHz::from_khz_str(s)
    }
}
