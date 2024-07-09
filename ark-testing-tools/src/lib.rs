mod constants;
mod util;

use bitcoind::BitcoinD;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TestContext {
	#[allow(dead_code)]
	name: String,
	base_path: PathBuf,
	bitcoind: Vec<Arc<bitcoind::BitcoinD>>,
}

impl TestContext {

	fn new(name: String, base_path: PathBuf) -> Self {
		fs::create_dir_all(base_path.clone()).unwrap();
		TestContext { name, base_path, bitcoind: Vec::new() }
	}

	pub fn bitcoind(&mut self) -> Arc<BitcoinD> {
		// Find the path to `bitcoind`
		let exepath = bitcoind::exe_path().unwrap();

		// Configure `bitcoind`
		// Ensure that it starts within our directory
		// and `-txindex` is enabled.
		// 
		// The `TestContext` struct is responsible for clean-up
		// By setting `staticdir` we ensure the `bitcoind`-crate
		// never cleans up after itself.
		let mut pathbuf = self.base_path.clone();
		pathbuf.push(format!("bitcoind-{}", self.bitcoind.len()));

		let mut conf = bitcoind::Conf::default();
		conf.args.push("-txindex");
		conf.staticdir = Some(pathbuf);

		let arc_bitcoind = Arc::new(BitcoinD::with_conf(exepath, &conf).unwrap());
		self.bitcoind.push(arc_bitcoind.clone());
		return arc_bitcoind;
	}
}

impl Default for TestContext {

	fn default() -> Self {
		let name = util::random_string();
		let base_path = ["tmp", &name].iter().collect();

		Self::new(name, base_path)
	}
}

impl Drop for TestContext {
	fn drop(&mut self) {

		// Remove the data-directory
		// If the user has set `LEAVE_INTACT` we don't delete any 
		// test-data.
		if let Ok(_) = std::env::var(constants::ENV_LEAVE_INTACT) {
			return
		}
		if self.base_path.exists() {
			std::fs::remove_dir_all(self.base_path.clone()).unwrap();
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;
		use bitcoind::bitcoincore_rpc::RpcApi;

    #[test]
    fn context_creates_and_deletes_datadir() {
				let context = TestContext::default();
				let base_path = context.base_path.clone();

				// The base-path is created
				assert!(context.base_path.exists());
				drop(context);

				// The test cleans up after itself
				match std::env::var(constants::ENV_LEAVE_INTACT) {
					Ok(_) => assert!(base_path.exists()),
					Err(_) => assert!(!base_path.exists())
				}
    }

		#[test]
		fn create_bitcoin_node() {
			let mut context = TestContext::default();
			let bitcoind = context.bitcoind();

			let networkinfo = bitcoind.client.get_blockchain_info().unwrap();
			assert_eq!(networkinfo.chain.to_string(), "regtest");
		}
}
