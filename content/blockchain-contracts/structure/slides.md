---
title: Blockchain Structure
description: The Blockchain data structure including hash-linking, forks, header vs body, and extrinsics.
duration: 30 min
---

# Blockchain Structure

<img style="width: 1000px" src="./img/opaque-blockchain.svg" />

---

## Shared Story

A Blockchain **cryptographically guarantees** that a history of events has not been tampered with.
This allows interested parties to have a **shared history**.

Notes:

And it allows them to know whether they have identical histories in O(1) by just comparing the tip of the chain.

---

## Hash Linked List

<img style="width: 1000px" src="./img/hash-linked-1.svg" />

Notes:

This is a simplified blockchain.
Each block has a pointer to the parent block as well as a payload.

---v

## Hash Linked List

<img style="width: 1000px" src="./img/hash-linked-2.svg" />

Notes:

The pointer is a cryptographic hash of the parent block.
This ensures data integrity throughout the entire history of the chain.
This is the simplest form that a blockchain could take and indeed it allows us to agree on a shared history.

---v

## Hash Linked List

<img style="width: 1000px" src="./img/hash-linked-3.svg" />

Notes:

This ensures data integrity throughout the entire history of the chain.
This is the simplest form that a blockchain could take and indeed it allows us to agree on a shared history.

---v

### Genesis Block

<img style="width: 1000px" src="./img/hash-linked-genesis.svg" />

Notes:

The first block in the chain is typically called a the "Genesis block" named after the first book in the judaeo-christian mythology - The beginning of our shared story.
The parent hash is chosen to be some specific value.
Typically we use the all-zero hash, although any fixed widely agreed-upon value would also do.

---

## State Machines (Again)

A state machine defines:

<pba-flex center>

- Set of valid states
- Rules for transitioning between states

</pba-flex>

<img style="width: 900px;" src="./img/state-machine-general.svg" />

---v

### Blockchain meet State Machine

<img style="width: 1000px" src="./img/blockchain-meet-state-machine.svg" />

Notes:

The simplest way to join a blockchain to a state machine is to to make the blockchain's payload a state machine transition.
By doing so, we effectively track the history of a state machine in a cryptographically guaranteed way.

---v

### Where do the States Live?

Somewhere else!

<img style="width: 1000px" src="./img/blockchain-with-state-outside.svg" />

Notes:

There is a state associated with each block.
But typically the state is NOT stored in the block.
This state information is redundant because it can always be obtained by just re-executing the history of the transitions.
It is possible to store the state in the blocks, but the redundancy is undesirable.
It wastes disk space for anyone who wants to store the history of the chain.
Storing the state in the block is not done by any moderately popular blockchain today.
If you _want_ to store the states, you are welcome to do so.
Software that does this is known as an Archive node or an indexer.
But it is stored separately from the block
...Pause...
One more time to make sure it sinks in: The state is NOT in the block.

---v

### State Roots

A cryptographic anchor to the state

<img style="width: 1000px" src="./img/blockchain-with-state-roots.svg" />

Notes:

Some data redundancy can be good to help avoid corruption etc.
It is common for a block to contain a cryptographic fingerprint of
the state.
This is known as a state root.
You think of it as a hash of the state.
In practice, the state is typically built into a Merkle tree like structure and the tree root is included.
Not all blockchains do this.
Notably bitcoin doesn't.
But most do.
We'll go into details about exactly how this state root is calculated for Substrate in the next two modules, but for now we just consider the state root to be some kind of cryptographic fingerprint.

---

## Forks

<img style="width: 1000px" src="./img/forks.svg" />

A state machine can have different possible histories.
These are called forks.

Notes:

You can think of them like alternate realities.
We need to decide which of the many possible forks is ultimately the "real" one.
This is the core job of consensus and we will talk about it in two upcoming lessons in this module.

---v

## Invalid Transitions

<img style="width: 1000px" src="./img/forks-some-invalid.svg" />

Notes:

Before we even get to hardcore consensus, we can rule out _some_ possibilities based on the state machine itself

---

## Realistic Blockchain Structure

<img width="600px" src="./img/header-body.svg" />

- Header: Summary of minimal important information about this block
- Body: A batched list of state transitions

Notes:

The header is a minimal amount of information.
In some ways it is like metadata.
The body contains the real "payload".
It is almost always a batch of state transitions.
There are many name aliases for what is included in the body:

- Transitions
- Transactions
- Extrinsics

---v

## Blocks in Substrate

```rust
/// Abstraction over a Substrate block.
pub struct Block<Header, Extrinsic: MaybeSerialize> {
	/// The block header.
	pub header: Header,
	/// The accompanying extrinsics.
	pub extrinsics: Vec<Extrinsic>,
}
```

Notes:

This example is from Substrate and as such it strives to be a general and flexible format, we will cover Substrate in more depth in the next module.
This is representative of nearly all real-world blockchains

---

## Headers

Exact content varies per blockchain.
Always contains the parent hash.
Headers are the _actual_ hash-linked list, not entire blocks.

Notes:

The parent hash links blocks together (cryptographically linked list).
The other info is handy for other infrastructure and applications (more on that later).

---v

## Header Examples

<pba-cols>
<pba-col>

<pba-flex center>

**Bitcoin**

</pba-flex>

- Version
- Previous Hash
- Tx Merkle Root
- Time
- N_Bits
- Nonce

</pba-col>
<pba-col>

<pba-flex center>

**Ethereum**

</pba-flex>

- Time
- Block Number
- Base Fee
- Difficulty
- Mix Hash
- Parent Hash
- State Root
- Nonce

</pba-col>
</pba-cols>

---v

## Substrate Header

- Parent hash
- Number
- State root
- Extrinsics root
- Consensus Digest

Notes:

Extrinsics root is a crypto link to the body of the block.
It is very similar to the state root.
Consensus Digest is information necessary for the consensus algorithm to determine a block's validity.
It varies widely with the consensus algorithm used and we will discuss it in two upcoming lectures.

---v

## Substrate Header (Full Picture)

<img style="width: 1000px" src="./img/headers-link-state-body.svg" />

---

## Extrinsics

Packets from the outside world with _zero_ or more signatures attached.

- Function calls to the STF
- Some functions require signatures (e.g., transfer some tokens)
- Others don't, but usually have some validation means

---

## DAGS

**Directed Acyclic Graphs**

<img style="margin-left: 215px;" src="./img/forks.svg" />

Notes:

In math there is a notion of a Directed Acyclic Graph.
Define graph, than directed, than acyclic.
Blockchains are examples of DAGs.
Actually blockchains are a specific kind of a DAG called a tree.
Sometimes you will hear me talk about the "block tree" which really means all the histories of the chain.

But there are more kinds of DAGs than just trees.
Consider if someone authored a block that looks like this.

CLICK

---v

## DAGS

**Directed Acyclic Graphs**

<img style="width: 1000px" src="./img/dag.svg" />

Notes:

What if a block could have more than one parent!?
It could allow parallelization and increased throughput!
But it also leads to problems.
What if there are conflicting transactions in the two parent histories?
How do you even know if there are conflicting histories?

---

<!-- FIXME TODO

This might be a good place to split the lesson.
The part before this is about a data structure.
The part after this is about a P2P network of nodes that track this data structure.
-->

## Blockchain 💒 P2P Networks

<img style="width: 900px;" src="./img/blockchain_p2p.svg" />

Notes:

So hopefully some parts of this figure look familiar.
What do you see here?

- Diverse servers.
- In a p2p network.
- Each with their own view of the blockchain.

---v

## Nodes

Software agents that participate in blockchain network.<br />
May perform these jobs:

<pba-cols>
<pba-col>
<pba-flex center>

- Gossip blocks
- Execute and Validate blocks
- Store blocks
- Store states
- Gossip transactions

</pba-flex>
</pba-col>
<pba-col>
<pba-flex center>

- Maintain a transaction pool
- Author blocks
- Store block headers
- Answer user requests for data (RPC)

</pba-flex>
</pba-col>
</pba-cols>
Notes:

Many nodes only perform a subset of these tasks

---v

## Types of Nodes

<pba-flex center>

- Full Nodes
- Light Nodes (aka Light clients)
- Authoring nodes
- Archive nodes
- RPC nodes

</pba-flex>

---

## Blockspace

A resource created, and often sold, by a decentralized blockchain network.

<img style="width: 700px;" src="./img/Web2Web3Stacks.png" />

#### Learn more:

- Article: <https://a16zcrypto.com/posts/article/blockspace-explained/>
- Article: <https://www.rob.tech/polkadot-blockspace-over-blockchains/>
- Podcast: <https://www.youtube.com/watch?t=5330&v=jezH_7qEk50>

Notes:

A Blockchain network is a replacement for a centralized server.
It sells a product to application deployers.
The state machine is the application layer, and the blockchain is the server replacement.
In the same way that applications pay data centers for server resources like cpu time, disk space, bandwidth etc.
Applications (maybe via their developers or users) pay for the privilege of having their history attested to and their state tracked by a trustless unstoppable consensus layer.

---v

## Transaction Pool

- Contains transactions that are not yet in blocks.
- Constantly prioritizing and re-prioritizing transactions.
- Operates as a blockspace market.

Notes:

Sometimes known as mempool (thanks bitcoin 🙄)
Authoring nodes determine the order of upcoming transactions.
In some sense they can see the future.

Foreshadow forks where players disagree on the rules
History: dao fork bch fork
foreshadow consensus: arbitrary additional constraints for a block to be valid

---

# Let's #BUIDL It
