// XSD Primitive Types that we're going to support
// -----------------------------------------------
// integer  - 32 bit - chosen as i32 is the default integer type in rust
// boolean
// date
// dateTime
// double   - 64 bit - chosen because f64 is the default floating-point type in rust
// string
// time

use crate::terms::NamedNode;
use chrono::{Date, DateTime, Utc};
use time::Time;

pub trait XsdType {
    fn xsd_type() -> NamedNode;
}

impl XsdType for String {
    fn xsd_type() -> NamedNode {
        NamedNode::new("http://www.w3.org/2001/XMLSchema#string").unwrap()
    }
}

impl XsdType for &str {
    fn xsd_type() -> NamedNode {
        NamedNode::new("http://www.w3.org/2001/XMLSchema#string").unwrap()
    }
}

impl XsdType for u32 {
    fn xsd_type() -> NamedNode {
        NamedNode::new("http://www.w3.org/2001/XMLSchema#integer").unwrap()
    }
}

impl XsdType for f32 {
    fn xsd_type() -> NamedNode {
        NamedNode::new("http://www.w3.org/2001/XMLSchema#double").unwrap()
    }
}

impl XsdType for f64 {
    fn xsd_type() -> NamedNode {
        NamedNode::new("http://www.w3.org/2001/XMLSchema#double").unwrap()
    }
}

impl XsdType for bool {
    fn xsd_type() -> NamedNode {
        NamedNode::new("http://www.w3.org/2001/XMLSchema#boolean").unwrap()
    }
}

impl XsdType for DateTime<Utc> {
    fn xsd_type() -> NamedNode {
        NamedNode::new("http://www.w3.org/2001/XMLSchema#dateTime").unwrap()
    }
}

impl XsdType for Date<Utc> {
    fn xsd_type() -> NamedNode {
        NamedNode::new("http://www.w3.org/2001/XMLSchema#date").unwrap()
    }
}

impl XsdType for Time {
    fn xsd_type() -> NamedNode {
        NamedNode::new("http://www.w3.org/2001/XMLSchema#time").unwrap()
    }
}
