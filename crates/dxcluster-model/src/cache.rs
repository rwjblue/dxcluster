use crate::spot::Spot;

#[derive(Debug)]
pub struct SpotCache {
    capacity: usize,
    items: Vec<Spot>,
}

impl SpotCache {
    pub fn new(capacity: usize) -> Self {
        SpotCache {
            capacity,
            items: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, spot: Spot) {
        if self.items.len() == self.capacity {
            self.items.remove(0);
        }
        self.items.push(spot);
    }

    pub fn recent(&self, n: usize) -> impl Iterator<Item = &Spot> {
        let start = self.items.len().saturating_sub(n);
        self.items[start..].iter().rev()
    }
}
