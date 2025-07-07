---
title: Transaction Extensions
description: Transaction Extensions, Transaction Priority.
---

# Transaction Extensions

---v

- In this lecture you will learn above one of the more advanced FRAME concepts, _Transaction Extensions_.

* They allow for a multitude of custom features to be added to FRAME transactions.

---

## History

- Transaction Extensions are an evolution of Signed Extensions.
- See [the introducing PR](https://github.com/paritytech/polkadot-sdk/pull/3685)
- In essence, they are a generic way to **extend** the transaction.

---

## Anatomy

A transaction extension can either or both of the following things:

- Some additional data that is attached to the transaction.
  - The tip!

<!-- .element: class="fragment" -->

- Some hooks that are executed before and after the transaction is executed.
  - Before each transaction is executed, it must pay its fee upfront.
  - Perhaps refund the fee partially 🤑.

<!-- .element: class="fragment" -->

---v

### Anatomy

- Some additional validation logic that is used to validate the transaction, and give feedback to the pool.
  - Set priority of transaction priority based on some metric!

<!-- .element: class="fragment" -->

- Some additional data that must be present in the signed payload of each transaction.
  - Data that the sender has, the chain also has, it is not communicated itself, but it is part of the signature payload.
  - Spec version and genesis hash is part of all transaction's signature payload!

<!-- .element: class="fragment" -->

---v

### Anatomy: Let's Peek at the Trait

```rust [1-100|4-6|11|12-21|22-29|32-38]
pub trait TransactionExtension<Call>: /* snip required traits */
where Call: Dispatchable,
{
    type Implicit: Codec + StaticTypeInfo;
    type Val;
    type Pre;

    const IDENTIFIER: &'static str;

    // Required methods
    fn weight(&self, call: &Call) -> Weight;
    fn validate(
        &self,
        origin: <Call as Dispatchable>::RuntimeOrigin,
        call: &Call,
        info: &<Call as Dispatchable>::Info,
        len: usize,
        self_implicit: Self::Implicit,
        inherited_implication: &impl Implication,
        source: TransactionSource,
    ) -> Result<(ValidTransaction, Self::Val, <Call as Dispatchable>::RuntimeOrigin), TransactionValidityError>;
    fn prepare(
        self,
        val: Self::Val,
        origin: &<Call as Dispatchable>::RuntimeOrigin,
        call: &Call,
        info: &<Call as Dispatchable>::Info,
        len: usize,
    ) -> Result<Self::Pre, TransactionValidityError>;

	// Provided (but might implement)
	fn post_dispatch(
        pre: Self::Pre,
        info: &<Call as Dispatchable>::Info,
        post_info: &mut <Call as Dispatchable>::PostInfo,
        len: usize,
        result: &Result<(), DispatchError>,
    ) -> Result<(), TransactionValidityError> { ... }

    // Provided methods
    fn implicit(&self) -> Result<Self::Implicit, TransactionValidityError> { ... }
    fn metadata() -> Vec<TransactionExtensionMetadata> { ... }
    /* snip */
}
```

---

## Grouping Transaction Extension

- Is also a transaction extension itself!
- You can look at the implementation yourself.. but the TLDR is:
  - Executes each individually
  - Passes resulting `Origin` from one to the next
  - Combines results

Notes:

TODO: how `TransactionValidity` is `combined_with` is super important here, but probably something to cover more somewhere else and recap here.

---v

## Usage In The Runtime

- Each runtime has a bunch of signed extensions. They can be grouped as a tuple

```rust
pub type TxExtension = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	pallet_asset_tx_payment::ChargeAssetTxPayment<Runtime>,
);

type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, TxExtension>;
```

- Transaction extensions might originate from a pallet, but are applied to ALL EXTRINSICS 😮‍💨!

Notes:

We will get to this later as well, but recall that TransactionExtensions are not a _FRAME/Pallet_
concept per se. FRAME just implements them. This also implies that everything regarding signed
extensions is applied to **all transactions**, throughout the runtime.

---

## Encoding

```rust
struct Foo(u32, u32);
impl TransactionExtension for Foo {
  type Implicit = u32;
  fn implicit(&self) -> Result<Self::Implicit, TransactionValidityError> {
    Ok(42u32)
  }
  /* snip */
}

pub struct UncheckedExtrinsic<Address, Call, Signature, (Foo)>
{
	pub preamble: Preamble<(Address, Signature, Extension)>,
	pub function: Call,
}
```

- Two `u32` are decoded as `self`, `42u32` is expected to be in the signature payload.

---v

## Encoding

Here's the `check` function of `CheckedExtrinsic` to hint at this:

```rust
// UncheckedExtrinsic::check
fn check(self, lookup: &Lookup) -> Result<Self::Checked, TransactionValidityError> {
  Ok(match self.preamble {
    Preamble::Signed(signed, signature, tx_ext) => {
      let signed = lookup.lookup(signed)?;
      // The `Implicit` is "implicitly" included in the payload.
      let raw_payload = SignedPayload::new(self.function, tx_ext)?;
      if !raw_payload.using_encoded(|payload| signature.verify(payload, &signed)) {
        return Err(InvalidTransaction::BadProof.into())
      }
      let (function, tx_ext, _) = raw_payload.deconstruct();
      CheckedExtrinsic { format: ExtrinsicFormat::Signed(signed, tx_ext), function }
    },
    /* snip */
  })
}
```

---

## Transaction Pool Validation

- Each pallet also has `#[pallet::validate_unsigned]`.
- This kind of overlaps with creating a transaction extension and implementing `bare_validate`.
- Substrate is in the process of migrating to transaction extensions.

Notes:

https://github.com/paritytech/substrate/issues/6102
https://github.com/paritytech/substrate/issues/4419

---v

### Transaction Pool Validation

- Recall that transaction pool validation should be minimum effort and static.
- In `executive`, we only do the following:
  - check signature.
  - call `TransactionExtension::validate`/`TransactionExtension::bare_validate`
  - call `ValidateUnsigned::validate`, if unsigned.

Notes:

> Transaction queue is not part of the consensus system. Validation of transaction are _free_. Doing
> too much work in validation of transactions is essentially opening a door to be DOS-ed.

---

## Post Dispatch

- The dispatch result, plus generic type (`type Pre`) returned from `prepare` is passed to `post_dispatch`.

---

## Notable Transaction Extensions

- These are some of the default transaction extensions that come in FRAME.
- See if you can predict how they are made!

---v

### `ChargeAssetTxPayment`

Charge payments, refund if `Pays::No`.

```rust
pub enum Pre<T: Config> {
	Charge {
		tip: BalanceOf<T>,
		// who paid the fee
		who: T::AccountId,
		// imbalance resulting from withdrawing the fee
		initial_payment: InitialPayment<T>,
		// asset_id for the transaction payment
		asset_id: Option<ChargeAssetIdOf<T>>,
		// weight used by the extension
		weight: Weight,
	},
	NoCharge {
		// weight initially estimated by the extension, to be refunded
		refund: Weight,
	},
}
```

<!-- .element: class="fragment" -->

---v

### `CheckGenesis`

Wants to make sure you are signing against the right chain.

Put the genesis hash in `implicit`.

<!-- .element: class="fragment" -->

`CheckSpecVersion` and `CheckTxVersion` work very similarly.

<!-- .element: class="fragment" -->

---v

### `CheckNonZeroSender`

- interesting story: any account can sign on behalf of the `0x00` account.
- discovered by [@xlc](https://github.com/xlc).
- uses `validate` to ensure the signing account is not `0x00`.
  - Note: used to check in both `validate` and `pre_dispatch` in the old `SignedExtension`.

Notes:

https://github.com/paritytech/substrate/issues/10413

---v

### `CheckNonce`

- `validate`: check the nonce, DO NOT WRITE ANYTHING, returns `provides` and `requires`.
- `prepare`: check nonce and actually update it.

<!-- .element: class="fragment" -->

<div>

- remember that:
  - `validate` is only for lightweight checks, no read/write.
  - anything you write to storage is reverted anyhow.

</div>

<!-- .element: class="fragment" -->

---v

### `CheckWeight`

- Check there is enough weight in `validate`.
- Calculate and update the consumed weight in `prepare`.
- Adjust consumed weight in `post_dispatch` based on unspent weight.

<!-- .element: class="fragment" -->

---

## Big Picture: Pipeline of Extension

- Transaction extensions remind me of other pipelines in computer graphics or data processing where you pass data from one stage to another.

---

## Exercises

- Walk over the notable transaction extensions above and riddle each other about how they work.
- TransactionExtensions are an important part of the transaction encoding. Try and encode a correct
  transaction against a template runtime in any language that you want, using only a scale-codec
  library.
- TransactionExtension that logs something on each transaction
- TransactionExtension that keeps a counter of all transactions
- TransactionExtension that keeps a counter of all successful/failed transactions
- TransactionExtension that tries to refund the transaction from each account as long as they submit less
  than 1tx/day.
