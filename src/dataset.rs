// use crate::{quad::Quad, error::Error};

// trait Dataset: Sized {
//     // design thought: take ownership of the quad and return a shared reference to it
//     fn push(&mut self, quad: Quad) -> Result<&Quad, Error>;
//     fn length(&self) -> usize;
//     // fn delete(&self, quad: &Quad) -> Result<Error, Self>;
//     // fn has(&self, quad: &Quad) -> bool;
//     // fn match_term(
//     //     &self,
//     //     subject: Option<Subject>,
//     //     predicate: Option<NamedNode>,
//     //     object: Option<Object>,
//     //     graph: Option<NamedNode>,
//     // );
//     // fn add_all(&self, dataset: &Self) -> Result<Error, Self>;
//     // fn contains(&self, other: &Self) -> bool;
//     // deleteMatches(same as match_term)
//     // fn difference(&self, other: &Self) -> Result<Error, Self>;
//     // boolean equals (Dataset other);
//     // boolean every (QuadFilterIteratee iteratee);
//     // Dataset filter (QuadFilterIteratee iteratee);
//     // void forEach (QuadRunIteratee iteratee);
//     // Promise<Dataset> import (Stream stream);
//     // Dataset intersection (Dataset other);
//     // Dataset map (QuadMapIteratee iteratee);
//     // any reduce (QuadReduceIteratee iteratee, optional any initialValue);
//     // boolean some (QuadFilterIteratee iteratee);
//     // String toCanonical ();
//     // Stream toStream ();
//     // String toString ();
//     // Dataset union (Dataset quads);
// }

// // what collection shall I use for the dataset itself?
// // struct QuadIndexedDataset {
// //     quads:
// // }

// pub struct NaiveDataset {
//     quads: Vec<Quad>,
// }

// impl NaiveDataset {
//     pub fn new() -> NaiveDataset {
//         NaiveDataset { quads: Vec::new() }
//     }
// }

// // TODO: read chapter on traits
// // TODO: read chapter on iteration and collections
// // TODO: read chapter on functions and/or closures
// impl Dataset for NaiveDataset {
//     fn push(&mut self, quad: Quad) -> Result<&Quad, Err> {
//         if self.quads.contains(&quad) == false {
//             self.quads.push(quad);
//             self.quads.last().unwrap()
//         } else {
//             // given that this naive structure is really a vanilla set,
//             // order isn't really something we can count on
//             for q in &(self.quads) {
//                 if q == &quad {
//                     q
//                 } else {
//                     panic!("should not get here")
//                 }
//             }
//         }
//     }

//     fn length(&self) -> usize {
//         self.quads.len()
//     }
// }

// #[cfg(test)]
// mod test_naive_dataset {
//     use crate::dataset::{Dataset, NaiveDataset};
//     use crate::quad::*;
//     use crate::terms::*;

//     #[test]
//     fn push() {
//         let mut ds = NaiveDataset::new();

//         let q = Quad {
//             subject: Subject::BlankNode(BlankNode::new("b1").unwrap()),
//             predicate: NamedNode::new("https://foo.bar/baz").unwrap(),
//             object: Object::Literal(Literal::Int(42)),
//             graph: None,
//         };

//         let expected = q.clone();

//         let actual = ds.push(q);

//         assert_eq!(expected, actual)
//     }

//     #[test]
//     fn add_quad_does_not_allow_duplicates() {
//         let mut ds = NaiveDataset::new();
//         ds.push(quad::Quad {
//             subject: quad::Subject::BlankNode(terms::BlankNode::new("b1").unwrap()),
//             predicate: terms::NamedNode::new("https://foo.bar/baz").unwrap(),
//             object: quad::Object::Literal(terms::Literal::Int(42)),
//             graph: None,
//         });

//         ds.push(quad::Quad {
//             subject: quad::Subject::BlankNode(terms::BlankNode::new("b1").unwrap()),
//             predicate: terms::NamedNode::new("https://foo.bar/baz").unwrap(),
//             object: quad::Object::Literal(terms::Literal::Int(42)),
//             graph: None,
//         });

//         assert_eq!(1, ds.length())
//     }
// }
