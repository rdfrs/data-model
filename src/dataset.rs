use crate::error::Error;
use crate::quad::{Object, Quad, Subject};
use crate::terms::NamedNode;

trait Dataset: Sized {
    // design thought: take ownership of the quad and return a shared reference to it
    fn add(&mut self, quad: Quad) -> &Quad;
    fn size(&self) -> usize;
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

// what collection shall I use for the dataset itself?
// struct QuadIndexedDataset {
//     quads:
// }

pub struct NaiveDataset {
    quads: Vec<Quad>,
}

impl NaiveDataset {
    pub fn new() -> NaiveDataset {
        NaiveDataset { quads: Vec::new() }
    }
}

// TODO: read chapter on traits
// TODO: read chapter on iteration and collections
// TODO: read chapter on functions and/or closures
impl Dataset for NaiveDataset {
    fn add(&mut self, quad: Quad) -> &Quad {
        self.quads.push(quad);
        self.quads.last().unwrap()
    }

    fn size(&self) -> usize {
        self.quads.len()
    }
}

#[cfg(test)]
mod test {
    use crate::dataset::{Dataset, NaiveDataset};
    use crate::quad;
    use crate::terms;

    #[test]
    fn add_quad() {
        let mut ds = NaiveDataset::new();
        let q = ds.add(quad::Quad {
            subject: quad::Subject::BlankNode(terms::BlankNode::new("b1").unwrap()),
            predicate: terms::NamedNode::new("https://foo.bar/baz").unwrap(),
            object: quad::Object::Literal(terms::Literal::Int(42)),
            graph: None,
        });

        assert_eq!(1, ds.size())
    }
}
