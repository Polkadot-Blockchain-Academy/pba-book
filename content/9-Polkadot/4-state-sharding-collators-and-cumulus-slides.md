---
title: State Sharding (Cumulus & Collators)
description: Cumulus, architecture and function. Part 1/3 of parachains protocol
duration: 1 hour
owner: Maciej Zyszkiewicz (Bradley Olson originally)
---

# State Sharding

## Collators and Cumulus

Notes:
Today we'll be covering State Sharding in Polkadot. To understand it we'll be diving deeper into collator nodes as well as a Polkadot-sdk tool called cumulus.

---

# State Sharding

## Actors

<pba-cols>
<pba-col>
  <ul>
    <li>Collators</li>
  </ul>
</pba-col>
<pba-col>
<img style="width: 400px" src="./img/actors.png" />
</pba-col>
</pba-cols>

Notes:
Our actors here are collators which are parachain nodes and validators which are relay chain elected nodes.

---

# State Sharding

## Agenda

- Elves Overview
- State
- State Shards
- Collator Networks
- Collations
- Cumulus

---

# ELVES

<pba-flex center>

1. **Collation: Collect transactions.**
1. Backing: Assign responsibility.
1. Availability: Preserve data.
1. Approval Checking: Verify correctness.
1. Disputes: Resolve escalations.

</pba-flex>

Notes:
So today's lecture is part of a 3 lecture sequence describing the ELVES protocol. ELVES is a research paper published by W3F that is describing how polkadot achieves state and execution sharding and how it does it securely.

There are 5 main subprotocols in ELVES. On this lecture we will diving into the Collation side of things. This is everything that happens on the parachain side which is a single Polkadot shard or rollup.

---

# State Sharding

## What is State?

- Status of all the accounts, balances, variables in a blockchain
- [...]
- [...]

Notes:
First a quick reminder to bring everyone up to speed. What is tate in a blockchain context?

Blockchains were often described as big ledgers. This analogy was pretty dismissive of the execution layer of blockchains but it was pretty accurate when it comes to the state layer.

State is all aggregated status of all the data, variables, balances, accounts, smart contracts etc in the blockchain system. If Alice has 10 tokens on her account then this information is a part of the state. State is ofc more than balances. All the blockchain programmables bits, like wasm runtimes, smart contracts etc are also in the state. They are blobs we can interact with.

---v

# State Sharding

## What is State?

- Status of all the accounts, balances, variables in a blockchain
- State is modified by transactions/extrinsics
- [...]

Notes:
State can also be modified by TXs and extrinsics. So if for instance Alice sends 5 DOT to Bob it modifies the bit of state that corresponds to Alice, subtracts 5 from it and moves the 5 to Bob's State. Transactions are effectively operations on the state, they allow us to modify it. Hence we often call executing them state transitions.

---v

# State Sharding

## What is State?

- Status of all the accounts, balances, variables in a blockchain
- State is modified by transactions/extrinsics
- State is usually stored by ALL nodes that participate in consensus

Notes:
That is also why everyone needs to keep track of all the state. Because to apply the transactions we need to have the relevant state. So in most blockchains and I really mean nearly ALL of them. The state is simply and naively just duplicated between ALL the nodes in the network.

---

# (No) State Sharding

<pba-cols>
<pba-col>
  <ul>
    <li>Imagine no state sharding</li>
    <li>All nodes need to store all the state</li>
    <li>O(n^2) storage complexity</li>
  </ul>
</pba-col>
<pba-col>
<img style="width: 400px" src="./img/state.png" />
</pba-col>
</pba-cols>

Notes:
So in a world with no state sharding all the nodes store all the data entries.

---v

# (No) State Sharding

<pba-cols>
<pba-col>
  <ul>
    <li>Imagine no state sharding</li>
    <li>All nodes need to store all the state</li>
    <li>O(n^2) storage complexity</li>
  </ul>
</pba-col>
<pba-col>
<img style="width: 600px" src="./img/fully_connected_7_nodes.png" />
</pba-col>
</pba-cols>

Notes:
if we have a thousand items in the state and want to have a thousand nodes. That is already a milion things to store across the whole network. It is effectively quadratic in storage complexity.

---v

# (No) State Sharding

<pba-cols>
<pba-col>
  <ul>
    <li>Imagine no state sharding</li>
    <li>All nodes need to store all the state</li>
    <li>O(n^2) storage complexity</li>
  </ul>
</pba-col>
<pba-col>
<img style="width: 600px" src="./img/fully_connected_20_nodes.png" />
</pba-col>
</pba-cols>

Notes:
If we would have a million items to store and a thousand nodes then we already need over a billion of items stored across the network. This is not sustainable and it can lead to centralisation if it imposes higher and higher hardware requirements.

---

# State Sharding

## Data replication

<pba-cols>
<pba-col>
  <ul>
    <li>Data replication is good...</li>
    <li>but only in moderation</li>
  </ul>
</pba-col>
<pba-col>
<img style="width: 400px" src="./img/state.png" />
</pba-col>
</pba-cols>

Notes:
But data replication is not inherently bad. It is in fact really really good, it provides decentralisation and safety so we don't want to get rid of it. But just by limiting data replication we can achieve simillar security guarantees AND drastically reduce the storage overhead.

---v

# State Sharding

<pba-cols>
<pba-col>
  <ul>
    <li>State is split into multiple shards (rollups/parachains)</li>
    <li>Collators store and manage parachain state</li>
    <li>Validators do NOT store parachain state</li>
    <li>Validators only store hashes of parachain state and their runtimes</li>
  </ul>
</pba-col>
<pba-col>
<img style="width: 400px" src="./img/state.drawio.svg" />
</pba-col>
</pba-cols>

Notes:
The polkadot network instead of storing it's state in a single monolithic storage decided to split and silo it's storage.
And this is partially where parachains even come from. Each small silo for a state is their own little blockchain that has it's own sovereign state.

The nodes of each parachain AKA collators are responsible with storing and managing the parachain state. Relay chain validators only store a hash of the parachain state.

Additionally the relay chain also has some state in it, but the general direction is that we want to minimize the state on the relay chain. The relay chain should be completely transactionless and generally unavailable to the end-users. It is a meta-layer to be used by rollups and people. This initiative is often called the Minimal Relay Chain.

---v

# State Sharding

<pba-cols>
<pba-col>
  <ul>
    <li>Relay chain stores the minimum allowed amount of state</li>
    <li>Ideally there are no transactions at all on the relay chain</li>
  </ul>
</pba-col>
<pba-col>
<img style="width: 400px" src="./img/state.drawio.svg" />
</pba-col>
</pba-cols>

Notes:
Additionally the relay chain also has some state in it, but the general direction is that we want to minimize the state on the relay chain. The relay chain should be completely transactionless and generally unavailable to the end-users. It is a meta-layer to be used by rollups and people. This initiative is often called the Minimal Relay Chain.

---

# State Sharding

## Parachains

- Parachain Runtime (WASM STF)
- Collator Network
- State

Notes:
Now that we've mentioned parachains again it might be time to talk about them a bit more.
Parachains are generally just sovereign blockchain networks. Most of them are built with Substrate and Frame. Each parachain features a distinct and custom built wasm runtime (that is the state transition funciton), a collator network and a sovereign parachain state over which they preside.

---v

# State Sharding

## Collator Networks

- Choose your own block production engine (usually Aura)
- Customise your collator incentives
- Permissioned vs permissionless?

Notes:
How exactly a collator network should look like is left nearly completely up to the specific parachain and its governance.

The network can decide on things like, what block parachain block authoring to use. Most default to Aura since most of the security will come from the relay chain at later stages anyway.

Parachains also control how to incentivise collators. Some of them run their own staking systems where collators get rewarded for their work. Others prefer it keep things more centralised and under control. Substrate and cumulus allows you to pick from a wide variety of options, everything from a centrally controlled chain run on beefy collators to, decetralised chains is on the table.

---v

# State Sharding

## Collator Duties

- Store parachain state
- Accept transactions to TX pool
- Build parachain blocks (a collation of transactions)
- Send the collations to the relay chain validators

Notes:
But the one thing all of them need to do no matter what consenus, incentivisation or topology they use is to eventually keep building parachain blocks. Collators are most importantly collecting aggregating and sequencing all parachain transactions into parachain blocks. We often call parachain blocks parablocks. The parablock together with some extra data is sent over to the relay chain.

---

# State Sharding

## Collation Validation

Notes:
And why do collations get sent over to the relay chain? To get validated of course. The parachains use the rely chain as an auditing layer to inherit its economic security. How relay chain achieves consensus on the validity of collations is a topic for next the next 2 lectures but let's take a look how an singular collation could be verified by a singular other node. No consensus yet, just a local check.

---v

# State Sharding

## Collation Validation

<pba-flex center>

- The relay chain ensures that every parachain block follows the rules defined by that parachain's current code.

<!-- .element: class="fragment" data-fragment-index="1" -->

- Constraint: The relay chain validators must be able to execute runtime validation of a parachain block without access to the entirety of that parachain's state

<!-- .element: class="fragment" data-fragment-index="2" -->

</pba-flex>

<div class="r-stack">
<img src="assets/cumulus-deep-dive/runtime_validation_1.svg" style="width: 60%" />
<img src="assets/cumulus-deep-dive/runtime_validation_2.svg" style="width: 60%" />
<!-- .element: class="fragment" data-fragment-index="1" -->
<img src="assets/cumulus-deep-dive/runtime_validation_3.svg" style="width: 60%" />
<!-- .element: class="fragment" data-fragment-index="2" -->
</div>

<pba-flex center>

</pba-flex>

Notes:
Well a parachain block is just a bunch of transactions altering the state.

SWITCH

Here we can see a state transition occuring as a consequence of the transactions.

So if someone has access to the previous state, and appplies the the transactions using the WASM runtime which is the state transition function then they receive the output state. If all the transitions are legal then the resulting state is also legal.

SWITCH

But remember that relay chain validators do NOT store parachain state. So imagine telling your bank to send 100 dollars to Bob, but the bank does not even know how much money you have. Is that a legal transaction? Hard to say we dont know how much you have.

So to answer how we deal with it let's dive into the code and do some inspecting.

---v

# State Sharding

## Collation

```rust[1|8-9|10-11]
pub struct Collation<BlockNumber = polkadot_primitives::BlockNumber> {
	/// Messages destined to be interpreted by the Relay chain itself.
	pub upward_messages: UpwardMessages,
	/// The horizontal messages sent by the parachain.
	pub horizontal_messages: HorizontalMessages,
	/// New validation code.
	pub new_validation_code: Option<ValidationCode>,
	/// The head-data produced as a result of execution.
	pub head_data: HeadData,
	/// Proof of storage to verify the state transition of the parachain.
	pub proof_of_validity: MaybeCompressedPoV,
	/// The number of messages processed from the DMQ.
	pub processed_downward_messages: u32,
	/// The mark which specifies the block number up to which all inbound HRMP messages are processed.
	pub hrmp_watermark: BlockNumber,
}
```

Notes:
Here we have the actual struct used to represent a collation. There is a lot of stuff here but dont worry.

SWITCH

Right now we only care about a few items here. First is head_data. That is the promise of what we should get when we execute the parachain block.

SWITCH

Most importantly there is something called a proof of validity. If we'd inspect this field further eventually we would get here:

---v

# State Sharding

## Collation

```rust [6|9-10|11-12]
/// The parachain block that is created by a collator.
///
/// This is send as PoV (proof of validity block) to the relay-chain validators. There it will be
/// passed to the parachain validation Wasm blob to be validated.
#[derive(codec::Encode, codec::Decode, Clone)]
pub struct ParachainBlockData<B: BlockT> {
    /// The header of the parachain block.
    header: B::Header,
    /// The extrinsics of the parachain block.
    extrinsics: alloc::vec::Vec<B::Extrinsic>,
    /// The data that is required to emulate the storage accesses executed by all extrinsics.
    storage_proof: sp_trie::CompactProof,
}
```

Notes:
To the Parachain block data which is the decoded PoV.

Switch

And as expected we have a list of all transactions here, that's in the extrinsics field

Switch

but we also have a storage proof. Hmmm what's that?

---v

# State Sharding

## Collation Storage Proof

- Makes up most of the information in a collation
- Acts as a replacement for the parachain's pre-state for the purpose of validating a single block
<!-- .element: class="fragment" data-fragment-index="1" -->
- It enables the construction of a sparse in-memory merkle trie
<!-- .element: class="fragment" data-fragment-index="2" -->
- State root can then be compared to that from parent header
<!-- .element: class="fragment" data-fragment-index="3" -->

Notes:
So that field contatins partial parachain state. All the fields being touched by executions we are validating, so read or modified need to be included in the storage proof.

Storage proof in fact makes up most of the collation. Extrinsic calls are usually quite light.

With the storage proof we can reconstruct a small part of the state that is relevant to our current state transition function and ignore other bits. But of course we cannot just trust collators so all the state items are provided as a merkle proof that we compare to the state root from the parent block.

---v

# State Sharding

## Collation Storage Proof Creation

<div class="r-stack">
<img src="assets/cumulus-deep-dive/pov_witness_data_1.svg" style="width: 70%" />
<!-- .element: class="fragment fade-out" data-fragment-index="1" -->
<img src="assets/cumulus-deep-dive/pov_witness_data_2.svg" style="width: 70%" />
<!-- .element: class="fragment" data-fragment-index="1" -->
</div>

<br/>

- Only includes the data modified in this block along with hashes of the data from the rest of the trie
<!-- .element: class="fragment" data-fragment-index="2" -->
- This makes up the majority of the data in a collation (max 10MB)
<!-- .element: class="fragment" data-fragment-index="3" -->

Notes:
Let's see an example. Imagine this is our state merkle trie. And imagine Alice from A sends some funds to Bob and Charlie.

The other leafs are also state items but we neither read them or modify them so they are not needed to verify the transactions.

SWITCH

But we still need the hashes of them to have a proper merkle proof. So we need to supply the hashes in the green nodes which together with our data will fully reconstruct the blue node which is the state merkle root. The red leafs were omitted. So we are saving a lot of bandwitdh by not sending them. Usually we only touch a small subset for the whle state sp the savings are quite big.

- orange: Data values modified in this block
- green: Hash of the siblings node required for the pov
- white: Hash of the nodes that are constructed with orange and green nodes
- red: Unneeded hash
- blue: Head of the trie, hash present in the previous block header

---

# State Sharding

## Parachain STF

- The current STF of each Parachain is stored on the Relay Chain, wrapped as a PVF
- New state transitions that occur on a parachain must be validated against the PVF

```rust [6]
/// A struct that carries code of a parachain validation function and its hash.
///
/// Should be cheap to clone.
#[derive(Clone)]
pub struct Pvf {
    pub(crate) code: Arc<Vec<u8>>,
    pub(crate) code_hash: ValidationCodeHash,
}
```

<br/>

- PVF ≈ STF = Parachain Runtime = WASM Blob = Parachain's Code

Notes:
Luckily at least the State transition function is stored on the relay chain. From the perspective of the relay chain it is often called PVF - Parachain Validation Function but that is effectively just a wrapper over the state transition function. Reminder that state transition function is just the WASM blob, the parachain runtime or parachain code. We are terrible at naming things, I know.

---v

#### Why PVF Rather than STF?

<pba-cols>
<pba-col center>

<img src="assets/cumulus-deep-dive/cumulus_sketch_4.svg" width = "100%"/>

</pba-col>
<pba-col center>

- The PVF is not just a copy paste of the parachain Runtime

<br/>

- The PVF contains an extra function, `validate_block`

<br/>

**WHY!?**

<!-- .element: class="fragment" data-fragment-index="1" -->

</pba-col>
</pba-cols>

Notes:

The function `validate_block` needed to interpret all the extra information in a PoV required for validation.
This extra information is unique to each parachain and opaque to the relay chain.

---v

#### Validation Path Visualized

<div class="r-stack">
<img src="assets/cumulus-deep-dive/collation_path_1.svg" style="width: 70%" />
<img src="assets/cumulus-deep-dive/collation_path_2.svg" style="width: 70%" />
</div>

Notes:
The input of the runtime validation process is the PoV, and the function called in the PVF is 'validate_block'. Validate block converts the PoV into necessary inputs on top of which a parachain's STF can be run. The output created is called a CandidateReceipt.

---v

#### What Does validate_block Actually Do?

<pba-flex center>

- The parachain runtime expects to run in conjunction with a parachain client
- But validation is occurring in a relay chain node
<!-- .element: class="fragment" data-fragment-index="1" -->
- We need to implement the API the parachain client exposes to the runtime, known as host functions
<!-- .element: class="fragment" data-fragment-index="2" -->
- The host functions most importantly allow the runtime to query its state, so we need a light weight replacement for the parachain's state sufficient for the execution of this single block
<!-- .element: class="fragment" data-fragment-index="3" -->
- validate_block prepares said state and host functions, then runs the parachain's STF on top of them
<!-- .element: class="fragment" data-fragment-index="4" -->

</pba-flex>

---v

#### Validate Block in Code

```rust [2|3-4|6|8-11|14]
// Very simplified
fn validate_block(input: InputParams) -> Output {
    // First let's initialize the state
    let state = input.storage_proof.into_state().expect("Storage proof invalid");

    replace_host_functions();

    // Run Substrate's `execute_block` on top of the state
    with_state(state, || {
        execute_block(input.block).expect("Block is invalid")
    })

    // Create the output of the result
    create_output()
}
```

Notes:

1. We construct the sparse in-memory database from the storage proof and then ensure that the storage root matches the storage root in the `parent_head`.
2. Replace host functions
3. Execute block
4. Create output. We check whether the `storage_root` and other outputs resulting from validation matched the commitments made by the collator

---v

##### Host Function Replacement Visualized

<div class="r-stack">
<img src="assets/cumulus-deep-dive/replace_host_function_1.svg" style="width: 70%" />
<!-- .element: class="fragment fade-out" data-fragment-index="1" -->
<img src="assets/cumulus-deep-dive/replace_host_function_2.svg" style="width: 70%" />
<!-- .element: class="fragment" data-fragment-index="1" -->
</div>

---v

# State Sharding

## All Together

<img style="width: 900px" src="./assets/execution-sharding/parachain-validation-multiple.svg" />

---

# State Sharding

## Solochains disclaimer

Notes:
So you all learned substrate and frame. You can all build substrate chains. And I want to give you a reminder that Substrate is not opinionated about building just for Polkadot. Substrate chains can exist as completely independent blockchains where they try to ensure their own economic security. Substrate chains dont have to be parachains, but if they would like to be, then the transition is super easy

and this is where cumulus comes in.

---

# State Sharding

## Cumulus

Notes:
So we've discussed all the duties of collators but we ofc dont expect every parachain to reimplement them from scratch. Most of that come built in with Cumulus.

---v

# State Sharding

## Cumulus

<img style="width: 1200px" src="./img/glue.drawio.png" />

Notes:
Cumulus is an extension tool to Substrate. It is a bunch of libraries, scripts and tools that can take in your classic substrate chain and turn it into a polkadot compatible parachain. Cumulus is like the glue between substrate parachains and the Polkadot relay chain.

So most teams working on parachains in Polkadot develop their parachain purely in substrate just like a solo-chain but then they use cumulus to do the heavy-lifting of integrating with the relay chain.

---v

# State Sharding

## Cumulus' Key Processes

- Follow relay relay chain progression
- Collation generation and announcement
- Request parablocks not shared by peers from relay (data recovery)

Notes:
We discussed a lot of the responsibilities of collators but those are in fact mostly handled by Cumulus. Cumulus has triggers and hooks for building parachain blocks and it also has the integrations to announce them to the relay chain. Additionally Cumulus follows the relay chain to notice any relay chain forks or to find the new best relay chain head.

---v

## Cumulus and Parachain Runtime Upgrades

<pba-flex center>

- Every Substrate blockchain supports runtime upgrades
<!-- .element: class="fragment" data-fragment-index="0" -->
- Every time a validator wants to validate a parablock, it must first compile the PVF
<!-- .element: class="fragment" data-fragment-index="1" -->

##### Problem

<!-- .element: class="fragment" data-fragment-index="2" -->

- What happens if PVF compilation takes too long?
  <!-- .element: class="fragment" data-fragment-index="2" -->
  - Approval no-shows
  - In disputes neither side may reach super-majority

<!-- .element: class="fragment" data-fragment-index="2" -->

> Updating a Parachain runtime is not as easy as updating a standalone blockchain runtime

<!-- .element: class="fragment" data-fragment-index="3" -->

</pba-flex>

---v

### Solution

The relay chain needs a fairly hard guarantee that PVFs can be compiled within a reasonable amount of time.

<br/>

- Collators execute a runtime upgrade but it is not applied
- Code sent in collation `Option<ValidationCode>`
- The relay chain executes the **PVF Pre-Checking Process**
- The first parachain block to be included after the end of the process applies the new runtime

<!-- .element: class="fragment" data-fragment-index="1" -->

> Cumulus follows the relay chain, waiting for a go ahead signal to apply the runtime change

<!-- .element: class="fragment" data-fragment-index="2" -->

Notes:

https://github.com/paritytech/cumulus/blob/master/docs/overview.md#runtime-upgrade

---v

##### PVF Pre-Checking Process

<pba-flex center>

- Track
- Check
<!-- .element: class="fragment" data-fragment-index="1" -->
- Vote
<!-- .element: class="fragment" data-fragment-index="2" -->
- Conclude
<!-- .element: class="fragment" data-fragment-index="3" -->
- Upgrade
<!-- .element: class="fragment" data-fragment-index="4" -->
- Notify
<!-- .element: class="fragment" data-fragment-index="5" -->

</pba-flex>

Notes:

- The relay chain keeps track of all the new PVFs that need to be checked
- Each validator checks if the compilation of a PVF is valid and does not require too much time, then it votes
  - binary vote: accept or reject
- Super majority concludes the vote
- The new PVF replaces the prior one in relay chain state
- A "go ahead" signal is sent, telling the parachain to apply the upgrade

reference: https://paritytech.github.io/polkadot/book/pvf-prechecking.html

---

# State Sharding

## Summary

- State is divided into shards called parachains
- Shard state is managed by the collators
- Collators aggregate parachain TXs and build parachain blocks
- Parachain blocks together with state proofs get sent to validators
- Cumulus transforms Substrate Chains into Parachains

---

## References

1. 🦸 [Bradley Olson](https://github.com/bradleyolson64) original lecturer
1. 🦸 [Gabriele Miotti](https://github.com/gabriele-0201), who was a huge help putting together these slides
1. https://github.com/paritytech/cumulus/blob/master/docs/overview.md

---

<!-- .slide: data-background-color="#4A2439" -->

# Questions
