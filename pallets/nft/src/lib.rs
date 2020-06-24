//! # Unique Assets
//!
//! This pallet exposes capabilities for managing unique assets, also known as
//! non-fungible tokens (NFTs).
//!
//! - [`nft::Trait`](./trait.Trait.html)
//! - [`Call`](./enum.Call.html)
//! - [`Module`](./struct.Module.html)
//!
//! ## Overview
//!
//! This pallet allows an "asset admin" origin to control the creation and
//! distribution of unique assets that share a common metadata structure. There
//! is also a configuration parameter that is used to limit the number of
//! instances of a particular type of unique asset that any single account may
//! hold.
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * `mint_asset` - use the provided asset info to create a new unique asset
//!                  for the specified user; may only be called by asset admin

#![cfg_attr(not(feature = "std"), no_std)]

use codec::FullCodec;
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
    traits::{EnsureOrigin, Get},
    Hashable,
};
use frame_system::{self as system, ensure_signed};
use sp_runtime::traits::Member;
use sp_std::fmt::Debug;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait<I = DefaultInstance>: system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    type AssetAdmin: EnsureOrigin<Self::Origin>;
    type AssetInfo: Hashable + Member + Debug + Default + FullCodec;
    type AssetLimit: Get<u128>;
    type UserAssetLimit: Get<u64>;
}

pub type AssetId = [u8; 16];

decl_storage! {
    trait Store for Module<T: Trait<I>, I: Instance = DefaultInstance> as NFT {
        // The total number of this type of unique asset that is currently in existence
        TotalBalance get(fn total_balance): u128 = 0;
        // The total number of this type of unique asset that has been burned (may overflow)
        TotalBurned get(fn total_burned): u128 = 0;
        // The total number of this type of unique asset owned by an account
        BalanceForAccount get(fn balance_for_account): map hasher(blake2_128_concat) T::AccountId => u64 = 0;
        // Mapping from holder address to their (enumerable) set of owned assets
        AssetsForAccount get(fn assets_for_account): double_map hasher(blake2_128_concat) T::AccountId, hasher(identity) AssetId => T::AssetInfo;
        // Mapping from asset ID to the address that owns it
        AccountForAsset get(fn account_for_asset): map hasher(identity) AssetId => T::AccountId;
    }
}

decl_event!(
    pub enum Event<T, I = DefaultInstance>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        AssetBurned(AssetId),
        AssetMinted(AssetId, AccountId),
        AssetTransferred(AssetId, AccountId),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait<I>, I: Instance> {
        // The asset already exists
        AssetExists,
        // The user is not the asset owner
        NotAssetOwner,
        // There are too many assets
        TooManyAssets,
        // The user has too many assets
        TooManyAssetsForUser,
    }
}

// The pallet's dispatchable functions.
decl_module! {
    pub struct Module<T: Trait<I>, I: Instance = DefaultInstance> for enum Call where origin: T::Origin {
        type Error = Error<T, I>;
        fn deposit_event() = default;

        /// Create a new unique asset from the provided asset info and identify the specified
        /// account as its owner.
        ///
        /// The dispatch origin for this call must be the asset admin.
        ///
        /// This function will throw an error if it is called with asset info that describes
        /// an existing (duplicate) asset, or if the specified owner already has the maximum
        /// allowed number of this type of unique asset.
        ///
        /// - `owner_account`: Receiver of the asset.
        /// - `asset_info`: The information that defines the asset.
        #[weight = 10_000]
        pub fn mint_asset(origin, owner_account: T::AccountId, asset_info: T::AssetInfo) -> dispatch::DispatchResult {
            T::AssetAdmin::ensure_origin(origin)?;

            let asset_id = asset_info.blake2_128();

            ensure!(!AccountForAsset::<T, I>::contains_key(&asset_id), Error::<T, I>::AssetExists);
            ensure!(Self::total_balance() <= T::AssetLimit::get(), Error::<T, I>::TooManyAssets);
            ensure!(Self::balance_for_account(&owner_account) <= T::UserAssetLimit::get(), Error::<T, I>::TooManyAssetsForUser);

            TotalBalance::<I>::mutate(|balance| *balance += 1);
            BalanceForAccount::<T, I>::mutate(&owner_account, |balance| *balance += 1);
            AssetsForAccount::<T, I>::insert(&owner_account, &asset_id, asset_info);
            AccountForAsset::<T, I>::insert(&asset_id, &owner_account);

            Self::deposit_event(RawEvent::AssetMinted(asset_id, owner_account));
            Ok(())
        }

        #[weight = 10_000]
        pub fn burn_asset(origin, asset_id: AssetId) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(who == Self::account_for_asset(&asset_id), Error::<T, I>::NotAssetOwner);

            TotalBalance::<I>::mutate(|balance| *balance -= 1);
            TotalBurned::<I>::mutate(|balance| *balance += 1);
            BalanceForAccount::<T, I>::mutate(&who, |balance| *balance -= 1);
            AssetsForAccount::<T, I>::remove(&who, &asset_id);
            AccountForAsset::<T, I>::remove(&asset_id);

            Self::deposit_event(RawEvent::AssetBurned(asset_id));
            Ok(())
        }

        #[weight = 10_000]
        pub fn transfer_asset(origin, dest_account: T::AccountId, asset_id: AssetId) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(who == Self::account_for_asset(&asset_id), Error::<T, I>::NotAssetOwner);
            ensure!(Self::balance_for_account(&dest_account) <= T::UserAssetLimit::get(), Error::<T, I>::TooManyAssetsForUser);

            BalanceForAccount::<T, I>::mutate(&who, |balance| *balance -= 1);
            BalanceForAccount::<T, I>::mutate(&dest_account, |balance| *balance += 1);
            let asset_info = AssetsForAccount::<T, I>::take(who, &asset_id);
            AssetsForAccount::<T, I>::insert(&dest_account, &asset_id, asset_info);
            AccountForAsset::<T, I>::insert(&asset_id, &dest_account);

            Self::deposit_event(RawEvent::AssetTransferred(asset_id, dest_account));
            Ok(())
        }
    }
}
