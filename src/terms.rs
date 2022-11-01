use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("{0} is not a valid absolute IRI")]
    InvalidURIError(String), // Question: is it possible to accept both the string _and_ a #[from] parameter? does this even make sense?
}

#[derive(PartialEq, Eq, Debug)]
pub struct NamedNode {
    value: String, // is there a way to hide the field since I'm validating it in the `new` fn
}

// TODO: need a more robust regex
const ABSOLUTE_URI_EXPRESSION: &str = r"^(?:http|https)://(?:\w+\.)+\w+/.*$";

impl NamedNode {
    fn new(value: &str) -> Result<Self, Error> {
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

#[derive(PartialEq, Eq, Debug)]
pub struct BlankNode {
    value: String,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Literal {
    value: String,
    language: String, // Note: this should change to conform to a standard list of language codes
    data_type: NamedNode,
}

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
    let bn1 = BlankNode {
        value: "foo".to_string(),
    };
    let bn2 = BlankNode {
        value: "foo".to_string(),
    };

    assert_eq!(nn1, nn1);
    assert_ne!(nn1, nn2);
    assert_eq!(nn1, nn3);
    assert_eq!(nn3, nn1);
    assert_eq!(bn1, bn2);
    assert_eq!(bn2, bn1);
}

#[test]
fn test_valid_named_node_new() {
    let nn = NamedNode {
        value: "https://foo.com/bar".to_string(),
    };

    assert_eq!(NamedNode::new("https://foo.com/bar").unwrap(), nn);
}

#[test]
fn test_invalid_named_node_new() {
    let r = NamedNode::new("foo");
    assert_eq!(r.is_err(), true);
    assert_eq!(r.err(), Some(Error::InvalidURIError("foo".to_string())));
}
