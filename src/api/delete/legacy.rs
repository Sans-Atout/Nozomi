use crate::SecureDelete;

use super::builder::DeleteRequestBuilder;

impl From<SecureDelete> for DeleteRequestBuilder {
    fn from(sd: SecureDelete) -> Self {
        DeleteRequestBuilder::new().path(sd.path)
    }
}

#[cfg(test)]
mod tests {
    use crate::api::delete::DeleteRequestBuilder;
    use crate::{Error, SecureDelete};
    use pretty_assertions::assert_eq;
    use std::fs::File;

    #[cfg(not(feature = "error-stack"))]
    use crate::Result;

    #[cfg(feature = "error-stack")]
    use crate::Result;
    #[cfg(feature = "error-stack")]
    use error_stack::{Report, ResultExt};

    #[test]
    #[cfg(not(feature = "error-stack"))]
    fn from() -> Result<()> {
        File::create("/tmp/request_builder_from").map_err(|e| Error::FileCreationError(e))?;
        let tested = SecureDelete::new("/tmp/request_builder_from")?;
        let wanted = DeleteRequestBuilder::new().path("/tmp/request_builder_from");
        assert_eq!(wanted, DeleteRequestBuilder::from(tested));
        std::fs::remove_file("/tmp/request_builder_from")
            .map_err(|e| Error::FileCreationError(e))?;
        Ok(())
    }

    #[test]
    #[cfg(feature = "error-stack")]
    fn from() -> Result<()> {
        File::create("/tmp/request_builder_from").change_context(Error::FileCreationError)?;
        let tested = SecureDelete::new("/tmp/request_builder_from")?;
        let wanted = DeleteRequestBuilder::new().path("/tmp/request_builder_from");
        assert_eq!(wanted, DeleteRequestBuilder::from(tested));
        std::fs::remove_file("/tmp/request_builder_from")
            .change_context(Error::FileCreationError)?;
        Ok(())
    }
}
