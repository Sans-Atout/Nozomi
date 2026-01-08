
// -- Region : Module export
mod dod_522022_me;
mod dod_522022_mece;
mod afssi_5020;
mod gutmann;
mod hmgi_s5;
mod pseudo_random;
mod rcmp_tssit_ops_ii;
mod common;

use std::path::PathBuf;
use crate::{Error, Method};

#[cfg(not(feature = "error-stack"))]
use crate::Result;

#[cfg(not(feature = "error-stack"))]
pub(crate) fn overwrite_file(method: &Method, path : &PathBuf) -> Result<()> {
	match method {
		Method::Dod522022MECE => dod_522022_me::overwrite_file(path)?,
		Method::Dod522022ME => dod_522022_mece::overwrite_file(path)?,
		Method::Afssi5020 => afssi_5020::overwrite_file(path)?,
		Method::RcmpTssitOpsII => rcmp_tssit_ops_ii::overwrite_file(path)?,
		Method::HmgiS5 => hmgi_s5::overwrite_file(path)?,
		Method::Gutmann => gutmann::overwrite_file(path)?,
		Method::PseudoRandom => pseudo_random::overwrite_file(path)?,
	};
	Ok(())
}