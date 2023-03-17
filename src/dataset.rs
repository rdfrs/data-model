use crate::quad_enums::*;
use crate::Result;

trait Dataset {
    fn new() -> Self;
    // fn push(&mut self, quad: Quad) -> Result<&Quad, Error>;
    // fn length(&self) -> usize;
    // fn delete(&self, quad: &Quad) -> Result<Error, Self>;
    // fn has(&self, quad: &Quad) -> bool;
    // fn match_term(
    //     &self,
    //     subject: Option<Subject>,
    //     predicate: Option<NamedNode>,
    //     object: Option<Object>,
    //     graph: Option<NamedNode>,
    // );
    // fn add_all(&self, dataset: &Self) -> Result<Error, Self>;
    // fn contains(&self, other: &Self) -> bool;
    // deleteMatches(same as match_term)
    // fn difference(&self, other: &Self) -> Result<Error, Self>;
    // boolean equals (Dataset other);
    // boolean every (QuadFilterIteratee iteratee);
    // Dataset filter (QuadFilterIteratee iteratee);
    // void forEach (QuadRunIteratee iteratee);
    // Promise<Dataset> import (Stream stream);
    // Dataset intersection (Dataset other);
    // Dataset map (QuadMapIteratee iteratee);
    // any reduce (QuadReduceIteratee iteratee, optional any initialValue);
    // boolean some (QuadFilterIteratee iteratee);
    // String toCanonical ();
    // Stream toStream ();
    // String toString ();
    // Dataset union (Dataset quads);
}

pub struct IndexedDataset {
    quads: Vec<Quad>,
}

// impl Dataset for IndexedDataset {
// fn push(&mut self, quad: Quad) -> Result<&Quad, Err> {
//     if self.quads.contains(&quad) == false {
//         self.quads.push(quad);
//         self.quads.last().unwrap()
//     } else {
//         // given that this naive structure is really a vanilla set,
//         // order isn't really something we can count on
//         for q in &(self.quads) {
//             if q == &quad {
//                 q
//             } else {
//                 panic!("should not get here")
//             }
//         }
//     }
// }

// fn length(&self) -> usize {
//     self.quads.len()
// }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_have_length_of_zero() -> Result<()> {
        todo!()
    }
}
