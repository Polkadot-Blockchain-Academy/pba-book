---
title: FRAME Calls
description: FRAME Calls for web3 builders.
duration: 1 hour
---

# FRAME Calls

---

## Overview

Calls allow users to interact with your state transition function.

In this lecture, you will learn how to create calls for your Pallet with FRAME.

---

## Terminology

The term "call", "extrinsic", and "dispatchable" all get mixed together.

Here is a sentence which should help clarify their relationship, and why they are such similar terms:

> Users submit an **extrinsic** to the blockchain, which is **dispatched** to a Pallet **call**.

---

## Call Definition

Here is a simple pallet call. Let's break it down.

```rust
#[pallet::call(weight(<T as Config>::WeightInfo))]
impl<T: Config> Pallet<T> {
	#[pallet::call_index(0)]
	pub fn transfer(
		origin: OriginFor<T>,
		dest: AccountIdLookupOf<T>,
		#[pallet::compact] value: T::Balance,
	) -> DispatchResult {
		let source = ensure_signed(origin)?;
		let dest = T::Lookup::lookup(dest)?;
		<Self as fungible::Mutate<_>>::transfer(&source, &dest, value, Expendable)?;
		Ok(())
	}
}
```

---

## Call Implementation

Calls are just functions which are implemented on top of the `Pallet` struct.

You can do this with any kind of function, however, "FRAME magic" turns these into dispatchable calls through the `#[pallet::call]` macro.

---

## Call Origin

Every pallet call must have an `origin` parameter, which uses the `OriginFor<T>` type, which comes from `frame_system`.

```rust
/// Type alias for the `Origin` associated type of system config.
pub type OriginFor<T> = <T as crate::Config>::RuntimeOrigin;
```

---

## Origin

The basic origins available in frame are:

```rust
/// Origin for the System pallet.
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub enum RawOrigin<AccountId> {
	/// The system itself ordained this dispatch to happen: this is the highest privilege level.
	Root,
	/// It is signed by some public key and we provide the `AccountId`.
	Signed(AccountId),
	/// It is signed by nobody, can be either:
	/// * included and agreed upon by the validators anyway,
	/// * or unsigned transaction validated by a pallet.
	None,
}
```

We will have another presentation diving deeper into Origins.

---

## Origin Checking

Normally, the first thing you do in a call is check that the origin of the caller is what you expect.

Most often, this is checking that the extrinsic is `Signed`, which is a transaction from a user account.

```rust
let caller: T::AccountId = ensure_signed(origin)?;
```

---

## Call Parameters

Pallet calls can have additional parameters beyond `origin` allowing you to submit relevant data about what the caller would like to do.

All call parameters need to satisfy the `Parameter` trait:

```rust
/// A type that can be used as a parameter in a dispatchable function.
pub trait Parameter: Codec + EncodeLike + Clone + Eq + fmt::Debug + scale_info::TypeInfo {}
impl<T> Parameter for T where T: Codec + EncodeLike + Clone + Eq + fmt::Debug + scale_info::TypeInfo {}
```

---

## Parameter Limits

Most everything can be used as a call parameter, even a normal `Vec`, however, FRAME ensures that encoded block are smaller than a maximum block size, which inherently limits the extrinsic length.

In Polkadot this is currently 5 MB.

---

## Compact Parameters

Parameters that are compact encoded can be used in calls.

```rust
pub fn transfer(
	origin: OriginFor<T>,
	dest: AccountIdLookupOf<T>,
	#[pallet::compact] value: T::Balance,
) -> DispatchResult { ... }
```

This can help save lots of bytes, especially in cases like balances as shown above.

---

## Call Logic

The most relevant part of a call is the "call logic".

There is really nothing magical happening here, just normal Rust.

However, you must follow one important rule...

---

## Calls MUST NOT Panic

Under no circumstances (save, perhaps, storage getting into an irreparably damaged state) must this function panic.

Allowing callers to trigger a panic from a call can allow users to attack your chain by bypassing fees or other costs associated with executing logic on the blockchain.

---

## Call Return

Every call returns a `DispatchResult`:

```rust
pub type DispatchResult = Result<(), sp_runtime::DispatchError>;
```

This allows you to handle errors in your runtime, and NEVER PANIC!

---

## Returning an Error

At any point in your call logic, you can return a `DispatchError`.

```rust
ensure!(new_balance >= min_balance, Error::<T, I>::LiquidityRestrictions);
```

When you do, thanks to transactional storage layers, all modified state will be reverted.

---

## Returning Success

If everything in your pallet completed successfully, you simply return `Ok(())`, and all your state changes are committed, and the extrinsic is considered to have executed successfully.

---

## Call Index

It is best practice to explicitly label your calls with a `call_index`.

```rust
#[pallet::call_index(0)]
```

This can help ensure that changes to your pallet do not lead to breaking changes to the transaction format.

---

## Call Encoding

At a high level, a call is encoded as two bytes (plus any parameters):

1. The Pallet Index
1. The Call Index

Pallet Index comes from the order / explicit numbering of the `construct_runtime!`. If things change order, without explicit labeling, a transaction generated by a wallet (like a ledger) could be incorrect!

Notes:

Note that this also implies there can only be 256 calls per pallet due to the 1 byte encoding.

---

## Weight

Each call must also include specify a call `weight`.

We have another lecture on Weights and Benchmarking, but the high level idea is that this weight function tells us how complex the call is, and the fees that should be charged to the user.

---

## Weight Per Call

This can be done per call:

```rust [3]
#[pallet::call]
impl<T: Config> Pallet<T> {
	#[pallet::weight(T::WeightInfo::transfer())]
	#[pallet::call_index(0)]
	pub fn transfer(
		origin: OriginFor<T>,
		dest: AccountIdLookupOf<T>,
		#[pallet::compact] value: T::Balance,
	) -> DispatchResult {
		let source = ensure_signed(origin)?;
		let dest = T::Lookup::lookup(dest)?;
		<Self as fungible::Mutate<_>>::transfer(&source, &dest, value, Expendable)?;
		Ok(())
	}
}
```

---

## Weight for the Pallet

Or for all calls in the pallet:

```rust [1]
#[pallet::call(weight(<T as Config>::WeightInfo))]
impl<T: Config> Pallet<T> {
	#[pallet::call_index(0)]
	pub fn transfer(
		origin: OriginFor<T>,
		dest: AccountIdLookupOf<T>,
		#[pallet::compact] value: T::Balance,
	) -> DispatchResult {
		let source = ensure_signed(origin)?;
		let dest = T::Lookup::lookup(dest)?;
		<Self as fungible::Mutate<_>>::transfer(&source, &dest, value, Expendable)?;
		Ok(())
	}
}
```

In this case, the weight function name is assumed to match the call name for all calls.

Notes:

<https://github.com/paritytech/substrate/pull/13932>

---

<!-- .slide: data-background-color="#4A2439" -->

# Questions
