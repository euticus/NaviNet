//! # Access Gate Pallet
//!
//! A pallet for managing stake-to-unlock access to network resources.
//!
//! ## Overview
//!
//! This pallet allows networks to gate resources by requiring users to stake
//! tokens for access, with support for tiered pricing and pay-per-use models.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

/// Pricing tier for resource access
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Tier {
    /// Name of the tier
    pub name: BoundedVec<u8, ConstU32<64>>,
    /// Amount to stake for this tier
    pub stake: u128,
    /// Duration in blocks for this tier
    pub duration_blocks: u32,
}

/// Pricing configuration for a resource
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Pricing {
    /// Base stake amount
    pub base_stake: u128,
    /// Base duration in blocks
    pub duration_blocks: u32,
    /// Optional tiered pricing
    pub tiers: BoundedVec<Tier, ConstU32<10>>,
    /// Optional pay-per-use amount
    pub ppu: Option<u128>,
}

/// Resource information
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Resource {
    /// Content identifier
    pub cid: BoundedVec<u8, ConstU32<256>>,
    /// Resource kind/type
    pub kind: u8,
    /// Pricing configuration
    pub pricing: Pricing,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_network_factory::Config {
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    /// Membership information for an account in a network
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Membership<T: Config> {
        /// Network ID
        pub network_id: u64,
        /// Account ID
        pub who: T::AccountId,
        /// Tier index (None for base tier)
        pub tier_idx: Option<u32>,
        /// Block number when membership expires
        pub expires_at: BlockNumberFor<T>,
    }

    /// Storage for resources by network and resource ID
    #[pallet::storage]
    pub type Resources<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        u64, // NetworkId
        Blake2_128Concat,
        u64, // ResourceId
        Resource,
    >;

    /// Next resource ID for each network
    #[pallet::storage]
    pub type NextResourceId<T: Config> = StorageMap<_, Blake2_128Concat, u64, u64, ValueQuery>;

    /// Memberships by network and account
    #[pallet::storage]
    pub type Memberships<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        u64, // NetworkId
        Blake2_128Concat,
        T::AccountId,
        Membership<T>,
    >;

    /// Events for the access gate pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A resource was registered
        ResourceRegistered { network_id: u64, resource_id: u64 },
        /// Access was granted to a resource
        AccessGranted {
            network_id: u64,
            who: T::AccountId,
            tier_idx: Option<u32>,
            expires_at: BlockNumberFor<T>,
        },
        /// Access expired for an account
        AccessExpired { network_id: u64, who: T::AccountId },
        /// Pay-per-use payment was made
        PayPerUsePaid {
            network_id: u64,
            who: T::AccountId,
            resource_id: u64,
            amount: u128,
        },
    }

    /// Errors for the access gate pallet.
    #[pallet::error]
    pub enum Error<T> {
        /// CID is too long
        CidTooLong,
        /// Tier name is too long
        TierNameTooLong,
        /// Too many tiers
        TooManyTiers,
        /// Resource not found
        ResourceNotFound,
        /// Network not found
        NetworkNotFound,
        /// Invalid tier index
        InvalidTierIndex,
        /// Membership not found
        MembershipNotFound,
        /// Membership not expired
        MembershipNotExpired,
        /// Resource ID overflow
        ResourceIdOverflow,
        /// Pay-per-use not enabled
        PayPerUseNotEnabled,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register a new resource for a network
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn register_resource(
            origin: OriginFor<T>,
            network_id: u64,
            cid: Vec<u8>,
            kind: u8,
            base_stake: u128,
            duration_blocks: u32,
            ppu: Option<u128>,
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            // TODO: Check that caller is network owner

            let bounded_cid = BoundedVec::try_from(cid).map_err(|_| Error::<T>::CidTooLong)?;

            let pricing = Pricing {
                base_stake,
                duration_blocks,
                tiers: BoundedVec::try_from(vec![]).unwrap(),
                ppu,
            };

            let resource = Resource {
                cid: bounded_cid,
                kind,
                pricing,
            };

            let resource_id = NextResourceId::<T>::get(network_id);
            let next_id = resource_id
                .checked_add(1)
                .ok_or(Error::<T>::ResourceIdOverflow)?;

            Resources::<T>::insert(network_id, resource_id, resource);
            NextResourceId::<T>::insert(network_id, next_id);

            Self::deposit_event(Event::ResourceRegistered {
                network_id,
                resource_id,
            });

            Ok(())
        }

        /// Stake for access to a network's resources
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn stake_for_access(
            origin: OriginFor<T>,
            network_id: u64,
            _resource_id: u64,
            tier_idx: Option<u32>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Verify network exists
            let _network = pallet_network_factory::Networks::<T>::get(network_id)
                .ok_or(Error::<T>::NetworkNotFound)?;

            // TODO: Lock tokens based on network coin type and tier

            let current_block = frame_system::Pallet::<T>::block_number();
            let duration = 100u32; // Placeholder duration
            let expires_at = current_block + duration.into();

            let membership = Membership {
                network_id,
                who: who.clone(),
                tier_idx,
                expires_at,
            };

            Memberships::<T>::insert(network_id, &who, membership);

            Self::deposit_event(Event::AccessGranted {
                network_id,
                who,
                tier_idx,
                expires_at,
            });

            Ok(())
        }

        /// Unstake if membership has expired
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn unstake_if_expired(
            origin: OriginFor<T>,
            network_id: u64,
            account: T::AccountId,
        ) -> DispatchResult {
            let _caller = ensure_signed(origin)?;

            let membership = Memberships::<T>::get(network_id, &account)
                .ok_or(Error::<T>::MembershipNotFound)?;

            let current_block = frame_system::Pallet::<T>::block_number();
            ensure!(
                current_block >= membership.expires_at,
                Error::<T>::MembershipNotExpired
            );

            // TODO: Unlock tokens

            Memberships::<T>::remove(network_id, &account);

            Self::deposit_event(Event::AccessExpired {
                network_id,
                who: account,
            });

            Ok(())
        }

        /// Pay per use for a resource
        #[pallet::call_index(3)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn pay_per_use(
            origin: OriginFor<T>,
            network_id: u64,
            resource_id: u64,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let resource =
                Resources::<T>::get(network_id, resource_id).ok_or(Error::<T>::ResourceNotFound)?;

            let ppu_amount = resource
                .pricing
                .ppu
                .ok_or(Error::<T>::PayPerUseNotEnabled)?;

            // TODO: Transfer tokens

            Self::deposit_event(Event::PayPerUsePaid {
                network_id,
                who,
                resource_id,
                amount: ppu_amount,
            });

            Ok(())
        }
    }
}
