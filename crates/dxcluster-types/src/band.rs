use crate::frequency::FrequencyHz;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Band {
    Meter160,
    Meter80,
    Meter40,
    Meter20,
    Meter15,
    Meter10,
}

impl Band {
    pub fn from_frequency(freq: FrequencyHz) -> Option<Self> {
        let hz = freq.0;
        match hz {
            1800000..=2000000 => Some(Band::Meter160),
            3500000..=4000000 => Some(Band::Meter80),
            7000000..=7300000 => Some(Band::Meter40),
            14000000..=14350000 => Some(Band::Meter20),
            21000000..=21450000 => Some(Band::Meter15),
            28000000..=29700000 => Some(Band::Meter10),
            _ => None,
        }
    }
}
