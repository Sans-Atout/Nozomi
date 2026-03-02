use std::path::Path;
use nozomi::Method::PseudoRandom;

mod events;

#[cfg(test)]
#[cfg_attr(test, allow(unused_imports, path_statements))]
#[test]
fn api_export() {
    use nozomi::Error;
    use nozomi::Method;
    use nozomi::SecureDelete;

    #[cfg(feature = "error-stack")]
    use nozomi::DeleteRequest;

    Method::Dod522022ME;
    Method::Afssi5020;

    //SecureDelete::new("README.md").unwrap().overwrite().unwrap();
}

#[test]
fn default_build_has_no_optional_capabilities() {
    use nozomi::*;
    let builder = DeleteRequest::builder().path(Path::new("/path/to/nozomi")).method(DeleteMethod::BuiltIn(PseudoRandom));
    let result = builder.build();
    assert!(result.is_ok());
}
