pub mod arkd;
mod constants;
pub mod command;
pub mod error;
pub mod runner;
mod util;

#[macro_use]
extern crate log;

use bitcoind::BitcoinD;
use arkd::ArkD;
use std::fs;
use std::path::PathBuf;
use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub struct TestContext<'a> {
	#[allow(dead_code)]
	pub name: String,
	pub network: &'a str,
	pub datadir: PathBuf,
	bitcoind_count: u64,
	arkd_count: u64
}

impl TestContext<'static> {

	fn new(name: String, base_path: PathBuf) -> Self {
		fs::create_dir_all(base_path.clone()).unwrap();
		TestContext { name, datadir: base_path, arkd_count:0, bitcoind_count:0, network: "regtest" }
	}

	pub fn bitcoind(&mut self) -> BitcoinD {
		// Configure `bitcoind`
		// Ensure that it starts within our directory
		// and `-txindex` is enabled.
		// 
		// The `TestContext` struct is responsible for clean-up
		// By setting `staticdir` we ensure the `bitcoind`-crate
		// never cleans up after itself.
		//
		info!("Creating arkd-{}", self.arkd_count);
		println!("Creating log arkd-{}", self.arkd_count);
		let bitcoin_datadir = self.datadir.join(format!("bitcoind-{}", self.bitcoind_count));
		self.arkd_count+=1;

		let mut conf = bitcoind::Conf::default();
		conf.args.push("-txindex");
		conf.staticdir = Some(bitcoin_datadir);

		// Find the path to `bitcoind`
		let exe_path = bitcoind::exe_path().unwrap();
		debug!("Launch `bitcoind` with exe_path={:?}", exe_path);

		BitcoinD::with_conf(exe_path, &conf).unwrap()
	}

	pub fn arkd<B : Borrow<BitcoinD>>(&mut self, bitcoind : B) -> arkd::ArkD {
		let arkd_name = format!("arkd-{}", self.arkd_count);
		self.arkd_count+=1;
		info!("Creating {}", &arkd_name);
		let ark_datadir = self.datadir.join(&arkd_name);

		// Get the executable
		// By default we use `cargo run --bin arkd --`
		// We can also extract the executable from an environment variable
		let exec = std::env::var(constants::ENV_ARKD_EXECUTABLE);
		let executable = match &exec {
			Ok(exec) => {
				debug!("arkd executable is `{}`", exec);
				let parts = exec.split(" ");
				let result : Vec<&str> = parts.collect();
				result
			}
			Err(_) => {
				debug!("arkd is compiled from source");
				vec!["cargo", "run", "--bin", "arkd", "--"]
			}
		};

		ArkD::new(
			arkd_name,
			&executable,
			ark_datadir,
			bitcoind
		)
	}
}

impl Default for TestContext<'static> {

	fn default() -> Self {
		let name = util::random_string();
		let datadir = ["tmp", &name].iter().collect();

		Self::new(name, datadir)
	}
}

impl Drop for TestContext<'_> {
	fn drop(&mut self) {

		// Remove the data-directory
		// If the user has set `LEAVE_INTACT` we don't delete any 
		// test-data.
		if let Ok(_) = std::env::var(constants::ENV_LEAVE_INTACT) {
			return
		}
		if self.datadir.exists() {
			log::info!("Test clean-up");
			std::fs::remove_dir_all(self.datadir.clone()).unwrap();
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
				let base_path = context.datadir.clone();

				// The base-path is created
				assert!(context.datadir.exists());
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
