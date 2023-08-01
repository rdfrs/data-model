use rdf_data_model::{Dataset, GenericResult, Quad};

#[test]
fn build_dataset_from_scratch() -> GenericResult<()> {
    let mut ds = Dataset::default();
    // assuming I can miraculously figure out how to leverage std lib traits to pull off the below,
    // see whether I can do the same thing such that explicitly constructing quad isn't needed
    // either because I can implement `Into<Quad>` as the type for the parameter and then implement
    // it for something like a typed tuple
    ds.push(Quad::new()?);

    assert_eq!(ds.len(), 5);

    Ok(())
}
