use crate::GenericError;
use url::Url;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GenericResult;

    #[test]
    fn named_node_from_str() -> GenericResult<()> {
        let n: NamedNode = NamedNode::try_from("https://acme.org")?;
        let expected = "https://acme.org/".to_string();
        assert_eq!(expected, n.value);
        Ok(())
    }
}
