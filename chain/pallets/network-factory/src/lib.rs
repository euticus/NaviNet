//! # Network Factory Pallet
//!
//! A pallet for creating and managing user-owned networks on NaviNet.
//!
//! ## Overview
//!
//! This pallet allows users to instantiate their own networks with custom
//! configurations and coin types.

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
        },
    }

    /// Errors for the network factory pallet.
    #[pallet::error]
    pub enum Error<T> {
        /// Placeholder error.
        PlaceholderError,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new network (placeholder implementation).
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn create_network(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Placeholder: use network_id = 1
            let network_id = 1u64;

            // Emit event
            Self::deposit_event(Event::NetworkCreated {
                network_id,
                owner: who,
            });

            Ok(())
        }
    }
}
