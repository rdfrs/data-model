#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BlankNode {
    local_name: String,
}

impl From<&str> for BlankNode {
    fn from(value: &str) -> Self {
        BlankNode {
            local_name: value.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GenericResult;

    #[test]
    fn create_blank_node() -> GenericResult<()> {
        let b = BlankNode::from("b0");
        let expected = BlankNode {
            local_name: "b0".to_string(),
        };
        assert_eq!(expected, b);
        Ok(())
    }
}
