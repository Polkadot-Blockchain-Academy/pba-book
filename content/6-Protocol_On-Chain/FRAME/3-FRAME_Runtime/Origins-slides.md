---
title: FRAME Origin
description: Deep dive into FRAME Origins
duration: 1 hour
---

# Origin

---

## Origin

This presentation will cover the use of Origin in FRAME, and how you can customize and extend this abstractions.

([Reference in `polkadot-sdk-docs`](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/frame_origin/index.html))

---

## What is Origin?

All dispatchable calls have an `Origin` that describes where the call originates from.

```rust
/// Make some on-chain remark.
#[pallet::weight(T::SystemWeightInfo::remark(_remark.len() as u32))]
pub fn remark(origin: OriginFor<T>, _remark: Vec<u8>) -> DispatchResultWithPostInfo {
	ensure_signed_or_root(origin)?;
	Ok(().into())
}
```

---

## FRAME System `RawOrigin`

These are origins which are included with FRAME by default.

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

---

## How is it used?

The Runtime Origin is used by dispatchable functions to check where a call has come from.

This is similar to `msg.sender` in Solidity, but FRAME is more powerful, and so is `Origin`.

---

## Origin Checks

```rust
/// Ensure that the origin `o` represents the root. Returns `Ok` or an `Err` otherwise.
pub fn ensure_root<OuterOrigin, AccountId>(o: OuterOrigin) -> Result<(), BadOrigin>
```

```rust
/// Ensure that the origin `o` represents a signed extrinsic (i.e. transaction).
/// Returns `Ok` with the account that signed the extrinsic or an `Err` otherwise.
pub fn ensure_signed<OuterOrigin, AccountId>(o: OuterOrigin) -> Result<AccountId, BadOrigin>
```

```rust
/// Ensure that the origin `o` represents an unsigned extrinsic. Returns `Ok` or an `Err` otherwise.
pub fn ensure_none<OuterOrigin, AccountId>(o: OuterOrigin) -> Result<(), BadOrigin>
```

```rust
/// Ensure that the origin `o` represents either a signed extrinsic (i.e. transaction) or the root.
/// Returns `Ok` with the account that signed the extrinsic, `None` if it was root,  or an `Err`
/// otherwise.
pub fn ensure_signed_or_root<OuterOrigin, AccountId>(o: OuterOrigin) -> Result<Option<AccountId>, BadOrigin>
```

---

## Examples: Signed Origin

A Simple Balance Transfer.

```rust
#[pallet::call_index(0)]
#[pallet::weight(T::WeightInfo::transfer())]
pub fn transfer(
	origin: OriginFor<T>,
	dest: AccountIdLookupOf<T>,
	#[pallet::compact] value: T::Balance,
) -> DispatchResultWithPostInfo {
	let transactor = ensure_signed(origin)?;
	// -- snip --
}
```

Most extrinsics use a `Signed` origin.

---

## Examples: Root Origin

The extrinsic to upgrade a chain.

```rust
/// Set the new runtime code.
#[pallet::call_index(2)]
#[pallet::weight((T::BlockWeights::get().max_block, DispatchClass::Operational))]
pub fn set_code(origin: OriginFor<T>, code: Vec<u8>) -> DispatchResultWithPostInfo {
	ensure_root(origin)?;
	Self::can_set_code(&code)?;
	T::OnSetCode::set_code(code)?;
	Ok(().into())
}
```

`Root` has access to many functions which can directly modify your blockchain. Assume Root access can do anything.

---

## Examples: None Origin

Setting the timestamp of the block.

```rust
		/// Set the current time.
		#[pallet::call_index(0)]
		#[pallet::weight((T::WeightInfo::set(), DispatchClass::Mandatory))]
		pub fn set(origin: OriginFor<T>, #[pallet::compact] now: T::Moment) -> DispatchResult {
			ensure_none(origin)?;
			// -- snip --
		}
	}
```

`None` origin is not very straight forward. More details next...

---

## None for Inherents

`None` origin can be used to represents extrinsics which are specifically included by the block author, also known as an inherent.

In those cases, it includes inherent checking logic with `ProvideInherent`.

```rust
#[pallet::inherent]
impl<T: Config> ProvideInherent for Pallet<T> {
	type Call = Call<T>;
	type Error = InherentError;
	const INHERENT_IDENTIFIER: InherentIdentifier = INHERENT_IDENTIFIER;

	// -- snip --
```

---

## None for Unsigned

`None` can also be used to represent "unsigned extrinsics", which are intended to be submitted by anyone without a key.

In those cases, it includes unsigned validation logic with `ValidateUnsigned`.

```rust
#[pallet::validate_unsigned]
impl<T: Config> ValidateUnsigned for Pallet<T> {
	type Call = Call<T>;
	fn validate_unsigned(source: TransactionSource, call: &Self::Call) -> TransactionValidity {
		Self::validate_unsigned(source, call)
	}

	fn pre_dispatch(call: &Self::Call) -> Result<(), TransactionValidityError> {
		Self::pre_dispatch(call)
	}
}
```

---

## Custom Origins

Origins are extensible and customizable.

Each pallet can introduce new Origins which can be used throughout the runtime.

```rust
/// The `#[pallet::origin]` attribute allows you to define some origin for the pallet.
#[pallet::origin]
pub struct Origin<T>(PhantomData<(T)>);
```

---

## Example: Collective Pallet

```rust
/// Origin for the collective module.
pub enum RawOrigin<AccountId, I> {
	/// It has been condoned by a given number of members of the collective from a given total.
	Members(MemberCount, MemberCount),
	/// It has been condoned by a single member of the collective.
	Member(AccountId),
	/// Dummy to manage the fact we have instancing.
	_Phantom(PhantomData<I>),
}
```

This custom origin allows us to represent a collection of users, rather than a single account.
For example: `Members(5, 9)` represents that 5 out of 9 members agree on something as controlled by the collective pallet logic.

---

## Example: Parachain Origin

```rust
/// Origin for the parachains.
#[pallet::origin]
pub enum Origin {
	/// It comes from a parachain.
	Parachain(ParaId),
}
```

This is a custom origin which allows us to represent a message that comes from a parachain.

---

## Re-Dispatching

You can actually dispatch a call within a call with an origin of your choice.

```rust [11]
#[pallet::call_index(0)]
#[pallet::weight({ let dispatch_info = call.get_dispatch_info(); (dispatch_info.weight, dispatch_info.class) })]
pub fn sudo(
	origin: OriginFor<T>,
	call: Box<<T as Config>::RuntimeCall>,
) -> DispatchResultWithPostInfo {
	// This is a public call, so we ensure that the origin is some signed account.
	let sender = ensure_signed(origin)?;
	ensure!(Self::key().map_or(false, |k| sender == k), Error::<T>::RequireSudo);

	let res = call.dispatch_bypass_filter(frame_system::RawOrigin::Root.into());
	Self::deposit_event(Event::Sudid { sudo_result: res.map(|_| ()).map_err(|e| e.error) });
	// Sudo user does not pay a fee.
	Ok(Pays::No.into())
}
```

Here, Sudo Pallet allows a `Signed` origin to elevate itself to a `Root` origin, if the logic allows.

---

## Example: Collective Pallet

Here you can see the Collective Pallet creating, and dispatching with the `Members` origin we showed previously.

```rust [5-6]
	fn do_approve_proposal(seats: MemberCount, yes_votes: MemberCount, proposal_hash: T::Hash, proposal: <T as Config<I>>::Proposal) -> (Weight, u32) {
		Self::deposit_event(Event::Approved { proposal_hash });

		let dispatch_weight = proposal.get_dispatch_info().weight;
		let origin = RawOrigin::Members(yes_votes, seats).into();
		let result = proposal.dispatch(origin);
		Self::deposit_event(Event::Executed { proposal_hash, result: result.map(|_| ()).map_err(|e| e.error) });
		// default to the dispatch info weight for safety
		let proposal_weight = get_result_weight(result).unwrap_or(dispatch_weight); // P1

		let proposal_count = Self::remove_proposal(proposal_hash);
		(proposal_weight, proposal_count)
	}
```

---

## Custom Origin Checks

You can then write logic which is only accessible with custom origins by implementing the `EnsureOrigin` trait.

```rust
/// Some sort of check on the origin is performed by this object.
pub trait EnsureOrigin<OuterOrigin> { ... }
```

These need to be configured in the Runtime, where all custom origins for your runtime are known.

---

## Example: Alliance Pallet

Pallet's can allow for various origins to be configured by the Runtime.

```rust
#[pallet::config]
pub trait Config<I: 'static = ()>: frame_system::Config {
	/// Origin for admin-level operations, like setting the Alliance's rules.
	type AdminOrigin: EnsureOrigin<Self::RuntimeOrigin>;
	/// Origin that manages entry and forcible discharge from the Alliance.
	type MembershipManager: EnsureOrigin<Self::RuntimeOrigin>;
	/// Origin for making announcements and adding/removing unscrupulous items.
	type AnnouncementOrigin: EnsureOrigin<Self::RuntimeOrigin>;
	// -- snip --
}
```

---

## Example: Alliance Pallet

Pallet calls can then use these custom origins to gate access to the logic.

```rust [5]
/// Set a new IPFS CID to the alliance rule.
#[pallet::call_index(5)]
#[pallet::weight(T::WeightInfo::set_rule())]
pub fn set_rule(origin: OriginFor<T>, rule: Cid) -> DispatchResult {
	T::AdminOrigin::ensure_origin(origin)?;

	Rule::<T, I>::put(&rule);

	Self::deposit_event(Event::NewRuleSet { rule });
	Ok(())
}
```

---

## Example: Alliance Pallet

Finally, the Runtime itself is where you configure what those Origins are.

```rust
impl pallet_alliance::Config for Runtime {
	type AdminOrigin = EitherOfDiverse<
		EnsureRoot<AccountId>,
		EnsureProportionAtLeast<AccountId, AllianceCollective, 1, 1>,
	>;
	type MembershipManager = EitherOfDiverse<
		EnsureRoot<AccountId>,
		EnsureProportionMoreThan<AccountId, AllianceCollective, 2, 3>,
	>;
	type AnnouncementOrigin = EitherOfDiverse<
		EnsureRoot<AccountId>,
		EnsureProportionMoreThan<AccountId, AllianceCollective, 1, 3>,
	>;
}
```

As you can see, they can even support multiple different origins!

---

## Fees

Fees are usually handled by some pallet like the Transaction Payments Pallet.

However, if there is no `Signed` origin, you can't really take a fee.

You should assume any transaction which is not from the `Signed` origin is feeless, unless you explicitly write code to handle it.

---

<!-- .slide: data-background-color="#4A2439" -->

# Questions
