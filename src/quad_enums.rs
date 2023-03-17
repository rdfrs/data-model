use crate::terms::*;
use crate::Result;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug)]
pub enum SubjectTypes {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
}

#[derive(PartialEq, Debug)]
pub enum ObjectTypes {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
    Literal(Literal),
}

#[derive(PartialEq, Debug)]
pub struct Quad {
    subject: SubjectTypes,
    predicate: NamedNode,
    object: ObjectTypes,
    graph: Option<NamedNode>,
}

impl Quad {
    pub fn new(
        s: SubjectTypes,
        p: NamedNode,
        o: ObjectTypes,
        g: Option<NamedNode>,
    ) -> Result<Self> {
        Ok(Quad {
            subject: s,
            predicate: p,
            object: o,
            graph: g,
        })
    }
}

impl Display for Quad {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        _ = match &self.subject {
            SubjectTypes::NamedNode(term) => write!(f, "{} ", term),
            SubjectTypes::BlankNode(term) => write!(f, "{} ", term)
        };

        write!(f, "{} ", self.predicate);

        _ = match &self.object {
            ObjectTypes::BlankNode(term) => write!(f, "{} .", term),
            ObjectTypes::Literal(term) => write!(f, "{} .", term),
            ObjectTypes::NamedNode(term) => write!(f, "{} .", term)
        };
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quad_should_have_the_correct_terms() -> Result<()> {
        let s = SubjectTypes::NamedNode(NamedNode::new("http://foo.com/bar")?);
        let p = NamedNode::new("https://schema.org/name")?;
        let o = ObjectTypes::Literal(Literal::String("bar".to_string(), None));

        let q = Quad::new(s, p, o, None)?;

        assert_eq!(
            SubjectTypes::NamedNode(NamedNode::new("http://foo.com/bar")?),
            q.subject
        );
        assert_eq!(NamedNode::new("https://schema.org/name")?, q.predicate);
        assert_eq!(
            ObjectTypes::Literal(Literal::String("bar".to_string(), None)),
            q.object
        );
        assert_eq!(None, q.graph);

        Ok(())
    }

    #[test]
    fn quads_should_be_equal_based_on_terms() -> Result<()> {
        let q1 = Quad::new(
            SubjectTypes::NamedNode(NamedNode::new("http://foo.com/bar")?),
            NamedNode::new("https://schema.org/name")?,
            ObjectTypes::Literal(Literal::String("bar".to_string(), None)),
            None,
        );

        let q2 = Quad::new(
            SubjectTypes::NamedNode(NamedNode::new("http://foo.com/bar")?),
            NamedNode::new("https://schema.org/name")?,
            ObjectTypes::Literal(Literal::String("bar".to_string(), None)),
            None,
        );

        assert_eq!(q1, q2);

        Ok(())
    }

    #[test]
    fn display_quad() -> Result<()> {
        let expected = "<http://foo.com/bar> <https://schema.org/name> \"bar\" .".to_string();

        let q = Quad::new(
            SubjectTypes::NamedNode(NamedNode::new("http://foo.com/bar")?),
            NamedNode::new("https://schema.org/name")?,
            ObjectTypes::Literal(Literal::String("bar".to_string(), None)),
            None,
        )?;

        assert_eq!(expected, format!("{}", q));

        Ok(())
    }
}
