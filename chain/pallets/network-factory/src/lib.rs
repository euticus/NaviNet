//! # Network Factory Pallet
//!
//! A pallet for creating and managing user-owned networks on NaviNet.
//!
//! ## Overview
//!
//! This pallet allows users to instantiate their own networks with custom
//! configurations and coin types. Each network can either use the native NAVI
//! token or mint its own custom asset.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use alloc::vec::Vec;

pub use pallet::*;

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

/// The type of coin a network uses.
#[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum CoinKind {
    /// Use the native NAVI token.
    UseNavi,
    /// Mint a custom asset for this network.
    MintAsset,
}

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::PalletId;
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::AccountIdConversion;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// The pallet ID for deriving treasury accounts.
    const PALLET_ID: PalletId = PalletId(*b"navifact");

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    /// Information about a network.
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct NetworkInfo<T: Config> {
        /// The owner of the network.
        pub owner: T::AccountId,
        /// The type of coin this network uses.
        pub coin_kind: CoinKind,
        /// The asset ID if using MintAsset (u32 for compatibility with pallet_assets).
        pub asset_id: Option<u32>,
        /// The treasury account for this network.
        pub treasury: T::AccountId,
        /// Metadata URI for the network.
        pub metadata_uri: BoundedVec<u8, ConstU32<256>>,
    }

    /// Storage for the next network ID.
    #[pallet::storage]
    #[pallet::getter(fn next_network_id)]
    pub type NextNetworkId<T> = StorageValue<_, u64, ValueQuery>;

    /// Storage for network information.
    #[pallet::storage]
    #[pallet::getter(fn networks)]
    pub type Networks<T: Config> = StorageMap<_, Blake2_128Concat, u64, NetworkInfo<T>>;

    /// Events for the network factory pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A network was created.
        NetworkCreated {
            /// The network ID.
            network_id: u64,
            /// The owner of the network.
            owner: T::AccountId,
            /// Whether the network uses NAVI (true) or a custom asset (false).
            use_navi: bool,
            /// The asset ID if using a custom asset (u32 for compatibility with pallet_assets).
            asset_id: Option<u32>,
        },
    }

    /// Errors for the network factory pallet.
    #[pallet::error]
    pub enum Error<T> {
        /// Network ID overflow.
        NetworkIdOverflow,
        /// Metadata URI too long.
        MetadataUriTooLong,
        /// Failed to create asset.
        AssetCreationFailed,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new network.
        ///
        /// # Parameters
        /// - `use_navi`: If true, use NAVI token; if false, mint a new asset.
        /// - `metadata_uri`: Metadata URI for the network (max 256 bytes).
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn create_network(
            origin: OriginFor<T>,
            use_navi: bool,
            metadata_uri: Vec<u8>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Get next network ID
            let network_id = NextNetworkId::<T>::get();
            let next_id = network_id
                .checked_add(1)
                .ok_or(Error::<T>::NetworkIdOverflow)?;

            // Convert metadata_uri to BoundedVec
            let bounded_uri: BoundedVec<u8, ConstU32<256>> = metadata_uri
                .try_into()
                .map_err(|_| Error::<T>::MetadataUriTooLong)?;

            // Derive treasury account
            let treasury = Self::treasury_account(network_id);

            // Determine coin kind and asset ID
            let (coin_kind, asset_id) = if use_navi {
                (CoinKind::UseNavi, None)
            } else {
                // Create a new asset
                let asset_id = network_id as u32;

                // Note: In a real implementation, we would call pallet_assets::create here
                // For now, we'll just record the asset_id
                (CoinKind::MintAsset, Some(asset_id))
            };

            // Store network info
            let network_info = NetworkInfo {
                owner: who.clone(),
                coin_kind,
                asset_id,
                treasury,
                metadata_uri: bounded_uri,
            };
            Networks::<T>::insert(network_id, network_info);

            // Update next network ID
            NextNetworkId::<T>::put(next_id);

            // Emit event
            Self::deposit_event(Event::NetworkCreated {
                network_id,
                owner: who,
                use_navi,
                asset_id,
            });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Derive the treasury account for a network.
        pub fn treasury_account(network_id: u64) -> T::AccountId {
            PALLET_ID.into_sub_account_truncating(network_id)
        }
    }
}
