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
    nozomi::Method::Afssi5020;

    //SecureDelete::new("README.md").unwrap().overwrite().unwrap();
}
