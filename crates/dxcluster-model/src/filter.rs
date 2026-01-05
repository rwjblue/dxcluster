use crate::spot::Spot;

#[derive(Debug, Clone, Default)]
pub struct Filter;

impl Filter {
    pub fn matches(&self, _spot: &Spot) -> bool {
        true
    }
}
