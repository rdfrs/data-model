use crate::error::Error;
use crate::terms::xsd_type::XsdType;
use crate::terms::NamedNode;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Eq, Debug)]
pub struct Literal<T: XsdType + Display> {
    value: T,
    language: Option<String>, // Note: this should change to conform to a standard list of language codes
    data_type: NamedNode,
}

impl<T: XsdType + Display> Literal<T> {
    pub fn new(value: T) -> Result<Self, Error> {
        Ok(Literal {
            value,
            language: None,
            data_type: T::xsd_type(),
        })
    }

    pub fn with_language(mut self, language: &str) -> Result<Self, Error> {
        self.language = Some(language.to_string());
        Ok(self)
    }
}

impl<T: XsdType + Display> Display for Literal<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let xsd_type = T::xsd_type();
        if xsd_type == NamedNode::new("http://www.w3.org/2001/XMLSchema#string").unwrap() {
            match &self.language {
                Some(lang) => write!(f, "\"{}\"@{}", self.value, lang),
                None => {
                    write!(f, "\"{}\"", self.value)
                }
            }
        } else {
            write!(f, "\"{}\"^^{}", self.value, xsd_type)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use time::macros::time;

    #[test]
    fn equality() {
        let n1 = Literal {
            value: "foo".to_string(),
            language: None,
            data_type: NamedNode {
                value: "http://www.w3.org/2001/XMLSchema#string".to_string(),
            },
        };

        let n2 = Literal {
            value: "foo".to_string(),
            language: None,
            data_type: NamedNode {
                value: "http://www.w3.org/2001/XMLSchema#string".to_string(),
            },
        };

        let n3 = Literal {
            value: "bar".to_string(),
            language: None,
            data_type: NamedNode {
                value: "http://www.w3.org/2001/XMLSchema#string".to_string(),
            },
        };

        let n4 = Literal {
            value: "bar".to_string(),
            language: Some("ES".to_string()),
            data_type: NamedNode {
                value: "http://www.w3.org/2001/XMLSchema#string".to_string(),
            },
        };

        assert_eq!(n1, n2);
        assert_ne!(n1, n3);
        assert_ne!(n3, n4);
    }

    #[test]
    fn construct_english_string() -> Result<(), Error> {
        let expected = Literal {
            value: "foo".to_string(),
            language: None,
            data_type: NamedNode {
                value: "http://www.w3.org/2001/XMLSchema#string".to_string(),
            },
        };

        let l1 = Literal::new("foo".to_string())?;
        assert_eq!(expected, l1);
        Ok(())
    }

    #[test]
    fn construct_english_slice() -> Result<(), Error> {
        let expected = Literal {
            value: "foo",
            language: None,
            data_type: NamedNode {
                value: "http://www.w3.org/2001/XMLSchema#string".to_string(),
            },
        };

        let l1 = Literal::new("foo")?;
        assert_eq!(expected, l1);
        Ok(())
    }

    #[test]
    fn construct_spanish_string() -> Result<(), Error> {
        let expected = Literal {
            value: "Hola Mundo",
            language: Some("es".to_string()),
            data_type: NamedNode {
                value: "http://www.w3.org/2001/XMLSchema#string".to_string(),
            },
        };

        // let l1 = Literal::for_language("Hola Mundo", "es");
        let l2 = Literal::new("Hola Mundo")?.with_language("es")?;

        assert_eq!(expected, l2);
        Ok(())
    }

    #[test]
    fn construct_integer() -> Result<(), Error> {
        let expected = Literal {
            value: 42,
            language: None,
            data_type: NamedNode {
                value: "http://www.w3.org/2001/XMLSchema#integer".to_string(),
            },
        };

        let l1 = Literal::new(42)?;
        assert_eq!(expected, l1);
        Ok(())
    }

    #[test]
    fn construct_decimal() -> Result<(), Error> {
        let expected = Literal {
            value: 42.42,
            language: None,
            data_type: NamedNode {
                value: "http://www.w3.org/2001/XMLSchema#double".to_string(),
            },
        };

        let l1 = Literal::new(42.42)?;
        assert_eq!(expected, l1);
        Ok(())
    }

    #[test]
    fn construct_boolean() -> Result<(), Error> {
        let expected = Literal {
            value: true,
            language: None,
            data_type: NamedNode {
                value: "http://www.w3.org/2001/XMLSchema#boolean".to_string(),
            },
        };

        let l1 = Literal::new(true)?;
        assert_eq!(expected, l1);
        Ok(())
    }

    #[test]
    fn construct_datetime() -> Result<(), Error> {
        let expected = Literal {
            value: Utc::today().and_hms(0, 0, 0),
            language: None,
            data_type: NamedNode {
                value: "http://www.w3.org/2001/XMLSchema#dateTime".to_string(),
            },
        };

        let l1 = Literal::new(Utc::today().and_hms(0, 0, 0))?;
        assert_eq!(expected, l1);
        Ok(())
    }

    #[test]
    fn construct_date() -> Result<(), Error> {
        let expected = Literal {
            value: Utc::today(),
            language: None,
            data_type: NamedNode {
                value: "http://www.w3.org/2001/XMLSchema#date".to_string(),
            },
        };

        let l1 = Literal::new(Utc::today())?;
        assert_eq!(expected, l1);
        Ok(())
    }

    #[test]
    fn construct_time() -> Result<(), Error> {
        let expected = Literal {
            value: time!(12:00:00),
            language: None,
            data_type: NamedNode {
                value: "http://www.w3.org/2001/XMLSchema#time".to_string(),
            },
        };

        let l1 = Literal::new(time!(12:00:00))?;
        assert_eq!(expected, l1);
        Ok(())
    }

    #[test]
    fn display_lang_string() {
        let l = Literal {
            value: "Hola Mundo",
            language: Some("es".to_string()),
            data_type: NamedNode {
                value: "http://www.w3.org/2001/XMLSchema#string".to_string(),
            },
        };

        let expected = "\"Hola Mundo\"@es".to_string();
        let actual = format!("{}", l);

        assert_eq!(expected, actual);
    }

    #[test]
    fn display_string() {
        let l = Literal {
            value: "Hello World",
            language: None,
            data_type: NamedNode {
                value: "http://www.w3.org/2001/XMLSchema#string".to_string(),
            },
        };

        let expected = "\"Hello World\"".to_string();
        let actual = format!("{}", l);

        assert_eq!(expected, actual);
    }

    #[test]
    fn display_non_string() {
        let l = Literal {
            value: 42,
            language: None,
            data_type: NamedNode {
                value: "http://www.w3.org/2001/XMLSchema#integer".to_string(),
            },
        };

        let expected = "\"42\"^^<http://www.w3.org/2001/XMLSchema#integer>".to_string();
        let actual = format!("{}", l);

        assert_eq!(expected, actual);
    }
}
