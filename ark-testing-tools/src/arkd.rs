use anyhow::Result;

use std::path::PathBuf;
use std::fs;
use std::fs::create_dir_all;
use std::process::{Command, ExitStatus};
use std::sync::{Arc, Mutex};
use std::fmt;
use std::borrow::Borrow;
use std::io::prelude::*;

use crate::runner::{RuntimeData, RunnerHelper, DaemonRunner};
use crate::error::Error;

pub struct ArkD {
	name: String,
	exe_path: String,
	default_args: Vec<String>,
  datadir: PathBuf,
	bitcoind_cookie: PathBuf,
	bitcoind_url: String,
	network: bitcoin::Network,
	runtime_data: Option<Arc<Mutex<RuntimeData<State>>>>
}

impl ArkD {

	pub fn new<B>(name: String, ark_cmd: &[&str], datadir: PathBuf, bitcoind: B) -> Self 
		where B : Borrow<bitcoind::BitcoinD>
	{
		Self {
			name,
			exe_path: ark_cmd[0].to_string(),
			default_args: ark_cmd.iter().skip(1).map(|x| x.to_string()).collect(),
			datadir,
			bitcoind_cookie: bitcoind.borrow().params.cookie_file.clone(),
			bitcoind_url: bitcoind.borrow().params.rpc_socket.to_string(),
			network: bitcoin::Network::Regtest,
			runtime_data: None,
		}
	}
}

impl fmt::Debug for ArkD {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?} --datadir {:?}", self.exe_path, self.datadir)
	}
}

#[derive(Debug)]
pub struct State{
	stdout: Option<fs::File>,
	stderr: Option<fs::File>
}

impl State {

	fn process_stdout(&mut self, name: &str, line: &str) {
		match &mut self.stdout {
			Some(file) => { 
				let _ = write!(file, "{} - {}\n", name, line);},
			_ => {}
		}
	}

	fn process_stderr(&mut self, line: &str) {
		match &mut self.stderr {
			Some(file) => {
				let _ = write!(file, "{}\n", line);
			},
			None => {}
		}
	}
}

impl RunnerHelper for ArkD {
	type State = State;

	fn _prepare(&mut self) -> Result<(), Error> {
		trace!("_prepare {}", self.name);
		create_dir_all(self.datadir.clone()).unwrap();

		// Create and initialize the datadir
		let network = self.network.to_string();
		let mut command = Command::new(self.exe_path.clone());
		command
			.args(self.default_args.clone())
			.arg("create")
			.arg("--datadir")
			.arg(self.datadir.clone())
			.arg("--bitcoind-url")
			.arg(self.bitcoind_url.clone())
			.arg("--bitcoind-cookie")
			.arg(self.bitcoind_cookie.clone())
			.arg("--network")
			.arg(network);

		let output = command.output()?;
		if output.status.success() {
			info!("Created {}", self.name)
		}
		else {
			error!("Created arkd with stderr: {}", std::str::from_utf8(&output.stderr).unwrap());
			panic!("Failed to create {}", self.name)
		}

		Ok(())
	}

	fn _command(&self) -> Command {
		let mut command = Command::new(self.exe_path.clone());
		command
			.args(self.default_args.clone())
			.arg("start")
			.arg("--datadir")
			.arg(self.datadir.clone());

		command
	}


	fn _process_stdout(name: &str, state: &mut Self::State, line: &str) {
		state.process_stdout(name, line);
	}


	fn _process_stderr(state: &mut Self::State, line: &str) {
		state.process_stderr(line);
		//TODO: Impelement
	}

	fn _init_state(&self) -> Self::State {
		// Create the log-files
		let stdout = fs::OpenOptions::new()
			.append(true)
			.create(true)
			.open(self.datadir.join("stdout.log")).unwrap();

		let stderr = fs::OpenOptions::new()
			.append(true)
			.create(true)
			.open(self.datadir.join("stderr.log")).unwrap();


		Self::State { 
			stdout: Some(stdout),
			stderr: Some(stderr)
		}
	}

	fn _get_runtime(&self) -> Option<Arc<Mutex<RuntimeData<Self::State>>>>  {
		self.runtime_data.clone()
	}

	fn _notif_started(&mut self, runtime_data: Arc<Mutex<RuntimeData<Self::State>>>) {
		self.runtime_data.replace(runtime_data);
	}
}

impl DaemonRunner for ArkD {

}
