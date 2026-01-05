use std::fmt;

use crate::frequency::FrequencyHz;

/// Amateur radio bands recognized by `dxcluster-types`.
///
/// The ranges and display labels for each band are centralized in
/// [`Band::definitions`], which is used for both formatting and frequency
/// classification.
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BandDefinition {
    pub band: Band,
    pub label: &'static str,
    pub low_hz: u64,
    pub high_hz: u64,
}

impl BandDefinition {
    pub const fn contains(&self, hz: u64) -> bool {
        hz >= self.low_hz && hz <= self.high_hz
    }
}

const BAND_DEFINITIONS: [BandDefinition; 14] = [
    BandDefinition {
        band: Band::Meter160,
        label: "160m",
        low_hz: 1_800_000,
        high_hz: 2_000_000,
    },
    BandDefinition {
        band: Band::Meter80,
        label: "80m",
        low_hz: 3_500_000,
        high_hz: 4_000_000,
    },
    BandDefinition {
        band: Band::Meter60,
        label: "60m",
        low_hz: 5_351_500,
        high_hz: 5_366_500,
    },
    BandDefinition {
        band: Band::Meter40,
        label: "40m",
        low_hz: 7_000_000,
        high_hz: 7_300_000,
    },
    BandDefinition {
        band: Band::Meter30,
        label: "30m",
        low_hz: 10_100_000,
        high_hz: 10_150_000,
    },
    BandDefinition {
        band: Band::Meter20,
        label: "20m",
        low_hz: 14_000_000,
        high_hz: 14_350_000,
    },
    BandDefinition {
        band: Band::Meter17,
        label: "17m",
        low_hz: 18_068_000,
        high_hz: 18_168_000,
    },
    BandDefinition {
        band: Band::Meter15,
        label: "15m",
        low_hz: 21_000_000,
        high_hz: 21_450_000,
    },
    BandDefinition {
        band: Band::Meter12,
        label: "12m",
        low_hz: 24_890_000,
        high_hz: 24_990_000,
    },
    BandDefinition {
        band: Band::Meter10,
        label: "10m",
        low_hz: 28_000_000,
        high_hz: 29_700_000,
    },
    BandDefinition {
        band: Band::Meter6,
        label: "6m",
        low_hz: 50_000_000,
        high_hz: 54_000_000,
    },
    BandDefinition {
        band: Band::Meter2,
        label: "2m",
        low_hz: 144_000_000,
        high_hz: 148_000_000,
    },
    BandDefinition {
        band: Band::Meter1_25,
        label: "1.25m",
        low_hz: 222_000_000,
        high_hz: 225_000_000,
    },
    BandDefinition {
        band: Band::Centimeter70,
        label: "70cm",
        low_hz: 420_000_000,
        high_hz: 450_000_000,
    },
];

impl Band {
    pub const fn definitions() -> &'static [BandDefinition] {
        &BAND_DEFINITIONS
    }

    pub fn from_frequency(freq: FrequencyHz) -> Option<Self> {
        let hz = freq.0;
        Band::definitions()
            .iter()
            .find_map(|definition| definition.contains(hz).then_some(definition.band))
    }

    pub fn label(&self) -> &'static str {
        Band::definitions()
            .iter()
            .find(|definition| definition.band == *self)
            .map(|definition| definition.label)
            .expect("all Band variants have a definition")
    }
}

impl fmt::Display for Band {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frequency::FrequencyHz;

    #[test]
    fn maps_frequency_to_band() {
        for definition in Band::definitions() {
            let center = FrequencyHz((definition.low_hz + definition.high_hz) / 2);
            assert_eq!(
                Band::from_frequency(center),
                Some(definition.band),
                "freq {} should map to {:?}",
                center,
                definition.band
            );
        }
    }

    #[test]
    fn displays_label() {
        for definition in Band::definitions() {
            assert_eq!(definition.band.to_string(), definition.label);
        }
    }

    #[test]
    fn covers_band_edges() {
        for definition in Band::definitions() {
            let low = FrequencyHz(definition.low_hz);
            let high = FrequencyHz(definition.high_hz);

            assert_eq!(Band::from_frequency(low), Some(definition.band));
            assert_eq!(Band::from_frequency(high), Some(definition.band));

            if definition.low_hz > 0 {
                let below = FrequencyHz(definition.low_hz - 1);
                assert!(Band::from_frequency(below).is_none());
            }

            let above = FrequencyHz(definition.high_hz + 1);
            assert!(Band::from_frequency(above).is_none());
        }
    }
}
