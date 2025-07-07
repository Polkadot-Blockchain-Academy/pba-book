#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
  use codec::{Codec, FullEncode};
  use frame_support::{pallet_prelude::*};
  use frame_system::pallet_prelude::*;

  #[pallet::config]
  pub trait Config: frame_system::Config {
    type MinimumBalance: Get<u128>;
  }

  #[pallet::storage]
  pub type Balances<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, u128>;

  #[pallet::pallet]
  #[pallet::without_storage_info]
  pub struct Pallet<T>(PhantomData<T>);

  #[pallet::genesis_config]
  #[cfg_attr(feature = "std", derive(frame_support::DefaultNoBound))]
  pub struct GenesisConfig<T: Config> {
    pub balances: Vec<(T::AccountId, u128)>,
  }

  #[pallet::genesis_build]
  impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
    fn build(&self) {
      for (who, initial_balance) in &self.balances {
        assert!(*initial_balance > T::MinimumBalance::get(), "initial balance too low");
        Balances::<T>::insert(who, initial_balance);
      }
		}
	}

  #[pallet::call]
  impl<T: Config> Pallet<T> {}
}
