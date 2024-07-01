// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Multisig pallet
//! A pallet for doing multisig dispatch.
//!
//! - [`Config`]
//! - [`Call`]
//!
//! ## Overview
//!
//! This pallet contains functionality for multi-signature dispatch, a (potentially) stateful
//! operation, allowing multiple signed
//! origins (accounts) to coordinate and dispatch a call from a well-known origin, derivable
//! deterministically from the set of account IDs and the threshold number of accounts from the
//! set that must approve it. In the case that the threshold is just one then this is a stateless
//! operation. This is useful for multisig wallets where cryptographic threshold signatures are
//! not available or desired.
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * `as_multi` - Approve and if possible dispatch a call from a composite origin formed from a
//!   number of signed origins.
//! * `approve_as_multi` - Approve a call from a composite origin.
//! * `cancel_as_multi` - Cancel a call from a composite origin.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]


use frame_support::{
	dispatch::{
		GetDispatchInfo,
		PostDispatchInfo,
	},
	traits::ReservableCurrency,
};
use frame_system::pallet_prelude::BlockNumberFor;
use sp_runtime::traits::Dispatchable;

/// The log target of this pallet.
pub const LOG_TARGET: &'static str = "runtime::reentrancy-attack";


#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch, pallet_prelude::*};
	use frame_system::pallet_prelude::OriginFor;
	use frame_support::traits::Currency;
	use frame_system::ensure_signed;


	pub use frame_system::{
		pallet_prelude::BlockNumberFor, Config as SystemConfig, Pallet as SystemPallet,
	};

	type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as SystemConfig>::AccountId>>::Balance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The overarching call type.
		type RuntimeCall: Parameter
			+ Dispatchable<RuntimeOrigin = Self::RuntimeOrigin, PostInfo = PostDispatchInfo>
			+ GetDispatchInfo
			+ From<frame_system::Call<Self>>;

		/// The currency mechanism.
		type Currency: ReservableCurrency<Self::AccountId>;
	}

	/// The in-code storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::error]
	pub enum Error<T> {
		/// Threshold must be 2 or greater.
		MinimumThreshold,
		/// Call is already approved by this signatory.
		AlreadyApproved,
	}

	#[pallet::event]
	pub enum Event<T: Config> {
		AccountsCreated(T::AccountId, T::AccountId),
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000)]
		pub fn create_accounts(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResult {
			let mut SEED = [1u8; 32];
			let sender = ensure_signed(origin)?;

            let current_block = <frame_system::Pallet<T>>::block_number();
			// let account_1 = <frame_system::Pallet<T>>::account;

			// let multi = Multisig::multi_account_id(&[1, 2, 3][..], 2);

			// T::Currency::transfer(&sender, dest, value, existence_requirement);

			Ok(())
		}
	}
}
