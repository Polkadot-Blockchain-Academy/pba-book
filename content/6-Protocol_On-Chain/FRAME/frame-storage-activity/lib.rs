#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type Item1<T> = StorageValue<_, u32>;

	#[pallet::storage]
	pub type Item2<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::type_value]
	pub fn MyDefault<T: Config>() -> u32 {
		42u32
	}

	#[pallet::storage]
	pub type Item3<T> = StorageValue<_, u32, ValueQuery, MyDefault<T>>;

	#[pallet::storage]
	pub type Item4<T> = StorageValue<_, Option<u32>>;

	#[pallet::storage]
	pub type Item5<T> = StorageValue<_, Option<u32>, ValueQuery>;

	#[pallet::storage]
	pub type Item6<T> = StorageValue<_, ()>;

	#[pallet::storage]
	#[pallet::unbounded]
	pub type Item7<T> = StorageValue<_, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	pub type Item8<T> = StorageValue<_, BoundedVec<u8, ConstU32<100>>, ValueQuery>;

	#[pallet::storage]
	pub type Item9<T: Config> = StorageMap<_, Blake2_128, u32, u32>;

	type Balance = u128;

	#[pallet::storage]
	pub type Item10<T: Config> = StorageMap<_, Blake2_128, T::AccountId, Balance, ValueQuery>;

	#[pallet::storage]
	pub type Item11<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Balance, ValueQuery>;

	#[pallet::storage]
	pub type Item12<T: Config> = StorageNMap<
		_,
		(
			NMapKey<Blake2_128Concat, u8>,
			NMapKey<Blake2_128Concat, u16>,
			NMapKey<Blake2_128Concat, u32>,
		),
		u128,
	>;

	#[pallet::storage]
	pub type Item13<T: Config> =
		CountedStorageMap<_, Blake2_128Concat, T::AccountId, Balance, ValueQuery>;
}
