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
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching runtime event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    /// Events for the service basket pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Service weights were updated.
        WeightsUpdated {
            /// The network ID.
            network_id: u64,
        },
    }

    /// Errors for the service basket pallet.
    #[pallet::error]
    pub enum Error<T> {
        /// Placeholder error.
        PlaceholderError,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Update service weights (placeholder implementation).
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn update_weights(origin: OriginFor<T>, network_id: u64) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            // Emit event
            Self::deposit_event(Event::WeightsUpdated { network_id });

            Ok(())
        }
    }
}
