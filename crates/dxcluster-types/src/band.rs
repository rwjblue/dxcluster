use std::fmt;

use crate::frequency::FrequencyHz;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Band {
    Meter160,
    Meter80,
    Meter60,
    Meter40,
    Meter30,
    Meter20,
    Meter17,
    Meter15,
    Meter12,
    Meter10,
    Meter6,
    Meter2,
    Meter1_25,
    Centimeter70,
}

impl Band {
    pub fn from_frequency(freq: FrequencyHz) -> Option<Self> {
        let hz = freq.0;
        match hz {
            1800000..=2000000 => Some(Band::Meter160),
            3500000..=4000000 => Some(Band::Meter80),
            5351500..=5366500 => Some(Band::Meter60),
            7000000..=7300000 => Some(Band::Meter40),
            10100000..=10150000 => Some(Band::Meter30),
            14000000..=14350000 => Some(Band::Meter20),
            18068000..=18168000 => Some(Band::Meter17),
            21000000..=21450000 => Some(Band::Meter15),
            24890000..=24990000 => Some(Band::Meter12),
            28000000..=29700000 => Some(Band::Meter10),
            50000000..=54000000 => Some(Band::Meter6),
            144000000..=148000000 => Some(Band::Meter2),
            222000000..=225000000 => Some(Band::Meter1_25),
            420000000..=450000000 => Some(Band::Centimeter70),
            _ => None,
        }
    }
}

impl fmt::Display for Band {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Band::Meter160 => "160m",
            Band::Meter80 => "80m",
            Band::Meter60 => "60m",
            Band::Meter40 => "40m",
            Band::Meter30 => "30m",
            Band::Meter20 => "20m",
            Band::Meter17 => "17m",
            Band::Meter15 => "15m",
            Band::Meter12 => "12m",
            Band::Meter10 => "10m",
            Band::Meter6 => "6m",
            Band::Meter2 => "2m",
            Band::Meter1_25 => "1.25m",
            Band::Centimeter70 => "70cm",
        };
        f.write_str(label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frequency::FrequencyHz;

    #[test]
    fn maps_frequency_to_band() {
        let cases = [
            (FrequencyHz(1_850_000), Band::Meter160),
            (FrequencyHz(3_750_000), Band::Meter80),
            (FrequencyHz(5_361_500), Band::Meter60),
            (FrequencyHz(7_100_000), Band::Meter40),
            (FrequencyHz(10_120_000), Band::Meter30),
            (FrequencyHz(14_250_000), Band::Meter20),
            (FrequencyHz(18_110_000), Band::Meter17),
            (FrequencyHz(21_200_000), Band::Meter15),
            (FrequencyHz(24_930_000), Band::Meter12),
            (FrequencyHz(28_500_000), Band::Meter10),
            (FrequencyHz(50_300_000), Band::Meter6),
            (FrequencyHz(145_000_000), Band::Meter2),
            (FrequencyHz(223_500_000), Band::Meter1_25),
            (FrequencyHz(432_000_000), Band::Centimeter70),
        ];

        for (freq, expected) in cases {
            assert_eq!(Band::from_frequency(freq), Some(expected), "freq {}", freq);
        }
    }

    #[test]
    fn displays_label() {
        let cases = [
            (Band::Meter160, "160m"),
            (Band::Meter80, "80m"),
            (Band::Meter60, "60m"),
            (Band::Meter40, "40m"),
            (Band::Meter30, "30m"),
            (Band::Meter20, "20m"),
            (Band::Meter17, "17m"),
            (Band::Meter15, "15m"),
            (Band::Meter12, "12m"),
            (Band::Meter10, "10m"),
            (Band::Meter6, "6m"),
            (Band::Meter2, "2m"),
            (Band::Meter1_25, "1.25m"),
            (Band::Centimeter70, "70cm"),
        ];

        for (band, expected) in cases {
            assert_eq!(band.to_string(), expected);
        }
    }
}
