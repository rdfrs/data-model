use crate::error::Error;
use regex::Regex;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq, Debug)]
pub struct NamedNode {
    value: String, // TODO: create a type FQDN that can convert to/from String, url, and &str
}

// TODO: need a more robust regex
const ABSOLUTE_URI_EXPRESSION: &str = r"^(?:http|https)://(?:\w+\.)+\w+/.*$";

impl NamedNode {
    pub fn new(value: &str) -> Result<Self, Error> {
        let regex =
            Regex::new(ABSOLUTE_URI_EXPRESSION).expect("Absolute URI Regex failed to compile");

        if regex.is_match(value) {
            Ok(NamedNode {
                value: value.to_string(),
            })
        } else {
            Err(Error::InvalidURIError(value.to_string()))
        }
    }
}

impl Display for NamedNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality() {
        let nn1 = NamedNode {
            value: "foo".to_string(),
        };
        let nn2 = NamedNode {
            value: "bar".to_string(),
        };
        let nn3 = NamedNode {
            value: "foo".to_string(),
        };

        assert_eq!(nn1, nn1);
        assert_ne!(nn1, nn2);
        assert_eq!(nn1, nn3);
        assert_eq!(nn3, nn1);
    }

    #[test]
    fn new_valid_url() -> Result<(), Error> {
        let nn = NamedNode {
            value: "https://foo.com/bar".to_string(),
        };

        assert_eq!(NamedNode::new("https://foo.com/bar")?, nn);
        Ok(())
    }

    #[test]
    fn new_invalid_url() {
        let r = NamedNode::new("foo");
        assert!(r.is_err());
        assert_eq!(r.err().unwrap(), Error::InvalidURIError("foo".to_string()));
    }

    #[test]
    fn display() {
        let expected = "<https://foo.com/bar>";
        let nn = NamedNode::new("https://foo.com/bar").expect("error creating NamedNode");
        assert_eq!(expected, format!("{nn}"));
    }
}
