use crate::object::Object;
use crate::quad::Quad;
use crate::subject::Subject;
use crate::NamedNode;
use std::collections::HashSet;

#[derive(Default, Debug, PartialEq)]
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

    pub fn insert(&mut self, q: Quad) {
        self.quads.insert(q.into());
    }

    pub fn delete(&mut self, q: &Quad) {
        self.quads.remove(q);
    }

    pub fn has(&self, q: &Quad) -> bool {
        self.quads.contains(q)
    }

    pub fn match_term<S, P, O, G>(
        &self,
        subject: Option<S>,
        predicate: Option<P>,
        object: Option<O>,
        graph: Option<G>,
    ) -> &Dataset
    where
        S: Into<Subject>,
        P: Into<NamedNode>,
        O: Into<Object>,
        G: Into<NamedNode>,
    {
        todo!()
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
            BlankNode::from("b0"),
            NamedNode::try_from("https://schema.org/bar")?,
            Literal::from(1),
            None,
        )?);

        Ok(ds)
    }

    #[test]
    fn sample_dataset_length() -> GenericResult<()> {
        assert_eq!(5, create_sample_dataset()?.len());
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

        assert_eq!(4, ds.len());

        Ok(())
    }
}
