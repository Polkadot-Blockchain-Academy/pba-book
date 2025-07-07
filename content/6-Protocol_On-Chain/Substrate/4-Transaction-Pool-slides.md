---
title: Substrate Transaction Pool
---

# Substrate Transaction Pool

---

## Why Transaction Pool?

Because it is a (⛓️ blockspace) market.

<pba-cols>
<pba-col>
<img style="width: 500px;" src="./img/tx-pool/BlockspaceBooth.png" />
</pba-col>

<pba-col>
<img style="width: 500px; margin-left: -100px; margin-top: 250px;" src="./img/tx-pool/short-line.png" /> <!-- .element: class="fragment" -->
</pba-col>

<pba-col>
<img style="width: 700px; margin-left: -100px; margin-top: 100px;" src="./img/tx-pool/long-line.png" /> <!-- .element: class="fragment" -->
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

---

## Context

<image rounded src="../../../assets/img/5-Substrate/dev-pool-context.svg">

Notes:

- Gossip
- Queue of all transactions, queue of ready transactions.
- Then given to the block author, only READY ones.

---v

### Context

Any Transaction Pool has two main objectives:

1. Validate Transactions
   - Ready, Future, or 🗑️?
2. Order them
   - Within each list, who is first?

---

## 1. Transaction Validation

Moving transactions from one list to the other.

<diagram class="mermaid" style="display: flex; width: 80%">
graph LR
    W["🤠 Wild West"] --"😈"--> T["🗑️"]
    W --"😇 ⌛️"--> R["✅ Ready"]
    W --"😇 ⏳"--> F["⏰ Future"]
</diagram>

---v

### 1. Transaction Validation

- Transaction validity is exclusively outside of the transaction pool, and is **100% determined by the Runtime**.
- Transaction validation should be **cheap** to perform.
- Transaction pool is entirely an **offchain operation**.
  - No state change

Notes:

Important, must pause and ask!

- Why is it from the runtime? because the transaction format is opaque and the node doesn't even know what to do with it.
- Why does it have to be cheap? wild west, unpaid, DoS!
- Pesky question: but be aware that from the runtime's perspective, the node could be malicious. The runtime cannot trust the node to obey.
  ** THE RUNTIME MUST RE-VALIDATE TRANSACTIONS LATER in block building and import as well **

---v

### 1. Transaction Validation

The runtime API.

```rust[1-100|6]
impl TaggedTransactionQueue<Block> for Runtime {
  fn validate_transaction(
    source: TransactionSource,
    tx: <Block  as BlockT>::Extrinsic,
    block_hash: <Block as BlockT>::Hash,
  ) -> TransactionValidity {
    ..
  }
}
```

---v

### 1. Transaction Validation

```rust[1-100|5-6|8-9|11-12|14-100|1-100]
pub type TransactionValidity = Result<ValidTransaction, TransactionValidityError>;

/// This is going into `Ready` or `Future`
pub struct ValidTransaction {
  /// If in "Ready", what is the priority?
  pub priority: u64,

  /// For how long is this validity?
  pub longevity: u64,

  /// Should be propagate it?
  pub propagate: bool,

  /// Does this require any other tag to be present in ready?
  ///
  /// This determines "Ready" or "Future".
  pub requires: Vec<Tag>,
  /// Does this provide any tags?
  pub provides: Vec<Tag>,
}

type Tag = Vec<u8>
```

---v

### 1. Transaction Validation: Banning

- Once certain transaction is discovered to be invalid, **its hash** is banned for a fixed duration of time.
- Default in substrate is `Duration::from_secs(60 * 30)`, can be configured via CLI.

Notes:

See: https://github.com/paritytech/substrate/pull/11786

we probably also ban the peer who sent us that transaction? but have to learn.

---

## 2. Transaction Ordering

- `provides` and `requires` is a very flexible mechanism; it allows you to:
  - Specify if a transaction is "Ready" or "Future"
  - Within each, what transactions should ge before which.

Note: it essentially forms a graph.

Order mostly matters within the ready pool. I am not sure if the code maintains an order in `future` as well. In any
case, not a big big deal.

---v

### 2. Transaction Ordering: Quiz Time.

<pba-cols>
<pba-col>

```
(
  A,
  provides: vec![],
  requires: vec![]
)
```

</pba-col>

<pba-col>
<table>
<thead>
  <tr>
    <th>Ready</th>
    <th>Future</th>
  </tr>
</thead>
<tbody class="fragment">
  <tr>
    <td>
    <pre>(A, pr: vec![], rq: vec![])</pre>
    </td>
    <td></td>
  </tr>
</tbody>
</table>
</pba-col>

</pba-cols>

---v

### 2. Transaction Ordering: Quiz Time.

<pba-cols>
<pba-col>

```
(
  B,
  provides: vec![2],
  requires: vec![1]
)
```

</pba-col>

<pba-col>
<table>
<thead>
  <tr>
    <th>Ready</th>
    <th>Future</th>
  </tr>
</thead>
<tbody class="fragment">
  <tr>
    <td>
      <pre>(A, pr: vec![], rq: vec![])</pre>
    </td>
    <td>
      <pre>(B, pr: vec![2], rq: vec![1])</pre>
    </td>
  </tr>
</tbody>
</table>
</pba-col>

</pba-cols>

---v

### 2. Transaction Ordering: Quiz Time.

<pba-cols>
<pba-col>

```
(
  C,
  provides: vec![3],
  requires: vec![2]
)
```

</pba-col>

<pba-col>
<table>
<thead>
  <tr>
    <th>Ready</th>
    <th>Future</th>
  </tr>
</thead>
<tbody class="fragment">
  <tr>
    <td>
      <pre>(A, pr: vec![], rq: vec![])</pre>
    </td>
    <td>
      <pre>(B, pr: vec![2], rq: vec![1])</pre>
    </td>
  </tr>
  <tr>
    <td>
    </td>
    <td>
      <pre>(C, pr: vec![3], rq: vec![2])</pre>
    </td>
  </tr>
</tbody>
</table>
</pba-col>

</pba-cols>

---v

### 2. Transaction Ordering: Quiz Time.

<pba-cols>
<pba-col>

```
(
  D,
  provides: vec![1],
  requires: vec![]
)
```

</pba-col>

<pba-col>
<table>
<thead>
  <tr>
    <th>Ready</th>
    <th>Future</th>
  </tr>
</thead>
<tbody class="fragment">
  <tr>
    <td>
      <pre>(A, pr: vec![], rq: vec![])</pre>
    </td>
    <td>
    </td>
  </tr>
  <tr>
    <td>
      <pre>(D, pr: vec![1], rq: vec![])</pre>
    </td>
    <td>
    </td>
  </tr>
  <tr>
    <td>
      <pre>(B, pr: vec![2], rq: vec![1])</pre>
    </td>
    <td>
    </td>
  </tr>
  <tr>
    <td>
      <pre>(C, pr: vec![3], rq: vec![2])</pre>
    </td>
    <td>
    </td>
  </tr>
</tbody>
</table>
</pba-col>

</pba-cols>

Note: The oder in this slide matters and it is top to bottom.

---v

### 2. Transaction Ordering: `priority`

From the **Ready pool**, when all requirements are met, then `priority` dictates the order.

Further tie breakers:

2. ttl: shortest `longevity` goes first
3. time in the queue: longest to have waited goes first

<!-- .element: class="fragment" -->

Note:

https://github.com/paritytech/polkadot-sdk/blob/bc53b9a03a742f8b658806a01a7bf853cb9a86cd/substrate/client/transaction-pool/src/graph/ready.rs#L146

---v

### 2. Transaction Ordering: `priority`

> How can the pool be a pure FIFO?

Notes:

All priorities set to 0.

---v

### 2. Transaction Ordering: `nonce`

Purposes of a nonce:

1. Ordering
2. Replay protection
3. Double spend protection

---v

### 2. Transaction Ordering: `nonce`

- ✅ You will implement a nonce system using the above primitives as a part of your assignment.
- General idea: `require -> (account, nonce - 1).encode()`, provide: `provides -> (account, nonce).encode()`

Notes:

Transaction Ordering: Each time a transaction is sent from an account, the nonce increases by one. This sequential
numbering ensures that transactions are processed in the order they are sent.

Double Spending Prevention: Since each transaction has a unique nonce, it's impossible for two transactions with the
same nonce to both be valid. This stops attackers from attempting to send the same funds twice.

Replay Attack Protection: In a replay attack, a valid transaction is maliciously or fraudulently repeated or delayed.
With a nonce, once a transaction is executed, any attempt to execute it again will fail since the nonce will no longer
match the current state of the account.

---

## Shower Thought: Runtime vs STF

> Transaction pool is entirely an **offchain operation**

Note:

what we said before. What does this imply?

---v

### Shower Thought: Runtime vs STF

<img style="width: 1100px;" src="./img/tx-pool/peter-parker-glasses-off.png" />

Notes:

It is commonly said that the runtime is basically your STF.
This is a good first order approximation.
It is nearly true.
---v

### Shower Thought: Runtime vs STF

<img style="width: 1100px;" src="./img/tx-pool/peter-parker-glasses-on.png" />

Notes:

But as we can see here, when we put our glasses on, actually only some of the apis are part of the stf. the non-stf
parts are runtime APIs that are called and use, but don't really contribute to the STF. typically the runtime cannot
make assumptions about these. From the PoV of the runtime, when doing the main consensus critical work (authoring,
importing) these did not happen.

---

## Lecture Recap

- Blockspace Market, Competition.
- Main tasks:
  - Validate (valid and invalid)
  - Order (split valid into "Ready" and "Future")
    - provides and requires
    - priority: Fee/tip
- By Runtime, but not in STF

Notes:

- Each node's pool is a local wild west.
- Because it is wild west, the transaction pool must only check static and cheap things.
- The block author won't trust the pool validation, and re-execute all checks.
- Shower Thought: Transaction queue validation is part of the runtime, but not part of the STF.

---

## Additional Resources

> Check speaker notes (click "s" 😉)

<img width="300px" rounded src="../../../assets/img/5-Substrate/thats_all_folks.png" />

Notes:

https://github.com/paritytech/polkadot-sdk/blob/bc53b9a03a742f8b658806a01a7bf853cb9a86cd/substrate/client/transaction-pool/src/graph/ready.rs#L149

Original pool PR from ages ago, old but gold: https://github.com/paritytech/substrate/issues/728

> Work towards a flexible transaction queue that relies **only on runtime logic to provide comprehensive dependency and queuing management**... should not be aware of the concepts of accounts, signatures, indexes or nonces.

> Returns `Valid` if the transaction can be **statically** validated; ... the u64 is the priority used to determine which of a mutually exclusive set of transactions are better to include... Any transactions that do get included in a block should be instantly discarded (and banned) if they result in a panic execution.

### Post Lecture

More about MEV

---

# Appendix

---

## Transaction Pool Submission API

```rust
pub enum TransactionStatus<Hash, BlockHash> {
	/// Transaction is part of the future queue.
	Future,
	/// Transaction is part of the ready queue.
	Ready,
	/// The transaction has been broadcast to the given peers.
	Broadcast(Vec<String>),
	/// Transaction has been included in block with given hash.
	InBlock(BlockHash),
	/// The block this transaction was included in has been retracted.
	Retracted(BlockHash),
	/// Maximum number of finality watchers has been reached,
	/// old watchers are being removed.
	FinalityTimeout(BlockHash),
	/// Transaction has been finalized by a finality-gadget, e.g GRANDPA
	Finalized(BlockHash),
	/// Transaction has been replaced in the pool, by another transaction
	/// that provides the same tags. (e.g. same (sender, nonce)).
	Usurped(Hash),
	/// Transaction has been dropped from the pool because of the limit.
	Dropped,
	/// Transaction is no longer valid in the current state.
	Invalid,
}
```
