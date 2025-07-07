---
title: Outer Enum
description: FRAME Outer Enum Web3 Engineers.
duration: 1 hour
---

# Outer Enum

([Reference in `polkadot-sdk-docs`](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_runtime_types/index.html#composite-enums))

---

## Outer Enum

In this presentation, you will learn about a common pattern used throughout FRAME, which abstracts many separate types into a single unified type that is used by the Runtime.

This is also known as "aggregate" types.

---

## Enums in FRAME

There are 4 main Enums which you will encounter throughout your FRAME development:

- The Call Enum
- The Event Enum
- The Error Enum
- The Origin Enum

All of these enums have some representation within individual pallets, but also the final FRAME runtime you develop.

---

### Breaking It Down (Without Substrate)

```rust [0|10-30|32-52|54-80|82-97|99|102-107|109-114|116-140]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use parity_scale_codec::Encode;

pub type AccountId = u16;
pub type Balance = u32;
pub type Hash = [u8; 32];

mod balances {
	use crate::*;

	#[derive(Encode)]
	pub enum Call {
		transfer { to: AccountId, amount: Balance },
		transfer_all { to: AccountId },
	}

	#[derive(Encode)]
	pub enum Error {
		InsufficientBalance,
		ExistentialDeposit,
		KeepAlive,
	}

	#[derive(Encode)]
	pub enum Event {
		Transfer { from: AccountId, to: AccountId, amount: Balance },
	}
}

mod democracy {
	use crate::*;

	#[derive(Encode)]
	pub enum Call {
		propose { proposal_hash: Hash },
		vote { proposal_id: u32, aye: bool },
	}

	#[derive(Encode)]
	pub enum Error {
		DuplicateProposal,
	}

	#[derive(Encode)]
	pub enum Event {
		Proposed { proposal_index: Hash },
		Passed { proposal_index: Hash },
		NotPassed { proposal_index: Hash },
	}
}

mod staking {
	use crate::*;

	#[derive(Encode)]
	pub enum Call {
		unstake,
		stake { nominate: Vec<AccountId>, amount: Balance },
	}

	#[derive(Encode)]
	pub enum Error {
		TooManyTargets,
		EmptyTargets,
		AlreadyBonded,
	}

	impl Into<DispatchError> for Error {
		fn into(self) -> DispatchError {
			DispatchError::Module(
				ModuleError {
					pallet: runtime::Runtime::Staking as u8,
					error: self as u8,
				}
			)
		}
	}
}

// Similar to `sp-runtime`
mod runtime_primitives {
	use crate::*;

	#[derive(Encode)]
	pub struct ModuleError {
		pub pallet: u8,
		pub error: u8,
	}

	#[derive(Encode)]
	pub enum DispatchError {
		BadOrigin,
		Module(ModuleError),
	}
}

mod runtime {
	use crate::*;

	#[derive(Encode)]
	pub enum PalletIndex {
		Balances = 0,
		Democracy = 1,
		Staking = 2,
	}

	#[derive(Encode)]
	pub enum RuntimeCall {
		BalancesCall(balances::Call),
		DemocracyCall(democracy::Call),
		StakingCall(staking::Call),
	}

	#[derive(Encode)]
	pub enum RuntimeEvent {
		BalancesEvent(balances::Event),
		DemocracyEvent(democracy::Event),
		// No staking events... not even in the enum.
	}

	// Imagine this for all of the possible types above...
	impl Into<RuntimeEvent> for balances::Event {
		fn into(self) -> RuntimeEvent {
			RuntimeEvent::BalancesEvent(self)
		}
	}

	// Imagine this for all of the possible types above...
	impl TryFrom<RuntimeEvent> for balances::Event {
		type Error = ();

		fn try_from(outer: RuntimeEvent) -> Result<Self, ()> {
			match outer {
				Event::BalancesEvent(event) => Ok(event),
				_ => Err(())
			}
		}
	}
}

use runtime_primitives::*;

fn main() {
	let democracy_call = democracy::Call::propose { proposal_hash: [7u8; 32] };
	println!("Pallet Call:   {:?}", democracy_call.encode());
	let runtime_call = runtime::RuntimeCall::Democracy(democracy_call);
	println!("Runtime Call:  {:?}", runtime_call.encode());
	let staking_error = staking::Error::AlreadyBonded;
	println!("Pallet Error:  {:?}", staking_error.encode());
	let runtime_error: DispatchError = staking_error.into();
	println!("Runtime Error: {:?}", runtime_error.encode());
	let balances_event = balances::Event::Transfer { from: 1, to: 2, amount: 3 };
	println!("Pallet Event:  {:?}", balances_event.encode());
	let runtime_event: runtime::RuntimeEvent = balances_event.into();
	println!("Runtime Event: {:?}", runtime_event.encode());
}
```

---

## Outer Enum Encoding

This now explains how all the different runtime types are generally encoded!

```rust [2,4,6,8,10,12]
fn main() {
	let democracy_call = democracy::Call::propose { proposal_hash: [7u8; 32] };
	println!("Pallet Call:   {:?}", democracy_call.encode());
	let runtime_call = runtime::RuntimeCall::Democracy(democracy_call);
	println!("Runtime Call:  {:?}", runtime_call.encode());
	let staking_error = staking::Error::AlreadyBonded;
	println!("Pallet Error:  {:?}", staking_error.encode());
	let runtime_error: DispatchError = staking_error.into();
	println!("Runtime Error: {:?}", runtime_error.encode());
	let balances_event = balances::Event::Transfer { from: 1, to: 2, amount: 3 };
	println!("Pallet Event:  {:?}", balances_event.encode());
	let runtime_event: runtime::RuntimeEvent = balances_event.into();
	println!("Runtime Event: {:?}", runtime_event.encode());
}
```

```sh
Pallet Call:   [0, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7]
Runtime Call:  [1, 0, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7]
Pallet Error:  [2]
Runtime Error: [1, 2, 2]
Pallet Event:  [0, 1, 0, 2, 0, 3, 0, 0, 0]
Runtime Event: [0, 0, 1, 0, 2, 0, 3, 0, 0, 0]
```

---

## Real Runtime

This was directly added to `substrate/bin/node-template/runtime/src/lib.rs`:

```rust [7,9,11,13,15,17]
#[test]
fn outer_enum_tests() {
	use sp_runtime::{DispatchError, MultiAddress};
	use sp_core::crypto::AccountId32;
	use codec::Encode;

	let balances_call = pallet_balances::Call::<Runtime>::transfer { dest: MultiAddress::Address32([1u8; 32]), value: 12345 };
	println!("Pallet Call:   {:?}", balances_call.encode());
	let runtime_call = crate::RuntimeCall::Balances(balances_call);
	println!("Runtime Call:  {:?}", runtime_call.encode());
	let balances_error = pallet_balances::Error::<Runtime>::InsufficientBalance;
	println!("Pallet Error:  {:?}", balances_error.encode());
	let runtime_error: DispatchError = balances_error.into();
	println!("Runtime Error: {:?}", runtime_error.encode());
	let balances_event = pallet_balances::Event::<Runtime>::Transfer { from: AccountId32::new([2u8; 32]), to: AccountId32::new([3u8; 32]), amount: 12345 };
	println!("Pallet Event:  {:?}", balances_event.encode());
	let runtime_event: crate::RuntimeEvent = balances_event.into();
	println!("Runtime Event: {:?}", runtime_event.encode());
}
```

---

## Real Runtime Output

```sh
Pallet Call:   [0, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 229, 192]
Runtime Call:  [5, 0, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 229, 192]
Pallet Error:  [2]
Runtime Error: [3, 5, 2, 0, 0, 0]
Pallet Event:  [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 57, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
Runtime Event: [5, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 57, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
```

Everything is just like our FRAME-less mock, but the types are more complex.

---

## Using Outer Enums

The path for using outer enums can be a bit confusing.

- The types which compose the outer enum come from pallets.

- They are aggregated in the runtime.

- They can be passed BACK to the pallets and used in pallet logic through associated types.

---

### System Aggregated Associated Types

You can see these "aggregate" types are associated types in FRAME System.

```rust
/// System configuration trait. Implemented by runtime.
#[pallet::config]
#[pallet::disable_frame_system_supertrait_check]
pub trait Config: 'static + Eq + Clone {
	/// The `RuntimeOrigin` type used by dispatchable calls.
	type RuntimeOrigin: Into<Result<RawOrigin<Self::AccountId>, Self::RuntimeOrigin>> + From<RawOrigin<Self::AccountId>> + Clone + OriginTrait<Call = Self::RuntimeCall>;

	/// The aggregated `RuntimeCall` type.
	type RuntimeCall: Parameter + Dispatchable<RuntimeOrigin = Self::RuntimeOrigin> + Debug + From<Call<Self>>;

	/// The aggregated event type of the runtime.
	type RuntimeEvent: Parameter + Member + From<Event<Self>> + Debug + IsType<<Self as frame_system::Config>::RuntimeEvent>;

	// -- snip --
}
```

---

## Pallet Event

You can now see why we need to add an `Event` associated type to each pallet which uses events:

```rust
/// Configure the pallet by specifying the parameters and types on which it depends.
#[pallet::config]
pub trait Config: frame_system::Config {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
}
```

---

<!-- .slide: data-background-color="#4A2439" -->

# Questions
