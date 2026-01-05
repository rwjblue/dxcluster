#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Cw,
    Ssb,
    Fm,
    Data,
    Other(String),
}
