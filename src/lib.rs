use url::Url;

pub type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type GenericResult<T> = Result<T, GenericError>;

#[derive(Default)]
pub struct Dataset {
    quads: Vec<Quad>,
}

impl Dataset {
    pub fn len(&self) -> usize {
        self.quads.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push<Q: Into<Quad>>(&mut self, q: Q) {
        self.quads.push(q.into())
    }
}

#[derive(Debug, PartialEq)]
pub struct Quad {
    subject: Subject,
    predicate: NamedNode,
    object: Object,
    graph: Option<NamedNode>,
}

impl Quad {
    pub fn new<S, O>(
        subject: S,
        predicate: NamedNode,
        object: O,
        graph: Option<NamedNode>,
    ) -> GenericResult<Self>
    where
        S: Into<Subject>,
        O: Into<Object>,
    {
        let q = Quad {
            subject: subject.into(),
            predicate,
            object: object.into(),
            graph,
        };
        Ok(q)
    }
}

#[derive(Debug, PartialEq)]
pub enum Subject {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
}

impl From<NamedNode> for Subject {
    fn from(value: NamedNode) -> Self {
        Subject::NamedNode(value)
    }
}

impl From<BlankNode> for Subject {
    fn from(value: BlankNode) -> Self {
        Subject::BlankNode(value)
    }
}

#[derive(Debug, PartialEq)]
pub enum Object {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
    Literal(Literal),
}

impl From<NamedNode> for Object {
    fn from(value: NamedNode) -> Self {
        Object::NamedNode(value)
    }
}

impl From<BlankNode> for Object {
    fn from(value: BlankNode) -> Self {
        Object::BlankNode(value)
    }
}

impl From<Literal> for Object {
    fn from(value: Literal) -> Self {
        Object::Literal(value)
    }
}

#[derive(Debug, PartialEq)]
pub struct NamedNode {
    value: String,
}

impl From<Url> for NamedNode {
    fn from(value: Url) -> Self {
        NamedNode {
            value: value.to_string(),
        }
    }
}

impl TryFrom<&str> for NamedNode {
    type Error = GenericError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parsed = Url::parse(value)?;
        Ok(NamedNode {
            value: parsed.to_string(),
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct BlankNode {}

const XSD_STRING: &str = "http://www.w3.org/2001/XMLSchema#string";
const XSD_INTEGER: &str = "http://www.w3.org/2001/XMLSchema#integer";
const XSD_BOOLEAN: &str = "http://www.w3.org/2001/XMLSchema#boolean";

#[derive(Debug, PartialEq)]
pub struct Literal {
    value: String,
    datatype: NamedNode,
    language: Option<String>,
}

impl From<&str> for Literal {
    fn from(value: &str) -> Self {
        Literal {
            value: value.to_string(),
            datatype: NamedNode {
                value: XSD_STRING.to_string(),
            },
            language: None,
        }
    }
}

impl From<i32> for Literal {
    fn from(value: i32) -> Self {
        Literal {
            value: value.to_string(),
            datatype: NamedNode {
                value: XSD_INTEGER.to_string(),
            },
            language: None,
        }
    }
}

impl From<bool> for Literal {
    fn from(value: bool) -> Self {
        Literal {
            value: value.to_string(),
            datatype: NamedNode {
                value: XSD_BOOLEAN.to_string(),
            },
            language: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use chrono::DateTime;

    #[test]
    fn named_node_from_str() -> GenericResult<()> {
        let n: NamedNode = NamedNode::try_from("http://acme.org")?;
        let expected = "http://acme.org/".to_string();
        assert_eq!(expected, n.value);
        Ok(())
    }

    #[test]
    fn subect_from_named_node() -> GenericResult<()> {
        let n = NamedNode {
            value: "http://acme.org/".to_string(),
        };

        let s: Subject = n.into();

        let expected = Subject::NamedNode(NamedNode {
            value: "http://acme.org/".to_string(),
        });

        assert_eq!(expected, s);
        Ok(())
    }

    #[test]
    fn quad_new() -> GenericResult<()> {
        let expected = Quad {
            subject: Subject::NamedNode(NamedNode {
                value: "http://acme.org/".to_string(),
            }),
            predicate: NamedNode {
                value: "https://schema.org/name".to_string(),
            },
            object: Object::NamedNode(NamedNode {
                value: "http://bar.com/".to_string(),
            }),
            graph: None,
        };

        let actual = Quad::new(
            NamedNode::try_from("http://acme.org")?,
            NamedNode::try_from("https://schema.org/name")?,
            NamedNode::try_from("http://bar.com")?,
            None,
        )?;

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn create_literals_from_native_types() -> GenericResult<()> {
        let string_literal = Literal::from("howard");
        let int_literal = Literal::from(42);
        let bool_literal = Literal::from(true);
        // let date_literal = Literal::from(DateTime::default());

        let expected_string_literal = Literal {
            value: "howard".to_string(),
            datatype: NamedNode {
                value: "http://www.w3.org/2001/XMLSchema#string".to_string(),
            },
            language: None,
        };

        let expected_int_literal = Literal {
            value: "42".to_string(),
            datatype: NamedNode {
                value: "http://www.w3.org/2001/XMLSchema#integer".to_string(),
            },
            language: None,
        };

        let expected_bool_literal = Literal {
            value: "true".to_string(),
            datatype: NamedNode {
                value: "http://www.w3.org/2001/XMLSchema#boolean".to_string(),
            },
            language: None,
        };

        assert_eq!(expected_string_literal, string_literal);
        assert_eq!(expected_int_literal, int_literal);
        assert_eq!(expected_bool_literal, bool_literal);

        Ok(())
    }
}
