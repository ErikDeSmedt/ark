use anyhow::Result;

use bitcoin::Network;
use std::path::PathBuf;
use std::fs::create_dir_all;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::fmt;

use crate::runner::{RuntimeData, RunnerHelper, DaemonRunner};
use crate::error::Error;

pub struct ArkD {
	ark_exe: PathBuf,
  datadir: PathBuf,
	bitcoind_url: String,
	bitcoind_cookie: String,
	network: Network,
	runtime_data: Option<Arc<Mutex<RuntimeData<State>>>>
}

impl fmt::Debug for ArkD {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?} --datadir {:?}", self.ark_exe, self.datadir)
	}
}

#[derive(Debug, Clone)]
pub struct State{}

impl ArkD {

	fn start_with_retries(&mut self, retries: usize) -> Result<()> {
		let attempts = 0;
		while attempts <= retries {
			self.start();
		};

		Ok(())

	}
}

impl DaemonRunner for ArkD {}

impl RunnerHelper for ArkD {
	type State = State;

	fn _prepare(&mut self) -> Result<(), Error> {
		create_dir_all(self.datadir.clone()).unwrap();

		// Create and initialize the datadir
		let network = self.network.to_string();
		let mut command = Command::new(self.ark_exe.clone());
		command
			.arg("--datadir")
			.arg(self.datadir.clone())
			.arg("--bitcoind-url")
			.arg(self.bitcoind_url.clone())
			.arg("--bitcoind-cookie")
			.arg(self.bitcoind_cookie.clone())
			.arg("--network")
			.arg(network);

		command.output()?;
		Ok(())
	}

	fn _command(&self) -> Command {
		let mut command = Command::new(self.ark_exe.clone());
		command
			.arg("start")
			.arg("--datadir")
			.arg(self.datadir.clone());

		command
	}

	fn _process_stdout(name: &str, state: &mut Self::State, line: &str) {
		//TODO: Implement
	}

	fn _process_stderr(state: &mut Self::State, line: &str) {
		//TODO: Impelement
	}

	fn _init_state(&self) -> Self::State {
		Self::State {}
	}

	fn _get_runtime(&self) -> Option<Arc<Mutex<RuntimeData<Self::State>>>>  {
		self.runtime_data.clone()
	}

	fn _notif_started(&mut self, runtime_data: Arc<Mutex<RuntimeData<Self::State>>>) {
		self.runtime_data.replace(runtime_data);
	}
}
