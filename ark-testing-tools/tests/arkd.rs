use ark_testing_tools::TestContext;
use ark_testing_tools::runner::DaemonRunner;
use ark_testing_tools::arkd::ArkD;
use env_logger;

#[test]
fn run_and_clean_arkd() {
	env_logger::init();

	let mut context = TestContext::default();
	let bitcoind1 = context.bitcoind();
	let mut arkd1 : ArkD = context.arkd(bitcoind1);
	arkd1.start().unwrap();

	std::thread::sleep(std::time::Duration::from_millis(1000));

	assert_eq!(context.datadir.as_os_str().to_str(), Some("erik"));

} 
