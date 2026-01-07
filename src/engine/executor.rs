use std::path::Path;
use crate::{Error, Method};

use super::planner;
use super::overwrite;


#[cfg(not(feature = "error-stack"))]
pub(crate) fn run(method: &Method, path: &Path) -> Result<(), Error> {
	let plan = planner::execution_plan(path)?;
	for file_path in &plan.files{
		overwrite::overwrite_file(method,file_path)?;
	}
	Ok(())
}
