use crate::Result;
use crate::terms::*;
use std::fmt::{Display, Formatter};

// design option 1: Generic struct with Trait constraints
// Pros: Works well with the type-checker
// Cons: Generic signature bleeds into types that compose the struct, including
//       collections. This poses a design problem in the case where the collection
//       needs to contain different type configurations

#[derive(PartialEq, Debug)]
pub struct Quad<S: Subject, O: Object> {
    subject: Box<S>,
    predicate: NamedNode,
    object: Box<O>,
    graph: Option<NamedNode>,
}

impl<S: Subject, O: Object> Quad<S, O> {
    pub fn new(s: S, p: NamedNode, o: O, g: Option<NamedNode>) -> Result<Quad<S, O>> {
        Ok(Quad {
            subject: Box::new(s),
            predicate: p,
            object: Box::new(o),
            graph: g,
        })
    }

    pub fn subject(&self) -> &S {
        self.subject.as_ref()
    }

    pub fn predicate(&self) -> &NamedNode {
        &self.predicate
    }

    pub fn object(&self) -> &O {
        self.object.as_ref()
    }

    pub fn graph(&self) -> &Option<NamedNode> {
        &self.graph
    }
}

impl<S: Subject, O: Object> Display for Quad<S, O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} .",
            &self.subject, &self.predicate, &self.object
        )
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn quad_should_have_the_correct_terms() -> Result<()>{
        let q = Quad::new(
            NamedNode::new("http://foo.com/bar")?, 
            NamedNode::new("https://schema.org/name")?, 
            Literal::String("bar".to_string(), None), 
            None
        )?;

        assert_eq!(&NamedNode::new("http://foo.com/bar")?, q.subject());
        assert_eq!(&NamedNode::new("https://schema.org/name")?, q.predicate());
        assert_eq!(&Literal::String("bar".to_string(), None), q.object());
        assert_eq!(&None, q.graph());

        Ok(())
    }

    #[test]
    fn quads_should_be_equal_based_on_terms() -> Result<()>{
        let q1 = Quad::new(
            NamedNode::new("http://foo.com/bar")?, 
            NamedNode::new("https://schema.org/name")?, 
            Literal::String("bar".to_string(), None), 
            None
        )?;

        let q2 = Quad::new(
            NamedNode::new("http://foo.com/bar")?, 
            NamedNode::new("https://schema.org/name")?, 
            Literal::String("bar".to_string(), None), 
            None
        )?;

        assert_eq!(q1, q2);

        Ok(())
    }

    #[test]
    fn display_quad() -> Result<()>{
        let expected = "<http://foo.com/bar> <https://schema.org/name> \"bar\" .".to_string();
        
        let q = Quad::new(
            NamedNode::new("http://foo.com/bar")?, 
            NamedNode::new("https://schema.org/name")?, 
            Literal::String("bar".to_string(), None), 
            None
        )?;

        assert_eq!(expected, format!("{}", q));

        Ok(())
    }
}
