use crate::SecureDelete;

use super::builder::DeleteRequestBuilder;

impl From<SecureDelete> for DeleteRequestBuilder {
	fn from(sd: SecureDelete) -> Self {
		DeleteRequestBuilder::new()
			.path(sd.path)
	}
}

#[cfg(test)]
mod tests{
	use std::fs::File;
	use crate::api::delete::DeleteRequestBuilder;
	use crate::{Error, SecureDelete};
	use pretty_assertions::assert_eq;

	#[cfg(not(feature = "error-stack"))]
	use crate::Result;

	#[test]
	#[cfg(not(feature = "error-stack"))]
	fn from() -> Result<()>{
		File::create("/tmp/request_builder_from").map_err(|e| Error::FileCreationError(e))?;
		let tested = SecureDelete::new("/tmp/request_builder_from")?;
		let wanted = DeleteRequestBuilder::new().path("/tmp/request_builder_from");
		assert_eq!(wanted,DeleteRequestBuilder::from(tested));
		std::fs::remove_file("/tmp/request_builder_from").map_err(|e| Error::FileCreationError(e))?;
		Ok(())
	}

}
