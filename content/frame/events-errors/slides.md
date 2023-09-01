---
title: Events and Errors
description: FRAME Events and Errors for web3 builders.
duration: 1 hour
---

# Events and Errors

---

## Events and Errors

In this presentation, we will go over two of the tools you have access to when developing FRAME Pallets to express how your runtime calls are executing.

---

# Errors

---

## Intro to Errors

Not all extrinsics are valid. It could be for a number of reasons:

- The extrinsic itself is badly formatted. (wrong parameters, encoding, etc...)
- The state transition function does not allow it.
  - Maybe a timing problem.
  - User might be lacking resources.
  - State transition might be waiting for other data or processes.
  - etc...

---

## Dispatch Result

All pallet calls return at the end a `DispatchResult`.

From: `substrate/frame/support/src/dispatch.rs`

```rust
pub type DispatchResult = Result<(), sp_runtime::DispatchError>;
```

So a function can either return `Ok(())` or some `DispatchError`.

---

## Dispatch Error

From: `substrate/primitives/runtime/src/lib.rs`

```rust [0|6-10|13-14|15-16]
/// Reason why a dispatch call failed.
#[derive(Eq, Clone, Copy, Encode, Decode, Debug, TypeInfo, PartialEq, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DispatchError {
	/// Some error occurred.
	Other(
		#[codec(skip)]
		#[cfg_attr(feature = "serde", serde(skip_deserializing))]
		&'static str,
	),
	/// Failed to lookup some data.
	CannotLookup,
	/// A bad origin.
	BadOrigin,
	/// A custom error in a module.
	Module(ModuleError),
	/// At least one consumer is remaining so the account cannot be destroyed.
	ConsumerRemaining,
	/// There are no providers so the account cannot be created.
	NoProviders,
	/// There are too many consumers so the account cannot be created.
	TooManyConsumers,
	/// An error to do with tokens.
	Token(TokenError),
	/// An arithmetic error.
	Arithmetic(ArithmeticError),
	/// The number of transactional layers has been reached, or we are not in a transactional
	/// layer.
	Transactional(TransactionalError),
	/// Resources exhausted, e.g. attempt to read/write data which is too large to manipulate.
	Exhausted,
	/// The state is corrupt; this is generally not going to fix itself.
	Corruption,
	/// Some resource (e.g. a preimage) is unavailable right now. This might fix itself later.
	Unavailable,
	/// Root origin is not allowed.
	RootNotAllowed,
}
```

---

## Module Errors

From: `substrate/primitives/runtime/src/lib.rs`

```rust
/// The number of bytes of the module-specific `error` field defined in [`ModuleError`].
/// In FRAME, this is the maximum encoded size of a pallet error type.
pub const MAX_MODULE_ERROR_ENCODED_SIZE: usize = 4;

/// Reason why a pallet call failed.
#[derive(Eq, Clone, Copy, Encode, Decode, Debug, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ModuleError {
	/// Module index, matching the metadata module index.
	pub index: u8,
	/// Module specific error value.
	pub error: [u8; MAX_MODULE_ERROR_ENCODED_SIZE],
	/// Optional error message.
	#[codec(skip)]
	#[cfg_attr(feature = "serde", serde(skip_deserializing))]
	pub message: Option<&'static str>,
}
```

So an error is at most just 5 bytes.

---

## Declaring Errors

```rust [0|23-30|47-48]
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type CurrentOwner<T: Config> = StorageValue<_, T::AccountId>;

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// There is currently no owner set.
		NoOwner,
		/// The calling user is not authorized to make this call.
		NotAuthorized,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// The owner has been updated.
		OwnerChanged,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// This function allows the current owner to set a new owner.
		/// If there is no owner, this function will return an error.
		#[pallet::weight(u64::default())]
		#[pallet::call_index(0)]
		pub fn change_ownership(origin: OriginFor<T>, new: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let current_owner = CurrentOwner::<T>::get().ok_or(Error::<T>::NoOwner)?;
			ensure!(current_owner == who, Error::<T>::NotAuthorized);
			CurrentOwner::<T>::put(new);
			Self::deposit_event(Event::<T>::OwnerChanged);
			Ok(())
		}
	}
}
```

---

## Using Errors

When writing tests, you can use errors to make sure that your functions execute exactly as expected.

```rust
#[test]
fn errors_example() {
	new_test_ext().execute_with(|| {
		assert_noop!(TemplateModule::change_ownership(Origin::signed(1), 2), Error::<T>::NoOwner);
		CurrentOwner::<T>::put(1);
		assert_ok!(TemplateModule::change_ownership(Origin::signed(1), 2));
		assert_noop!(TemplateModule::change_ownership(Origin::signed(1), 2), Error::<T>::NotAuthorized);
	});
}
```

---

## Encoding Errors

All errors ultimately become a `DispatchError`, which is the final type returned by the runtime.

```rust
println!("{:?}", Error::<T>::NoOwner.encode());
println!("{:?}", Error::<T>::NotAuthorized.encode());
let dispatch_error1: DispatchError = Error::<T>::NoOwner.into();
let dispatch_error2: DispatchError = Error::<T>::NotAuthorized.into();
println!("{:?}", dispatch_error1.encode());
println!("{:?}", dispatch_error2.encode());
```

```sh
[0]
[1]
[3, 1, 0, 0, 0, 0]
[3, 1, 1, 0, 0, 0]
```

---

## Dispatch Error Encoding

<table>
<tr>
	<td>3</td>
	<td>1</td>
	<td>1</td>
	<td>0</td>
	<td>0</td>
	<td>0</td>
</tr>
<tr>
	<td>DispatchError::Module</td>
	<td>Pallet #2</td>
	<td>Error #2</td>
	<td>(unused)</td>
	<td>(unused)</td>
	<td>(unused)</td>
</tr>
</table>

Encoding based on configuration:

<div class="flex-container">
<div class="left" style="max-width: 700px;">

```rust
// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub struct Test {
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		TemplateModule: pallet_template,
	}
);
```

</div>
<div class="right" style="margin-left: 10px; max-width: 700px;">

```rust
// Errors inform users that something went wrong.
#[pallet::error]
pub enum Error<T> {
	/// There is currently no owner set.
	NoOwner,
	/// The calling user is not authorized to make this call.
	NotAuthorized,
}
```

</div>
</div>
---

## Nested Errors

Errors support up to 5 bytes, which allows you to create nested errors, or insert other minimal data with the `PalletError` derive macro.

```rust
#[derive(Encode, Decode, PalletError, TypeInfo)]
pub enum SubError {
	SubError1,
	SubError2,
	SubError3,
}

use frame_system::pallet::Error as SystemError;

// Errors inform users that something went wrong.
#[pallet::error]
pub enum Error<T> {
	/// There is currently no owner set.
	NoOwner,
	/// The calling user is not authorized to make this call.
	NotAuthorized,
	/// Errors coming from another place.
	SubError(SubError),
	/// Errors coming from another place.
	SystemError(SystemError<T>),
	/// Some Error with minimal data
	DataError(u16),
}
```

Notes:

<https://github.com/paritytech/substrate/pull/10242>

---

# Events

---

## Intro to Events

When an extrinsic completes successfully, there is often some metadata you would like to expose to the outside world about what exactly happened during the execution.

For example, there may be multiple different ways an extrinsic completes successfully, and you want the user to know what happened.

Or maybe there is some significant state transition that you know users

---

## Declaring and Emitting Events

```rust [10-15|32-37|50]
#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type CurrentOwner<T: Config> = StorageValue<_, T::AccountId>;

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// There is currently no owner set.
		NoOwner,
		/// The calling user is not authorized to make this call.
		NotAuthorized,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// The owner has been updated.
		OwnerChanged,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// This function allows the current owner to set a new owner.
		/// If there is no owner, this function will return an error.
		#[pallet::weight(u64::default())]
		#[pallet::call_index(0)]
		pub fn change_ownership(origin: OriginFor<T>, new: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let current_owner = CurrentOwner::<T>::get().ok_or(Error::<T>::NoOwner)?;
			ensure!(current_owner == who, Error::<T>::NotAuthorized);
			CurrentOwner::<T>::put(new);
			Self::deposit_event(Event::<T>::OwnerChanged);
			Ok(())
		}
	}
}
```

---

## Deposit Event

```rust
#[pallet::generate_deposit(pub(super) fn deposit_event)]
```

Simply generates:

```rust
impl<T: Config> Pallet<T> {
	pub(super) fn deposit_event(event: Event<T>) {
		let event = <<T as Config>::Event as From<Event<T>>>::from(event);
		let event =
			<<T as Config>::Event as Into<<T as frame_system::Config>::Event>>::into(event);
		<frame_system::Pallet<T>>::deposit_event(event)
	}
}
```

<br />

`frame/support/procedural/src/pallet/expand/event.rs`

---

## Deposit Event in System

Events are just a storage item in FRAME System.

`frame/system/src/lib.rs`

```rust
/// Events deposited for the current block.
///
/// N O T E: The item is unbound and should therefore never be read on chain.
/// It could otherwise inflate the PoV size of a block.
///
/// Events have a large in-memory size. Box the events to not go out-of-memory
/// just in case someone still reads them from within the runtime.
#[pallet::storage]
pub(super) type Events<T: Config> =
	StorageValue<_, Vec<Box<EventRecord<T::RuntimeEvent, T::Hash>>>, ValueQuery>;

/// The number of events in the `Events<T>` list.
#[pallet::storage]
#[pallet::getter(fn event_count)]
pub(super) type EventCount<T: Config> = StorageValue<_, EventIndex, ValueQuery>;
```

---

## Deposit Events in System

Depositing events ultimately just appends a new event to this storage.

`frame/system/src/lib.rs`

```rust [0|1-4|34|13-16]
/// Deposits an event into this block's event record.
pub fn deposit_event(event: impl Into<T::RuntimeEvent>) {
	Self::deposit_event_indexed(&[], event.into());
}

/// Deposits an event into this block's event record adding this event
/// to the corresponding topic indexes.
///
/// This will update storage entries that correspond to the specified topics.
/// It is expected that light-clients could subscribe to this topics.
pub fn deposit_event_indexed(topics: &[T::Hash], event: T::RuntimeEvent) {
	let block_number = Self::block_number();
	// Don't populate events on genesis.
	if block_number.is_zero() {
		return
	}

	let phase = ExecutionPhase::<T>::get().unwrap_or_default();
	let event = EventRecord { phase, event, topics: topics.to_vec() };

	// Index of the to be added event.
	let event_idx = {
		let old_event_count = EventCount::<T>::get();
		let new_event_count = match old_event_count.checked_add(1) {
			// We've reached the maximum number of events at this block, just
			// don't do anything and leave the event_count unaltered.
			None => return,
			Some(nc) => nc,
		};
		EventCount::<T>::put(new_event_count);
		old_event_count
	};

	Events::<T>::append(event);

	for topic in topics {
		<EventTopics<T>>::append(topic, &(block_number, event_idx));
	}
}
```

---

## You Cannot Read Events

- The events storage are an unbounded vector of individual events emitted by your pallets.
- If you ever read this storage, you will introduce the whole thing into your storage proof!
- Never write runtime logic which reads from or depends on events.
- Tests are okay.

---

## You Cannot Read Events

`frame/system/src/lib.rs`

```rust
/// Get the current events deposited by the runtime.
///
/// Should only be called if you know what you are doing and outside of the runtime block
/// execution else it can have a large impact on the PoV size of a block.
pub fn read_events_no_consensus(
) -> impl sp_std::iter::Iterator<Item = Box<EventRecord<T::RuntimeEvent, T::Hash>>> {
	Events::<T>::stream_iter()
}

/// Get the current events deposited by the runtime.
///
/// NOTE: This should only be used in tests. Reading events from the runtime can have a large
/// impact on the PoV size of a block. Users should use alternative and well bounded storage
/// items for any behavior like this.
///
/// NOTE: Events not registered at the genesis block and quietly omitted.
#[cfg(any(feature = "std", feature = "runtime-benchmarks", test))]
pub fn events() -> Vec<EventRecord<T::RuntimeEvent, T::Hash>> {
	debug_assert!(
		!Self::block_number().is_zero(),
		"events not registered at the genesis block"
	);
	// Dereferencing the events here is fine since we are not in the
	// memory-restricted runtime.
	Self::read_events_no_consensus().map(|e| *e).collect()
}
```

---

## Testing Events

Remember to set the block number to greater than zero!

Some tools in FRAME System for you:

`frame/system/src/lib.rs`

```rust
/// Set the block number to something in particular. Can be used as an alternative to
/// `initialize` for tests that don't need to bother with the other environment entries.
#[cfg(any(feature = "std", feature = "runtime-benchmarks", test))]
pub fn set_block_number(n: BlockNumberFor<T>) {
	<Number<T>>::put(n);
}

/// Assert the given `event` exists.
#[cfg(any(feature = "std", feature = "runtime-benchmarks", test))]
pub fn assert_has_event(event: T::RuntimeEvent) {
	assert!(Self::events().iter().any(|record| record.event == event))
}

/// Assert the last event equal to the given `event`.
#[cfg(any(feature = "std", feature = "runtime-benchmarks", test))]
pub fn assert_last_event(event: T::RuntimeEvent) {
	assert_eq!(Self::events().last().expect("events expected").event, event);
}
```

---

## Using Events in Tests

```rust
#[test]
fn events_example() {
	new_test_ext().execute_with(|| {
		frame_system::Pallet::<T>::set_block_number(1);
		CurrentOwner::<T>::put(1);
		assert_ok!(TemplateModule::change_ownership(Origin::signed(1), 2));
		assert_ok!(TemplateModule::change_ownership(Origin::signed(2), 3));
		assert_ok!(TemplateModule::change_ownership(Origin::signed(3), 4));

		let events = frame_system::Pallet::<T>::events();
		assert_eq!(events.len(), 3);
		frame_system::Pallet::<T>::assert_has_event(crate::Event::<T>::OwnerChanged { old: 1, new: 2}.into());
		frame_system::Pallet::<T>::assert_last_event(crate::Event::<T>::OwnerChanged { old: 3, new: 4}.into());
	});
}
```

Remember other pallets can deposit events too!

---

## Summary

- Events and Errors are two ways you can signal to users what is happening when they dispatch an extrinsic.
- Events usually signify some successful thing happening.
- Errors signify when something has gone bad (and all changes reverted).
- Both are accessible by the end user when they occur.
