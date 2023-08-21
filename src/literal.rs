use crate::named_node::NamedNode;

const XSD_STRING: &str = "http://www.w3.org/2001/XMLSchema#string";
const XSD_INTEGER: &str = "http://www.w3.org/2001/XMLSchema#integer";
const XSD_BOOLEAN: &str = "http://www.w3.org/2001/XMLSchema#boolean";

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Literal {
    value: String,
    datatype: NamedNode,
    language: Option<String>,
}

impl From<&str> for Literal {
    fn from(value: &str) -> Self {
        Literal {
            value: value.to_string(),
            datatype: NamedNode::try_from(XSD_STRING).unwrap(),
            language: None,
        }
    }
}

impl From<i32> for Literal {
    fn from(value: i32) -> Self {
        Literal {
            value: value.to_string(),
            datatype: NamedNode::try_from(XSD_INTEGER).unwrap(),
            language: None,
        }
    }
}

impl From<bool> for Literal {
    fn from(value: bool) -> Self {
        Literal {
            value: value.to_string(),
            datatype: NamedNode::try_from(XSD_BOOLEAN).unwrap(),
            language: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GenericResult;

    #[test]
    fn create_literals_from_native_types() -> GenericResult<()> {
        let string_literal = Literal::from("howard");
        let int_literal = Literal::from(42);
        let bool_literal = Literal::from(true);
        // let date_literal = Literal::from(DateTime::default());

        let expected_string_literal = Literal {
            value: "howard".to_string(),
            datatype: NamedNode::try_from("http://www.w3.org/2001/XMLSchema#string")?,
            language: None,
        };

        let expected_int_literal = Literal {
            value: "42".to_string(),
            datatype: NamedNode::try_from("http://www.w3.org/2001/XMLSchema#integer")?,
            language: None,
        };

        let expected_bool_literal = Literal {
            value: "true".to_string(),
            datatype: NamedNode::try_from("http://www.w3.org/2001/XMLSchema#boolean")?,
            language: None,
        };

        assert_eq!(expected_string_literal, string_literal);
        assert_eq!(expected_int_literal, int_literal);
        assert_eq!(expected_bool_literal, bool_literal);

        Ok(())
    }
}
