---
title: Frame Miscellaneous
description: Signed Extensions and Pallet Tasks.
---

# Frame Miscellaneous

---

## Pre-Dispatch logic

- How is nonce and tip function implemented in polkadot?

<!-- .element: class="fragment" -->

- How do we add more pre-dispatch logic to our runtime?

<!-- .element: class="fragment" -->

Notes:

- See what is signed in the extrinsic.
- Looked at Signed Extension.

---

## Extending pre-dispatch checks

<img rounded src="../../../../assets//img/5-Substrate/dev-5-x-signed-extensions.svg" />

---

## Signed Extension

- Generic way to **extend** the transaction.
- Similar to middleware.
- Tips and Fees (ChargeTransactionPayment).
- Nonce (CheckNonce).

---

## Grouping Signed Extension

- Is also a signed extension itself!

- You can look at the implementation yourself.. but the TLDR is:

- Main takeaways:
  - `type AdditionalSigned = (SE1::AdditionalSigned, SE2::AdditionalSigned)`,
  - all of hooks:
    - Executes each individually, combines results

Notes:

TODO: how `TransactionValidity` is `combined_with` is super important here, but probably something to cover more in 4.3 and recap here.

---

## Usage In The Runtime

- Each runtime has a bunch of signed extensions. They can be grouped as a tuple

```rust
pub type SignedExtra = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	pallet_asset_tx_payment::ChargeAssetTxPayment<Runtime>,
);

type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
```

- Signed extensions might originate from a pallet, but are applied to ALL EXTRINSICS 😮‍💨!

Notes:

We will get to this later as well, but recall that SignedExtensions are not a _FRAME/Pallet_
concept per se. FRAME just implements them. This also implies that everything regarding signed
extensions is applied to **all transactions**, throughout the runtime.

---

### `ChargeTransactionPayment`

Charge payments, refund if `Pays::Yes`.

```rust
type Pre = (
  // tip
  BalanceOf<T>,
  // who paid the fee - this is an option to allow for a Default impl.
  Self::AccountId,
  // imbalance resulting from withdrawing the fee
  <<T as Config>::OnChargeTransaction as OnChargeTransaction<T>>::LiquidityInfo,
);
```

<!-- .element: class="fragment" -->

---

## Feeless Signed Extension

- If some condition meets, do not charge any fee.
- Every pallet/extrinsic needs to define a new Signed Extension.

---

## Feeless Signed Extension

```rust
#[pallet::feeless_if(|origin: &OriginFor<T>, ticket: &Ticket| -> bool {
    let account = ensure_signed(origin.clone())?;
    some_conditions_here(&account, &ticket)
})]
```

- [Issue #1725](https://github.com/paritytech/polkadot-sdk/issues/1725)

---

## Admin tasks

- You have accumulating rewards, and user needs to claim it.
- Voting period is over, user needs to unfreeze to get back their locked tokens.

<br/>

- Standard means of determining what work can be executed via an off-chain worker or script.

## <!-- .element: class="fragment" -->

## Off-chain worker and pallet::Tasks

```rust [0-7|8-17|18-100]
#[pallet::task]
pub enum Task<T: Config> {
    AddNumberIntoTotal {
        i: u32,
    }
}

/// Some running total.
#[pallet::storage]
pub(super) type Total<T: Config<I>, I: 'static = ()> =
StorageValue<_, (u32, u32), ValueQuery>;

/// Numbers to be added into the total.
#[pallet::storage]
pub(super) type Numbers<T: Config<I>, I: 'static = ()> =
StorageMap<_, Twox64Concat, u32, u32, OptionQuery>;

#[pallet::tasks_experimental]
impl<T: Config<I>, I: 'static> Pallet<T, I> {
	/// Add a pair of numbers into the totals and remove them.
	#[pallet::task_list(Numbers::<T, I>::iter_keys())]
	#[pallet::task_condition(|i| Numbers::<T, I>::contains_key(i))]
	#[pallet::task_index(0)]
	pub fn add_number_into_total(i: u32) -> DispatchResult {
		let v = Numbers::<T, I>::take(i).ok_or(Error::<T, I>::NotFound)?;
		Total::<T, I>::mutate(|(total_keys, total_values)| {
			*total_keys += i;
			*total_values += v;
		});
		Ok(())
	}
}
```

Notes:
PR: https://github.com/paritytech/polkadot-sdk/pull/1343
