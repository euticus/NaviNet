//! # Identity Pallet (NaviID)
//!
//! A pallet for managing decentralized identities on NaviNet.
//!
//! ## Overview
//!
//! This pallet provides the foundation for NaviID, allowing users to register
//! and manage their decentralized identities on the NaviNet chain.

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

    /// Events for the identity pallet.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// An identity was registered.
        IdentityRegistered {
            /// The account that registered the identity.
            who: T::AccountId,
        },
    }

    /// Errors for the identity pallet.
    #[pallet::error]
    pub enum Error<T> {
        /// Placeholder error.
        PlaceholderError,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Register a new identity (placeholder implementation).
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn register_identity(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Emit event
            Self::deposit_event(Event::IdentityRegistered { who });

            Ok(())
        }
    }
}
