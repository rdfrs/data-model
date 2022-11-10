use crate::error::Error;
use crate::terms::NamedNode;
use std::string::ToString;

#[derive(PartialEq, Eq, Debug)]
pub struct Literal<T: XSDType> {
    value: T,
    language: String, // Note: this should change to conform to a standard list of language codes
    data_type: NamedNode,
}

impl Literal<T> {
    pub fn new(value: T) -> Result<Self, Error> {
        Ok(Literal {
            value,
            language: "".to_string(),
            data_type: T.xsd_type(),
        })
    }
}

// XSD Primitive Types that we're going to support
// -----------------------------------------------

// integer  - 32 bit - chosen as i32 is the default integer type in rust
// boolean
// date
// dateTime
// double   - 64 bit - chosen because f64 is the default floating-point type in rust
// string
// time

pub trait XSDType {
    fn xsd_type() -> NamedNode;
}

impl XSDType for String {
    fn xsd_type() -> NamedNode {
        NamedNode::new("http://www.w3.org/2001/XMLSchema#integer").unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_for_english_string() {
        let expected = Literal {
            value: "foo",
            language: "EN".to_string(),
            data_type: NamedNode::new("http://www.w3.org/2001/XMLSchema#integer").unwrap(),
        };

        let l1 = Literal::new("foo");
        assert_eq!(expected, l1);
    }

    #[test]
    fn display_for_spanish_string() {
        todo!();
    }

    #[test]
    fn display_for_integer() {
        todo!();
    }

    #[test]
    fn display_for_decimal() {
        todo!();
    }

    #[test]
    fn display_for_duration() {
        todo!();
    }

    #[test]
    fn display_for_datetime() {
        todo!();
    }

    #[test]
    fn display_for_date() {
        todo!();
    }

    #[test]
    fn display_for_time() {
        todo!();
    }
}
