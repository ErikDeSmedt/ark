pub mod fixture;
pub mod util;

extern crate anyhow;
extern crate rand;
extern crate bitcoin;

use self::anyhow::Result;
use std::path::PathBuf;
use self::fixture::Fixture;
use std::sync::Arc;

pub struct TestConfig {
	test_id: String,
	base_path: PathBuf,
}

pub struct ArkFixture {
	config: TestConfig,
	bitcoind: Vec<Arc<bitcoind::BitcoinD>>
}

impl ArkFixture {

	pub fn bitcoind(&mut self) -> Arc<bitcoind::BitcoinD> {
		let mut config = bitcoind::Conf::default();
		config.args.push("-txindex");

		self.bitcoind_with_config(&config)
	}

	pub fn bitcoind_with_config(&mut self, config: &bitcoind::Conf) -> Arc<bitcoind::BitcoinD> {
		let bitcoind_exe = bitcoind::exe_path().unwrap();
		let bitcoind = Arc::new(bitcoind::BitcoinD::with_conf(bitcoind_exe, config).unwrap());

		self.bitcoind.push(bitcoind.clone());
		bitcoind
	}
}

impl Default for TestConfig {

	fn default() -> Self {
		let test_id = util::random_string();
		let base_path = ["tmp", &test_id].iter().collect();

		Self {
			test_id,
			base_path,
		}
	}
}

impl TestConfig {
	pub fn setup(self) -> Result<ArkFixture> {
		std::fs::create_dir_all(self.base_path.clone())?;
		Ok(
			ArkFixture {
				config: self,
				bitcoind: Vec::new()
			}

		)
	}
}

impl Drop for ArkFixture {
	fn drop(&mut self) {
		if self.config.base_path.exists() {
			std::fs::remove_dir_all(self.config.base_path.clone()).unwrap();
		}
	}
}
