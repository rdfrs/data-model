use crate::quad::Quad;
use std::collections::HashSet;

#[derive(Default)]
pub struct Dataset {
    quads: HashSet<Quad>,
}

impl Dataset {
    pub fn len(&self) -> usize {
        self.quads.len()
    }

    pub fn is_empty(&self) -> bool {
        self.quads.is_empty()
    }

    pub fn insert<Q: Into<Quad>>(&mut self, q: Q) {
        self.quads.insert(q.into());
    }
}

#[cfg(test)]
mod tests {}
