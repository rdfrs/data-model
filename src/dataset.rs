use crate::object::Object;
use crate::quad::Quad;
use crate::subject::Subject;
use crate::NamedNode;
use std::collections::{HashMap, HashSet};
// use std::iter::from_fn;

#[derive(Default, Debug, PartialEq)]
pub struct Dataset {
    quads: HashSet<Quad>,
    subject_index: HashMap<Subject, Box<Quad>>,
    predicate_index: HashMap<NamedNode, Box<Quad>>,
    object_index: HashMap<Object, Box<Quad>>,
    graph_index: HashMap<NamedNode, Box<Quad>>, // only create entry if there's a named graph
}

impl Dataset {
    pub fn len(&self) -> usize {
        self.quads.len()
    }

    pub fn is_empty(&self) -> bool {
        self.quads.is_empty()
    }

    pub fn insert(&mut self, q: Quad) {
        self.quads.insert(q);
    }

    pub fn delete(&mut self, q: &Quad) {
        self.quads.remove(q);
    }

    pub fn has(&self, q: &Quad) -> bool {
        self.quads.contains(q)
    }

    // 2 solution options to ensure I understand
    // 1) pass the match terms as references and understand how the lifetime annotations should work
    // 2) move the match terms into the iterator so that their lifetimes are bound by the closure
    //
    // I think the problem here may be related to the fact that our type parameters may inherently
    // be borrows because they are trait specifications. Let's try getting to more concrete types
    // outside the closure.... NOPE
    pub fn match_term(
        &self,
        subject: Option<Subject>,
        predicate: Option<NamedNode>,
        object: Option<Object>,
        graph: Option<NamedNode>,
    ) -> impl Iterator<Item = &Quad> {
        // implement the naive approach first - O(n) and then apply indexing
        // to implement a more custom iterator (using index), leverage std::iter::from_fn

        self.quads.iter().filter(move |q| -> bool {
            let subject_equals = subject.as_ref().is_some_and(|v| v == &q.subject);
            let predicate_equals = predicate.as_ref().is_some_and(|v| v == &q.predicate);
            let object_equals = object.as_ref().is_some_and(|v| v == &q.object);
            let graph_equals = graph.as_ref().is_some_and(|v| match &q.graph {
                None => false,
                Some(qv) => v == qv,
            });
            subject_equals || predicate_equals || object_equals || graph_equals
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BlankNode, GenericResult, Literal, NamedNode, Quad};

    fn create_sample_dataset() -> GenericResult<Dataset> {
        // <https://acme.com/JaneDoe> <https://schema.org/age> "23"^^<http://www.w3.org/2001/XMLSchema#integer> .
        // <https://acme.com/JaneDoe> <https://schema.org/foo> _:b0 .
        // <https://acme.com/JaneDoe> <https://schema.org/is_alive> "true"^^<http://www.w3.org/2001/XMLSchema#boolean> .
        // <https://acme.com/JaneDoe> <https://schema.org/jobTitle> "Professor" .
        // <https://acme.com/JaneDoe> <https://schema.org/modified> "Mon Jul 31 22:19:04 CDT 2023" .
        // <https://acme.com/JaneDoe> <https://schema.org/name> "Jane Doe" .
        // <https://acme.com/JaneDoe> <https://schema.org/telephone> "(425) 123-4567" .
        // <https://acme.com/JaneDoe> <https://schema.org/url> <https://www.janedoe.com> .
        // <https://acme.com/JaneDoe> <https://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://schema.org/Person> .
        // _:b0 <https://schema.org/bar> "1"^^<http://www.w3.org/2001/XMLSchema#integer> .

        let mut ds = Dataset::default();

        ds.insert(Quad::new(
            NamedNode::try_from("https://acme.com/JaneDoe")?,
            NamedNode::try_from("https://schema.org/age")?,
            Literal::from(23),
            None,
        )?);

        ds.insert(Quad::new(
            NamedNode::try_from("https://acme.com/JaneDoe")?,
            NamedNode::try_from("https://schema.org/foo")?,
            BlankNode::from("b0"),
            None,
        )?);

        ds.insert(Quad::new(
            NamedNode::try_from("https://acme.com/JaneDoe")?,
            NamedNode::try_from("https://schema.org/is_alive")?,
            Literal::from(true),
            None,
        )?);

        ds.insert(Quad::new(
            NamedNode::try_from("https://acme.com/JaneDoe")?,
            NamedNode::try_from("https://schema.org/jobTitle")?,
            Literal::from("Professor"),
            None,
        )?);

        ds.insert(Quad::new(
            NamedNode::try_from("https://acme.com/JaneDoe")?,
            NamedNode::try_from("https://schema.org/modified")?,
            Literal::from("Mon Jul 31 22:19:04 CDT 2023"),
            None,
        )?);

        ds.insert(Quad::new(
            NamedNode::try_from("https://acme.com/JaneDoe")?,
            NamedNode::try_from("https://schema.org/name")?,
            Literal::from("Jane Doe"),
            None,
        )?);

        ds.insert(Quad::new(
            NamedNode::try_from("https://acme.com/JaneDoe")?,
            NamedNode::try_from("https://schema.org/telephone")?,
            Literal::from("(425) 123-4567"),
            None,
        )?);

        ds.insert(Quad::new(
            NamedNode::try_from("https://acme.com/JaneDoe")?,
            NamedNode::try_from("https://schema.org/url")?,
            NamedNode::try_from("https://www.janedoe.com")?,
            None,
        )?);

        ds.insert(Quad::new(
            NamedNode::try_from("https://acme.com/JaneDoe")?,
            NamedNode::try_from("https://www.w3.org/1999/02/22-rdf-syntax-ns#type")?,
            NamedNode::try_from("https://schema.org/Person")?,
            None,
        )?);

        ds.insert(Quad::new(
            BlankNode::from("b0"),
            NamedNode::try_from("https://schema.org/bar")?,
            Literal::from(1),
            None,
        )?);

        Ok(ds)
    }

    #[test]
    fn sample_dataset_length() -> GenericResult<()> {
        assert_eq!(10, create_sample_dataset()?.len());
        Ok(())
    }

    #[test]
    fn sample_datasets_are_equivalent() -> GenericResult<()> {
        assert_eq!(create_sample_dataset()?, create_sample_dataset()?);
        Ok(())
    }

    #[test]
    fn sample_dataset_has_quad() -> GenericResult<()> {
        let ds = create_sample_dataset()?;
        let q = Quad::new(
            NamedNode::try_from("https://acme.com/JaneDoe")?,
            NamedNode::try_from("https://schema.org/is_alive")?,
            Literal::from(true),
            None,
        )?;
        let expected = true;
        let actual = ds.has(&q);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn sample_dataset_does_not_have_quad() -> GenericResult<()> {
        let ds = create_sample_dataset()?;
        let q = Quad::new(
            NamedNode::try_from("https://acme.com/JohnDoe")?,
            NamedNode::try_from("https://schema.org/is_alive")?,
            Literal::from(true),
            None,
        )?;
        let expected = false;
        let actual = ds.has(&q);

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn delete_from_sample_dataset() -> GenericResult<()> {
        let mut ds = create_sample_dataset()?;
        let q = Quad::new(
            NamedNode::try_from("https://acme.com/JaneDoe")?,
            NamedNode::try_from("https://schema.org/is_alive")?,
            Literal::from(true),
            None,
        )?;

        ds.delete(&q);

        assert_eq!(9, ds.len());

        Ok(())
    }

    #[test]
    fn match_subject() -> GenericResult<()> {
        let ds = create_sample_dataset()?;

        let term = NamedNode::try_from("https://acme.com/JaneDoe")?;

        let expected: usize = 9;
        let actual = ds
            .match_term(Some(Subject::NamedNode(term)), None, None, None)
            .collect::<Vec<_>>()
            .len();

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn match_predicate() -> GenericResult<()> {
        let ds = create_sample_dataset()?;

        let term = NamedNode::try_from("https://schema.org/name")?;

        let expected: usize = 1;
        let actual = ds
            .match_term(None, Some(term), None, None)
            .collect::<Vec<_>>()
            .len();

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn match_object() -> GenericResult<()> {
        let ds = create_sample_dataset()?;

        let term = Literal::from(true);

        let expected: usize = 1;
        let actual = ds
            .match_term(None, None, Some(Object::Literal(term)), None)
            .collect::<Vec<_>>()
            .len();

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn match_subject_and_predicate() -> GenericResult<()> {
        // the current logic is incorrect
        // per https://rdf.js.org/dataset-spec/#quad-matching, "Only quads matching all of the
        // given non-null arguments will be selected"

        let ds = create_sample_dataset()?;

        let subj = NamedNode::try_from("https://acme.com/JaneDoe")?;
        let obj = Literal::from(true);

        let expected: usize = 1;
        let actual = ds
            .match_term(
                Some(Subject::NamedNode(subj)),
                None,
                Some(Object::Literal(obj)),
                None,
            )
            .collect::<Vec<_>>()
            .len();

        assert_eq!(expected, actual);

        todo!()
    }
}
