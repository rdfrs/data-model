use crate::terms::NamedNode;
use chrono::{DateTime, Utc};
use std::fmt::{Display, Formatter};

// XSD Primitive Types that we're going to support
// -----------------------------------------------
// integer - usize since it has some flexibility with processor architecture
// boolean
// dateTime
// double   - 64 bit - chosen because f64 is the default floating-point type in rust
// string
#[derive(PartialEq, Debug)]
pub enum Literal {
    String(String, Option<String>),
    Int(usize),
    Boolean(bool),
    Float(f64),
    DateTime(DateTime<Utc>),
}

// TODO: look at implementing the from trait to make it easier
impl Literal {
    fn data_type(&self) -> NamedNode {
        match &self {
            Literal::String(_, _) => {
                NamedNode::new("http://www.w3.org/2001/XMLSchema#string").unwrap()
            }
            Literal::Int(_) => NamedNode::new("http://www.w3.org/2001/XMLSchema#integer").unwrap(),
            Literal::Boolean(_) => {
                NamedNode::new("http://www.w3.org/2001/XMLSchema#boolean").unwrap()
            }
            Literal::Float(_) => NamedNode::new("http://www.w3.org/2001/XMLSchema#double").unwrap(),
            Literal::DateTime(_) => {
                NamedNode::new("http://www.w3.org/2001/XMLSchema#dateTime").unwrap()
            }
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let xsd_type = &self.data_type();
        match &self {
            Literal::String(value, lang) => match lang {
                Some(l) => write!(f, "\"{}\"@{}", value, l),
                None => write!(f, "\"{}\"", value),
            },
            Literal::Int(value) => write!(f, "\"{}\"^^{}", value, xsd_type),
            Literal::Boolean(value) => write!(f, "\"{}\"^^{}", value, xsd_type),
            Literal::Float(value) => write!(f, "\"{}\"^^{}", value, xsd_type),
            Literal::DateTime(value) => write!(f, "\"{}\"^^{}", value, xsd_type),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        let n1 = Literal::String("foo".to_string(), None);
        let n2 = Literal::String("foo".to_string(), None);
        let n3 = Literal::String("bar".to_string(), None);
        let n4 = Literal::String("bar".to_string(), Some("es".to_string()));
        let n5 = Literal::Int(42);
        let n6 = Literal::Int(42);

        assert_eq!(n1, n2);
        assert_ne!(n1, n3);
        assert_ne!(n3, n4);
        assert_ne!(n1, n5);
        assert_eq!(n5, n6);
    }

    #[test]
    fn display_lang_string() {
        let l = Literal::String("Hola Mundo".to_string(), Some("es".to_string()));

        let expected = "\"Hola Mundo\"@es".to_string();
        let actual = format!("{l}");

        assert_eq!(expected, actual);
    }

    #[test]
    fn display_string() {
        let l = Literal::String("Hello World".to_string(), None);

        let expected = "\"Hello World\"".to_string();
        let actual = format!("{l}");

        assert_eq!(expected, actual);
    }

    #[test]
    fn display_non_string() {
        let l = Literal::Int(42);

        let expected = "\"42\"^^<http://www.w3.org/2001/XMLSchema#integer>".to_string();
        let actual = format!("{l}");

        assert_eq!(expected, actual);
    }
}
