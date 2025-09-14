mod balances;
mod support;
mod system;

mod types {
	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
	type Balance = types::Balance;
}

fn main() {
	let mut runtime = Runtime::new();
	runtime.balances.set_balance(&"alice".to_string(), 100);

	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(), 1);

	runtime.system.inc_nonce(&"alice".to_string());
	_ = runtime
		.balances
		.transfer("alice".to_string(), "bob".to_string(), 30)
		.map_err(|e| println!("{e:?}"));

	runtime.system.inc_nonce(&"alice".to_string());
	_ = runtime
		.balances
		.transfer("alice".to_string(), "charlie".to_string(), 20)
		.map_err(|e| println!("{e:?}"));

	println!("{runtime:#?}");
}
