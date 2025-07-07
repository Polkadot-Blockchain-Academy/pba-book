---
title: Introduction to FRAME
description: An introduction into FRAME, a framework for building Substrate runtimes.
duration: 1 hour
---

<!-- ## Lesson Plan

<table class="no-bullet-padding">
<tr>
  <td>Monday</td>
  <td>Tuesday</td>
  <td>Wednesday</td>
  <td>Thursday</td>
  <td>Friday</td>
  <td>Weekend</td>
</tr>
<tr class="text-small">
<td>

- Introduction to FRAME
- 👩‍💻 Exercise: Proof of Existence Runtime
- Assignment 3 Feedback
- FRAME Tips and Tricks
- Announce FRAME Assignment

</td>
<td>

- Pallet Coupling
- FRAME Common Knowledge (Pallets & Traits)
- FRAME Storage
- Live Coding

</td>
<td>

- Events & Errors
- Calls
- Origins
- Outer Enum
- Hooks
- Live Coding

</td>
<td>

- Construct Runtime + Tests
- 👨‍💻 Exercise: Tests
- FRAME Benchmarking
- 👨🏾‍💻 Exercise: Benchmarking
- Live Coding

</td>
<td>

- FRAME Under the Hood
  - Deep Dive
  - Executive
- Signed Extensions
- Migrations & Try Runtime
- Frame Updates
- Live Coding

</td>
<td>

- Complete FRAME Assignment

</td>
</tr>
</table> -->

# Introduction to FRAME

---

## What is FRAME?

FRAME is a Rust framework for more easily building Substrate runtimes.

---

## Explaining FRAME Concisely

<pba-flex center>

- Writing the Sudo Pallet:
- Without FRAME: up to 3268 lines of code.
- With FRAME: 318 lines of code.
- ~10x Smaller.

</pba-flex>

Notes:
Without FRAME number is based on expanded FRAME-based code.
A fair comparison would be a frameless sudo pallet that might be shorter (but potentially less featurefull).

---

## Goals of FRAME

- Make it easy and concise for developers to do development.
- Provide maximum flexibility and compatibility for pallet developers.
- Provide maximum modularity for runtime developers.
- Be as similar to vanilla Rust as possible.

---

## Building Blocks of FRAME

- FRAME Development
  - Pallets
  - Macros
- FRAME Coordination
  - FRAME System
  - FRAME Executive
  - Construct Runtime

---

## Pallets

FRAME takes the opinion that the blockchain runtime should be composed of individual modules. We call these Pallets.

<img style="height: 600px" src="../../../../assets/img/6-FRAME/frame1.svg" />

---

## Building Blocks of Pallets

Pallets are composed of multiple parts common for runtime development:

- Dispatchable extrinsics
- Storage items
- Hooks for:
  - Block initialization,
  - Finalizing block (_!= block finality i.e. GRANDPA_)

---

## More Building Blocks of Pallets

And some less important ones:

- Events
- Errors
- Custom validation/communication with tx-pool
- Offchain workers
- A lot more! but you will learn about them later.

---

### "Shell" Pallet

```rust
#[frame_support::pallet]
pub mod pallet {
  #[pallet::pallet]
  pub struct Pallet<T>(_);

  #[pallet::config]
  pub trait Config: frame_system::Config { ... }

  #[pallet::event]
  pub enum Event { .. }

  #[pallet::error]
  pub enum Error { .. }

  #[pallet::storage]
  // snip

  #[pallet::call]
  impl Pallet { .. }
}
```

---

## FRAME Macros

Rust allows you to write Macros, which is code that generates code.

FRAME uses Macros to simplify the development of Pallets, while keeping all of the benefits of using Rust.

We will look more closely at each attribute throughout this module.

---

## See For Yourself

- `tokei -f` will show the number of lines of a file.
- `cargo expand` will expand the macros to "pure" Rust.

```sh
➜  polkadot-sdk git:(master) ✗ tokei -f substrate/frame/sudo/src/{lib.rs,extension.rs,weights.rs}
    Total 318

➜  polkadot-sdk git:(master) ✗ cargo expand -p pallet-sudo > sudo.rs; tokei -f sudo.rs
    Total 3268
```

---

## FRAME System

The FRAME System is a Pallet which is assumed to always exist when using FRAME. You can see that in the `Config` of every Pallet:

```rust
#[pallet::config]
pub trait Config: frame_system::Config { ... }
```

---

## FRAME System

It contains all the most basic functions and types needed for a blockchain system. Also contains many low level extrinsics to manage your chain directly.

<div class="flex-container">
<div class="left-small">

- Block Number
- Accounts
- Hash
- etc...

</div>
<div class="right text-small">

- `BlockNumberFor<T>`
- `frame_system::Pallet::<T>::block_number()`
- `T::AccountId`
- `T::Hash`
- `T::Hashing::hash(&bytes)`

</div>
</div>

---

## FRAME Executive

The FRAME Executive is a "coordinator", defining the order that your FRAME based runtime executes.

```rust
/// Actually execute all transitions for `block`.
pub fn execute_block(block: Block) { ... }
```

- Initialize Block
  - `on_runtime_upgrade` and `on_initialize` hooks
- Initial Checks
- Signature Verification
- Execute Extrinsics
- `on_idle` and `on_finalize` hooks
- Final Checks

---

## Construct Runtime

Your final runtime is composed of Pallets, which are brought together with the `construct_runtime!` macro.

```rust
// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime! {
	pub struct Runtime {
		System: frame_system,

		Timestamp: pallet_timestamp,
		Balances: pallet_balances,
		TransactionPayment: pallet_transaction_payment,
		Sudo: pallet_sudo,
	}
};
```

---

## Construct Runtime

New syntax

```rust
// Composes the runtime by adding all the used pallets and deriving necessary types.
#[runtime]
mod runtime {
  /// The main runtime type.
  #[runtime::runtime]
  #[runtime::derive(RuntimeCall, RuntimeEvent, RuntimeError, RuntimeOrigin, RuntimeTask)]
  pub struct Runtime;

  pub type System = frame_system;
  pub type Timestamp = pallet_timestamp;
  pub type Balances = pallet_balances;
  pub type TransactionPayment = pallet_transaction_payment;
  pub type Sudo = pallet_sudo;
}
```

---

## Pallet Configuration

Before you can add a Pallet to the final runtime, it needs to be configured as defined in the `Config`.

<div class="flex-container text-small">
<div class="left" style="max-width: 50%;">

In the Pallet:

```rust
/// The timestamp pallet configuration trait.
#[pallet::config]
pub trait Config: frame_system::Config {
  type Moment: Parameter + Default + AtLeast32Bit + Scale<Self::BlockNumber, Output = Self::Moment> + Copy + MaxEncodedLen + scale_info::StaticTypeInfo;

  type OnTimestampSet: OnTimestampSet<Self::Moment>;

  #[pallet::constant]
  type MinimumPeriod: Get<Self::Moment>;

  type WeightInfo: WeightInfo;
}
```

</div>

<div class="right" style="max-width: 50%; padding-left: 10px;">

In the Runtime:

```rust
/// The timestamp pallet configuration.

impl pallet_timestamp::Config for Runtime {
  type Moment = u64;

  type OnTimestampSet = Aura;


  type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;

  type WeightInfo = ();
}
```

</div>
</div>

---

## Summary

- **FRAME**: A Rust framework that simplifies Substrate runtime development.
- **Goals**: Improve modularity, flexibility & developer ergonomics while maintaining safety.
- Core components:
  - **Pallets**: Modular runtime components with storage, extrinsics, events, errors, and hooks.
  - **FRAME System**: Foundational pallet providing basic blockchain types and functions.
  - **FRAME Executive**: Coordinates runtime execution (initialization, checks, extrinsic processing).
  - **Construct Runtime**: Combines pallets into a complete runtime
- **Development approach**: Uses Rust macros to generate boilerplate code while keeping the developer interface clean.
