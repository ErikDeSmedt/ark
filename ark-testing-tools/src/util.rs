use rand::RngCore;
use bitcoin::base58;
use std::net::TcpListener;

pub fn random_string() -> String {
	// Generate a few random bytes and base58 encode them
	// The entropy should be sufficient to generate unique test-names
	let mut entropy :[u8; 8] = [0; 8];
	rand::thread_rng().fill_bytes(&mut entropy);
	base58::encode(&entropy)
}

/// Returns a non-used local port if available.
///
/// Note there is a race condition during the time the method check availability and the caller
pub fn get_available_port() -> anyhow::Result<u16> {
    // using 0 as port let the system assign a port available
    let t = TcpListener::bind(("127.0.0.1", 0))?; // 0 means the OS choose a free port
    Ok(t.local_addr().map(|s| s.port())?)
}
