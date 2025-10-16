//! # Service Basket Pallet
//!
//! A pallet for managing the mix of external services per network.
//!
//! ## Overview
//!
//! This pallet allows networks to configure weighted baskets of external
//! services (ID, storage, compute) with proofs and indices.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use codec::{Decode, Encode, MaxEncodedLen};
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use scale_info::TypeInfo;
    use sp_runtime::{FixedU128, Permill, RuntimeDebug};

    /// Service key identifier (e.g., "ID", "FILE", "GOLEM")
    pub type ServiceKey = BoundedVec<u8, ConstU32<32>>;

    /// Service weight type for allocation (renamed to avoid conflict with FRAME Weight)
    pub type ServiceWeight = Permill;

    /// Service basket configuration for a network
    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct Basket {
        /// Weighted allocation of services (sum must be <= 1.0)
        pub weights: BoundedVec<(ServiceKey, ServiceWeight), ConstU32<10>>,
        /// Proof CIDs for service verification
        pub proofs: BoundedVec<BoundedVec<u8, ConstU32<256>>, ConstU32<100>>,
        /// Computed index value (deterministic placeholder)
        pub index: FixedU128,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    /// Storage for baskets per network
    #[pallet::storage]
    pub type Baskets<T: Config> = StorageMap<_, Blake2_128Concat, u64, Basket>;

    /// Events for the service basket pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Service weights were updated.
        WeightsUpdated {
            /// The network ID.
            network_id: u64,
        },
        /// Proof was added to basket.
        ProofAdded {
            /// The network ID.
            network_id: u64,
        },
        /// Index was recomputed.
        IndexRecomputed {
            /// The network ID.
            network_id: u64,
            /// The new index value.
            index: FixedU128,
        },
    }

    /// Errors for the service basket pallet.
    #[pallet::error]
    pub enum Error<T> {
        /// The sum of weights exceeds 1.0.
        WeightSumExceedsOne,
        /// Basket not found for the given network.
        BasketNotFound,
        /// Too many weights provided.
        TooManyWeights,
        /// Too many proofs stored.
        TooManyProofs,
        /// Service key is too long.
        ServiceKeyTooLong,
        /// Proof CID is too long.
        ProofCidTooLong,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Set service weights for a network.
        ///
        /// The sum of all weights must be <= 1.0 (100%).
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn set_weights(
            origin: OriginFor<T>,
            network_id: u64,
            weights: Vec<(Vec<u8>, u32)>,
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            // Convert Vec<(Vec<u8>, u32)> to BoundedVec<(ServiceKey, Weight), _>
            let mut bounded_weights = BoundedVec::new();
            let mut sum = 0u32;

            for (key, weight_parts) in weights {
                let service_key =
                    BoundedVec::try_from(key).map_err(|_| Error::<T>::ServiceKeyTooLong)?;
                let weight = Permill::from_parts(weight_parts);
                bounded_weights
                    .try_push((service_key, weight))
                    .map_err(|_| Error::<T>::TooManyWeights)?;
                sum = sum.saturating_add(weight_parts);
            }

            // Ensure sum <= 1.0 (1_000_000 parts)
            ensure!(sum <= 1_000_000, Error::<T>::WeightSumExceedsOne);

            // Get or create basket
            let mut basket = Baskets::<T>::get(network_id).unwrap_or(Basket {
                weights: BoundedVec::new(),
                proofs: BoundedVec::new(),
                index: FixedU128::from(0),
            });

            basket.weights = bounded_weights;
            Baskets::<T>::insert(network_id, basket);

            Self::deposit_event(Event::WeightsUpdated { network_id });

            Ok(())
        }

        /// Add a proof CID to the basket.
        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn add_proof(origin: OriginFor<T>, network_id: u64, cid: Vec<u8>) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            let bounded_cid = BoundedVec::try_from(cid).map_err(|_| Error::<T>::ProofCidTooLong)?;

            let mut basket = Baskets::<T>::get(network_id).ok_or(Error::<T>::BasketNotFound)?;

            basket
                .proofs
                .try_push(bounded_cid)
                .map_err(|_| Error::<T>::TooManyProofs)?;

            Baskets::<T>::insert(network_id, basket);

            Self::deposit_event(Event::ProofAdded { network_id });

            Ok(())
        }

        /// Recompute the index for a basket.
        ///
        /// This is a deterministic placeholder: sum(weights_i * 1.0)
        /// Later this will use oracle feeds.
        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn recompute_index(origin: OriginFor<T>, network_id: u64) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            let mut basket = Baskets::<T>::get(network_id).ok_or(Error::<T>::BasketNotFound)?;

            // Placeholder computation: sum of all weights
            let mut sum = 0u32;
            for (_key, weight) in basket.weights.iter() {
                sum = sum.saturating_add(weight.deconstruct());
            }

            // Convert to FixedU128 (parts per million to fixed point)
            let index = FixedU128::from_rational(sum as u128, 1_000_000);
            basket.index = index;

            Baskets::<T>::insert(network_id, basket);

            Self::deposit_event(Event::IndexRecomputed { network_id, index });

            Ok(())
        }
    }
}
