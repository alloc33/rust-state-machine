use std::{collections::BTreeMap, ops::AddAssign};

use num::{One, Zero};

/*
	TODO:
	Update the `Pallet` struct to be generic over the `AccountId`, `BlockNumber`, and `Nonce` type.
	You won't need the type definitions above after you are done.
	Types will now be defined in `main.rs`. See the TODOs there.
*/

pub trait Config {
	type AccountId: Ord + Clone;
	type BlockNumber: Zero + One + AddAssign + Copy;
	type Nonce: Zero + One + Copy;
}

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// The current block number.
	block_number: T::BlockNumber,
	/// A map from an account to their nonce.
	nonce: BTreeMap<T::AccountId, T::Nonce>,
}

/*
	TODO:
	The generic types need to satisfy certain traits in order to be used in the functions below.
	See if you can figure them out yourself.

	NOTE: You might need to adjust some of the functions below to satisfy the borrow checker.
*/

impl<T: Config> Pallet<T> {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self {
			block_number: T::BlockNumber::zero(),
			nonce: BTreeMap::<T::AccountId, T::Nonce>::new(),
		}
	}

	/// Get the current block number.
	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number += T::BlockNumber::one();
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &T::AccountId) {
		let nonce: T::Nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
		let new_nonce = nonce + T::Nonce::one();
		self.nonce.insert(who.clone(), new_nonce);
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;
	impl super::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn init_system() {
		/*
			TODO:
			When creating an instance of `Pallet`, you should explicitly define the types you use.
		*/
		let mut system = super::Pallet::<TestConfig>::new();
		system.inc_block_number();
		system.inc_nonce(&"alice".to_string());

		assert_eq!(system.block_number(), 1);
		assert_eq!(system.nonce.get("alice"), Some(&1));
		assert_eq!(system.nonce.get("bob"), None);
	}
}
