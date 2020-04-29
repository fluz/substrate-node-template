#![cfg_attr(not(feature = "std"), no_std)]

/// Dedicated migrations pallet; initialize a collective that is added to a runtime as part of a forkless upgrade with
/// a set of members

use hex_literal::hex;

use frame_support::{decl_module, dispatch::Decode, weights::Weight};
use frame_system::{self as system};
use sp_runtime::traits::Dispatchable;

/// The pallet's configuration trait.
pub trait Trait: pallet_collective::Trait<pallet_collective::Instance1> + system::Trait { /* migration pallet */ }

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: <T as system::Trait>::Origin, T: pallet_collective::Trait<pallet_collective::Instance1> {
		fn on_runtime_upgrade() -> Weight {
			pallet_collective::Call::<T, pallet_collective::Instance1>::set_members([
				// Ferdie
				T::AccountId::decode(&mut &hex!["1cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c"][..]).unwrap(),
				// Dave
				T::AccountId::decode(&mut &hex!["306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc20"][..]).unwrap(),
				// Bob
				T::AccountId::decode(&mut &hex!["8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48"][..]).unwrap(),
				// Charlie
				T::AccountId::decode(&mut &hex!["90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22"][..]).unwrap(),
				// Alice
				T::AccountId::decode(&mut &hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"][..]).unwrap(),
				// Eve
				T::AccountId::decode(&mut &hex!["e659a7a1628cdd93febc04a4e0646ea20e9f5f0ce097d9a05290d4a9e054df4e"][..]).unwrap(),
			].to_vec(), None).dispatch(system::RawOrigin::Root.into()).expect("[init-council] set_members failed");
			0
		}
	}
}
