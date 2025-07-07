#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
  use codec::{Encode, Decode};
  // TODO: maybe look into these preludes.
  use frame_support::{pallet_prelude::*, dispatch::{DispatchResultWithPostInfo, PostDispatchInfo}, weights::Pays};
  use frame_system::pallet_prelude::*;

  #[pallet::config]
  pub trait Config: frame_system::Config {}

  #[pallet::pallet]
  #[pallet::without_storage_info]
  pub struct Pallet<T>(PhantomData<T>);

  #[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, TypeInfo)]
  pub struct TransferConfig<AccountId> {
    from: AccountId,
    to: AccountId,
    amount: u64
  }

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    #[pallet::weight(42)]
    pub fn first_transaction(_origin: OriginFor<T>, _inc: u32) -> DispatchResult {
      Ok(())
    }

    #[pallet::weight(42)]
    pub fn transfer(_origin: OriginFor<T>, config: TransferConfig<T::AccountId>) -> DispatchResultWithPostInfo {
      if config.amount == 42 {
        Ok(PostDispatchInfo {
          actual_weight: Some(0),
          pays_fee: Pays::No,
        })
      } else {
        Ok(None.into())
      }
    }
  }
}
