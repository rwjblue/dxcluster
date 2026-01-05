use crate::error::PolicyReject;
use crate::spot::Spot;

#[derive(Debug, Clone, Default)]
pub struct Policy;

impl Policy {
    pub fn accept(&self, _spot: &Spot) -> Result<(), PolicyReject> {
        Ok(())
    }
}
