

struct ArkD {
	process: Child,
}

struct ArkdConf {

}

impl ArkD {
	pub fn with_conf<S: AsRef<OsStr>>(exe: S, conf: &Conf) -> anyhow::Result<ArkD> {
		let mut process = Command::new(exe.as_ref())
			.args(datadir_args)
			.args(bitcoind_args)
			.spawn()
			.with_context(|| format("Failed to launch Ark"))
			


	}


}
