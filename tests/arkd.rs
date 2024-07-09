extern crate bitcoind;
extern crate bitcoin;

mod common;

use bitcoind::bitcoincore_rpc::RpcApi;

use common::TestConfig;

#[test]
fn one_plus_one() {
	assert_eq!(1+1, 2)
}

#[test]
fn create_ark_fixture() {
	// Launches a default fixture
	// The fixture has a running `bitcoind` and `aspd`
	let config = TestConfig::default();
	let mut fixture = config.setup().unwrap();

	//
	let bitcoind = fixture.bitcoind();
	let networkinfo = bitcoind.client.get_blockchain_info().unwrap();
	assert_eq!(networkinfo.chain.to_string(), "regtest");

	// The fixture is dropped and auto-clean will occur
	// You can set TEST_LEAVE_INTACT to ensure data directories
	// will not be removed
}
