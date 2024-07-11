use anyhow::Result;

use bitcoin::Network;
use std::path::PathBuf;

pub struct Arkd {
	ark_exe: PathBuf,
  datadir: PathBuf,
	bitcoind_url: String,
	bitcoind_cookie: String,
	network: Network
}

impl Arkd {

	fn create() {

	}
}
