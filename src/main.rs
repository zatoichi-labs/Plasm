//! Substrate Node Template CLI library.

#![warn(missing_docs)]
#![warn(unused_extern_crates)]

pub mod chain_spec;
mod service;
mod cli;

pub use substrate_cli::{VersionInfo, IntoExit, error};

fn run() -> cli::error::Result<()> {
	let version = VersionInfo {
		name: "Substrate Node",
		commit: env!("VERGEN_SHA_SHORT"),
		version: env!("CARGO_PKG_VERSION"),
		executable_name: "plasm-node",
		author: "takumi",
		description: "plasm-node",
		support_url: "support.anonymous.an",
	};
	cli::run(::std::env::args(), cli::Exit, version)
}

error_chain::quick_main!(run);

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_for_demo() {
		println!("Alice: {:?}",chain_spec::account_key("Alice"));
		println!("Bob  : {:?}",chain_spec::account_key("Bob"));
	}
}
