---
title: Introduction to FRAME
description: An introduction into FRAME, a framework for building Substrate runtimes.
duration: 1 hour
---

## Lesson Plan

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

- Substrate Lectures

</td>
<td>

- Substrate Lectures

</td>
<td>

- Introduction to FRAME
- 👩‍💻 Exercise: Proof of Existence Runtime
- Announce FRAME Assignment

</td>
<td>

- Pallet Coupling
- FRAME Common Knowledge (Pallets & Traits)
- FRAME Storage

</td>
<td>

- Events & Errors
- Calls
- Origins
- Outer Enum
- Hooks

</td>
<td>

- Complete FRAMEless Exercise

</td>
</tr>
<tr class="text-small">
<td>

- Construct Runtime + Tests
- 👨‍💻 Exercise: Tests
- FRAME Benchmarking
- 👨🏾‍💻 Exercise: Benchmarking

</td>
<td>

- FRAME Under the Hood
  - Deep Dive
  - Executive
- Signed Extensions
- Migrations & Try Runtime

</td>
<td>

- Spill Over + Live Coding
- Polkadot Lectures

</td>

<td>

- Polkadot Lectures

</td>
<td>

- Polkadot Lectures

</td>
<td>

- Complete FRAME Exercise

</td>
</tr>
</table>

---

# Introduction to FRAME

---

## What is FRAME?

FRAME is a Rust framework for more easily building Substrate runtimes.

---

## Explaining FRAME Concisely

<pba-flex center>

Writing the Sudo Pallet:

Without FRAME: 2210 lines of code.

With FRAME: 310 lines of code.

7x Smaller.

</pba-flex>

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

<img style="height: 600px" src="./img/frame1.svg" />

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
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  #[pallet::config]  // snip
  #[pallet::event]   // snip
  #[pallet::error]   // snip
  #[pallet::storage] // snip
  #[pallet::call]    // snip
}
```

---

## FRAME Macros

Rust allows you to write Macros, which is code that generates code.

FRAME uses Macros to simplify the development of Pallets, while keeping all of the benefits of using Rust.

We will look more closely at each attribute throughout this module.

---

## See For Yourself

- `wc -l` will show the number of lines of a file.
- `cargo expand` will expand the macros to "pure" Rust.

```sh
➜  substrate git:(master) ✗ wc -l frame/sudo/src/lib.rs
    310 frame/sudo/src/lib.rs

➜  substrate git:(master) ✗ cargo expand -p pallet-sudo | wc -l
    2210
```

---

## FRAME System

The FRAME System is a Pallet which is assumed to always exist when using FRAME. You can see that in the `Config` of every Pallet:

```rust
#[pallet::config]
pub trait Config: frame_system::Config { ... }
```

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
construct_runtime!(
	pub struct Runtime {
		System: frame_system,
		RandomnessCollectiveFlip: pallet_randomness_collective_flip,
		Timestamp: pallet_timestamp,
		Aura: pallet_aura,
		Grandpa: pallet_grandpa,
		Balances: pallet_balances,
		TransactionPayment: pallet_transaction_payment,
		Sudo: pallet_sudo,
		// Include the custom logic from the pallet-template in the runtime.
		TemplateModule: pallet_template,
	}
);
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
