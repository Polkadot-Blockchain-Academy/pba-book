---
title: Substrate's Transaction Pool and its Runtime API
duration: 30 minutes
---

# Substrate's Transaction Pool

---

## Transaction Pools

<pba-cols>
<pba-col>

<img style="width: 500px;" src="./img/BlockspaceBooth.png" />

</pba-col>

<pba-col>
<img style="width: 500px; margin-left: -100px; margin-top: 250px;" src="./img/short-line.png" /> <!-- .element: class="fragment" -->
</pba-col>

<pba-col>
<img style="width: 700px; margin-left: -100px; margin-top: 100px;" src="./img/long-line.png" /> <!-- .element: class="fragment" -->
</pba-col>

</pba-cols>

Notes:

The blockchain produces blockspace, and users buy that blockspace.
Why do they buy it?
So they can contribute to the shared story.
So they can interact with the shared state machine.
You can think of these users standing in line with transactions in their hand, waiting for the chance to put their transactions into the chain's blockspace.
Sometimes the demand for blockspace is low and the queue is short.
In this case the queue gets completely emptied each time a new block is created.
Other times it gets long and backed up.
Then when a block comes, only a part of the queue gets emptied.

This simple model provides some good intuition about how the transaction pool works, but it is a bit simplified.

First, It is actually a priority queue.
You can jump the line by offering to bribe the block producers.

Second, it is more accurate to think of the transactions themselves waiting in line, not the users who sent those transactions.

Let's take a closer look.

---v

### Paths of a Transaction

<img style="width: 900px;" src="./img/blockchain_p2p.svg" />

Notes:

Previously, in the blockchain module, we saw this figure.
It points out that each node has its own view of the blockchain.
Now I'll show you another layer of detail which is that each node also has its own transaction pool
CLICK

---v

### Paths of a Transaction

<img style="width: 900px;" src="./img/blockchain_p2p_with_pool.svg" />

Notes:

There are many paths a transaction can take from the user who signed it to a finalized block.
Let's talk through some.
Directly to user's authoring node and into chain is simplest.
Could also be gossiped to other author.
Could even go in a block, get orphaned off, back to tx pool, and then in a new block

---v

### Pool Validation

- Check signature
- Check that sender can afford fees
- Make sure state is ready for application

Notes:

When a node gets a transaction, it does some light pre-validation sometimes known as pool validation.
This checking determines whether the transactions is {valid now, valid in some potential future, invalid}.
There is periodic re-validation if transactions have been in the pool for a long time.

---v

### Pool Prioritization

- Priority Queue
- Prioritized by...
  - Fee
  - Bribe
  - Fee per blockspace
- This is all off-chain

Notes:

There are a few more things the Substrate tx pool does too, and we will look at them in detail soon.

---

## Tx Pool Runtime Api

```rust
pub trait TaggedTransactionQueue<Block: BlockT>: Core<Block> {
    fn validate_transaction(
        &self,
        __runtime_api_at_param__: <Block as BlockT>::Hash,
        source: TransactionSource,
        tx: <Block as BlockT>::Extrinsic,
    ) -> Result<TransactionValidity, ApiError> { ... }
}
```

[`TaggedTransactionQueue` Rustdocs](https://paritytech.github.io/substrate/master/sp_transaction_pool/runtime_api/trait.TaggedTransactionQueue.html)

Introduced in [paritytech/substrate#728](https://github.com/paritytech/substrate/issues/728)

Notes:

This is another runtime api, similar to the block builder and the core that are used for creating and importing blocks.
Like most others, it requires that the Core api also be implemented.

This one is slightly different in that it is actually called from off-chain, and is not part of your STF.
So let's talk about that for a little bit.

---v

### Runtime vs STF

<img style="width: 1100px;" src="./img/peter-parker-glasses-off.png" />

Notes:

It is commonly said that the runtime is basically your STF.
This is a good first order approximation.
It is nearly true.

---v

### Runtime vs STF

<img style="width: 1100px;" src="./img/peter-parker-glasses-on.png" />

Notes:

But as we can see here, when we put our glasses on, actually only some of the apis are part of the stf.

---v

## Why is pool logic in the runtime?

- Transaction type is Opaque
- Runtime logic is opaque
- You must understand the transaction to prioritize it

Notes:

So if this is not part of the STF why is it in the runtime at all?
This logic is tightly related to the runtime application logic.
The types are opaque outside of the runtime.
So this logic must go in the runtime.

But if it is not on-chain, can individual validators customize it.
In short yes.
There is a mechanism for this.
We won't go deeply into the mechanism, but validators can specify alternate wasm blocs to use instead of the official one.

---

## Jobs of the API

- Make fast pre-checks
- Give the transaction a priority
- Determine whether the transaction is ready now or may be ready in the future
- Determine a dependency graph among the transactions

Notes:

So we spoke earlier about the jobs of a transaction pool in general.
Specifically the pre-checks and the priority
Here is a more specific list of tasks that Substrate's TaggedTransactionPool does.

The second two points are the new additions, and they are the duty of the "tags" after which the tagged transaction queue is named.

The results of all of this are returned to the client side through a shared type `ValidTransaction` or `InvalidTransaction`

---v

### `ValidTransaction`

```rust
pub struct ValidTransaction {
    pub priority: TransactionPriority,
    pub requires: Vec<TransactionTag>,
    pub provides: Vec<TransactionTag>,
    pub longevity: TransactionLongevity,
    pub propagate: bool,
}
```

[`ValidTransaction` Rustdocs](https://paritytech.github.io/substrate/master/sp_runtime/transaction_validity/struct.ValidTransaction.html)

Notes:

We indicate that the transaction passes the prechecks at all by returning this valid transaction struct.
If it weren't even valid, we would return a different, `InvalidTransaction` struct.
You learned yesterday how to navigate the rustdocs to find the docs on that one.

Priority we've discussed.
It is worth noting that the notion of priority is intentionally opaque to the client.
The runtime may assign this value however it sees fit.

Provides and requires all forming a dependency graph between the transactions.
Requires is a list of currently unmet dependency transactions.
This transaction will be ready in a future where these dependencies are met so it is kept in the pool.

A simple intuitive example of this is payments.
Image alice pays bob some tokens in transaction1.
Then bob pays those same tokes to charlie in transaction2.
trasnaction2 will be valid only after transaction1 has been applied.
It is a dependency.

Longevity is a field I'm not so familiar with.
It is how long the transaction should stay in the pool before being dropped or re-validated.

<!-- FIXME TODO what are the units? How does one set it? -->

And finally whether the transaction should be gossiped.
This is usually true.
Only in special edge cases would this be false.

---v

### Example 1: UTXO System

<img src="./img/utxo.svg" />

Notes:

Prioritize by implicit tip (difference of inputs and outputs)
Requires all missing input transactions
provides this input

---v

### Example 2: Nonced Account System

<img src="./img/accounts.svg" />

Notes:

Prioritize by explicit tip
Requires all previous nonces for this account
provides this nonce for this account

This demonstrates one of the biggest downsides of the Accounts system.
Transactions cannot deterministically specify the initial state on which they operate.
There is only an inherent ordering between transactions from the same account.

---v

## Always Re-check On-chain

<img style="width: 900px;" src="./img/blockchain_p2p_with_pool.svg" />

Notes:

None of this new pool information changes the fundamentals you learned last week.
You must execute the state transitions in full on chain.

Most of the time you are not the block author.
When you import a block from another node, you cannot trust them to have done the pre-checks correctly.
