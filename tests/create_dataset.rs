use rdf_data_model::{BlankNode, Dataset, GenericResult, Literal, NamedNode, Quad};

#[test]
fn build_dataset_from_scratch() -> GenericResult<()> {
    let mut ds = Dataset::default();
    // assuming I can miraculously figure out how to leverage std lib traits to pull off the below,
    // see whether I can do the same thing such that explicitly constructing quad isn't needed
    // either because I can implement `Into<Quad>` as the type for the parameter and then implement
    // it for something like a typed tuple

    // <https://acme.com/JaneDoe> <http://schema.org/age> "23"^^<http://www.w3.org/2001/XMLSchema#integer> .
    // <https://acme.com/JaneDoe> <http://schema.org/foo> _:b0 .
    // <https://acme.com/JaneDoe> <http://schema.org/is_alive> "true"^^<http://www.w3.org/2001/XMLSchema#boolean> .
    // <https://acme.com/JaneDoe> <http://schema.org/jobTitle> "Professor" .
    // <https://acme.com/JaneDoe> <http://schema.org/modified> "Mon Jul 31 22:19:04 CDT 2023" .
    // <https://acme.com/JaneDoe> <http://schema.org/name> "Jane Doe" .
    // <https://acme.com/JaneDoe> <http://schema.org/telephone> "(425) 123-4567" .
    // <https://acme.com/JaneDoe> <http://schema.org/url> <http://www.janedoe.com> .
    // <https://acme.com/JaneDoe> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://schema.org/Person> .
    // _:b0 <http://schema.org/bar> "1"^^<http://www.w3.org/2001/XMLSchema#integer> .

    let jane_doe = NamedNode::try_from("https://acme.com/JaneDoe")?;
    let b0 = BlankNode::from("b0");

    // there's some good opportunity for future optimization here by memoizing terms (jane_doe in case)

    ds.push(Quad::new(
        jane_doe.clone(),
        NamedNode::try_from("http://schema.org/age")?,
        Literal::from(23),
        None,
    )?);

    ds.push(Quad::new(
        jane_doe.clone(),
        NamedNode::try_from("http://schema.org/foo")?,
        b0.clone(),
        None,
    )?);

    ds.push(Quad::new(
        jane_doe.clone(),
        NamedNode::try_from("http://schema.org/is_alive")?,
        Literal::from(true),
        None,
    )?);

    ds.push(Quad::new(
        jane_doe.clone(),
        NamedNode::try_from("http://schema.org/jobTitle")?,
        Literal::from("Professor"),
        None,
    )?);

    ds.push(Quad::new(
        b0.clone(),
        NamedNode::try_from("http://schema.org/bar")?,
        Literal::from(1),
        None,
    )?);

    assert_eq!(ds.len(), 5);

    Ok(())
}
