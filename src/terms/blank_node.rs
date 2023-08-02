use super::{Object, Subject};
use crate::Result;
use std::{
    fmt::{Display, Formatter},
    ops::Sub,
};

#[derive(PartialEq, Eq, Debug)]
pub struct BlankNode {
    value: String,
}

impl Subject for BlankNode {}

impl Object for BlankNode {}

impl BlankNode {
    pub fn new(value: &str) -> Result<Self> {
        Ok(BlankNode {
            value: value.to_string(),
        })
    }
}

impl Display for BlankNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "_:{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        let bn1 = BlankNode {
            value: "foo".to_string(),
        };
        let bn2 = BlankNode {
            value: "foo".to_string(),
        };

        assert_eq!(bn1, bn2);
        assert_eq!(bn2, bn1);
    }

    #[test]
    fn display() {
        let expected = "_:b1";
        let bn = BlankNode::new("b1").expect("error creating BlankNode");
        assert_eq!(expected, format!("{bn}"));
    }
}
