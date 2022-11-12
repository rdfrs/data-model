use crate::error::Error;
use crate::terms::xsd_type::XsdType;
use crate::terms::NamedNode;

#[derive(PartialEq, Eq, Debug)]
pub struct Literal<T: XsdType> {
    value: T,
    language: Option<String>, // Note: this should change to conform to a standard list of language codes
    data_type: NamedNode,
}

impl<T: XsdType> Literal<T> {
    pub fn new(value: T) -> Result<Self, Error> {
        Ok(Literal {
            value,
            language: None,
            data_type: T::xsd_type(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use time::Time;

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
    fn display_for_english_string() {
        let expected = Literal {
            value: "foo".to_string(),
            language: None,
            data_type: NamedNode {
                value: "http://www.w3.org/2001/XMLSchema#string".to_string(),
            },
        };

        let l1 = Literal::new("foo".to_string()).unwrap();
        assert_eq!(expected, l1);
    }

    #[test]
    fn display_for_english_slice() {
        let expected = Literal {
            value: "foo",
            language: None,
            data_type: NamedNode {
                value: "http://www.w3.org/2001/XMLSchema#string".to_string(),
            },
        };

        let l1 = Literal::new("foo").unwrap();
        assert_eq!(expected, l1);
    }

    #[test]
    fn display_for_spanish_string() {
        todo!();
    }

    #[test]
    fn display_for_integer() {
        let expected = Literal {
            value: 42,
            language: None,
            data_type: NamedNode {
                value: "http://www.w3.org/2001/XMLSchema#integer".to_string(),
            },
        };

        let l1 = Literal::new(42).unwrap();
        assert_eq!(expected, l1);
    }

    #[test]
    fn display_for_decimal() {
        let expected = Literal {
            value: 42.42,
            language: None,
            data_type: NamedNode {
                value: "http://www.w3.org/2001/XMLSchema#double".to_string(),
            },
        };

        let l1 = Literal::new(42.42).unwrap();
        assert_eq!(expected, l1);
    }

    #[test]
    fn display_for_boolean() {
        let expected = Literal {
            value: true,
            language: None,
            data_type: NamedNode {
                value: "http://www.w3.org/2001/XMLSchema#boolean".to_string(),
            },
        };

        let l1 = Literal::new(true).unwrap();
        assert_eq!(expected, l1);
    }

    #[test]
    fn display_for_datetime() {
        let expected = Literal {
            value: Utc::today().and_hms(0, 0, 0),
            language: None,
            data_type: NamedNode {
                value: "http://www.w3.org/2001/XMLSchema#dateTime".to_string(),
            },
        };

        let l1 = Literal::new(Utc::today().and_hms(0, 0, 0)).unwrap();
        assert_eq!(expected, l1);
    }

    #[test]
    fn display_for_date() {
        let expected = Literal {
            value: Utc::today(),
            language: None,
            data_type: NamedNode {
                value: "http://www.w3.org/2001/XMLSchema#date".to_string(),
            },
        };

        let l1 = Literal::new(Utc::today()).unwrap();
        assert_eq!(expected, l1);
    }

    #[test]
    fn display_for_time() {
        let expected = Literal {
            value: Time::from_hms(12, 0, 0).unwrap(),
            language: None,
            data_type: NamedNode {
                value: "http://www.w3.org/2001/XMLSchema#time".to_string(),
            },
        };

        let l1 = Literal::new(Time::from_hms(12, 0, 0).unwrap()).unwrap();
        assert_eq!(expected, l1);
    }
}
