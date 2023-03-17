use crate::Result;
use crate::terms::*;

// design option 2: dynamic trait object
// Pros: Cleanest interface (terse, expressive)
// Cons: While it needs more research, boxing trait objects is potentially less than optimal
//       Requires downcasting from the boxed trait object to get back to the term data members

pub struct Quad {
    subject: Box<dyn Subject>,
    predicate: NamedNode,
    object: Box<dyn Object>,
    graph: Option<NamedNode>,
}

impl Quad {
    pub fn new<S: Subject + 'static, O: Object + 'static>(s: S, p: NamedNode, o: O, g: Option<NamedNode>) -> Result<Quad> {
        Ok(Quad{
            subject: Box::new(s),
            predicate: p,
            object: Box::new(o),
            graph: g
        })
    }
}

impl PartialEq for Quad{
    fn eq(&self, other: &Self) -> bool {
        self.subject.as_ref() == other.subject.as_ref() && self.predicate == other.predicate && self.object == other.object && self.graph == other.graph
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn quad_should_have_the_correct_terms() -> Result<()>{
        let q = Quad::new(
            Box::new(NamedNode::new("http://foo.com/bar")?), 
            NamedNode::new("https://schema.org/name")?, 
            Box::new(Literal::String("bar".to_string(), None)), 
            None
        )?;

        // assert_eq!(&NamedNode::new("http://foo.com/bar")?, q.subject());
        // assert_eq!(&NamedNode::new("https://schema.org/name")?, q.predicate());
        // assert_eq!(&Literal::String("bar".to_string(), None), q.object());
        // assert_eq!(&None, q.graph());

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
            Box::new(NamedNode::new("http://foo.com/bar")?), 
            NamedNode::new("https://schema.org/name")?, 
            Box::new(Literal::String("bar".to_string(), None)), 
            None
        )?;

        assert_eq!(expected, format!("{}", q));

        Ok(())
    }
}
