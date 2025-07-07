---
title: Data Availability (Backing & Availability)
description: Data Availability Problem, Erasure coding, Data sharding.  Part 2/3 of parachains protocol
duration: 1h
owner: Maciej Zyszkiewicz (Bradley Olson originally)
---

# Data Availability

## Backing & Availability

---

# Data Availability

## Actors

<pba-cols>
<pba-col>
  <ul>
    <li>Collators</li>
    <li>Validators</li>
  </ul>
</pba-col>
<pba-col>
<img style="width: 400px" src="./img/actors.png" />
</pba-col>
</pba-cols>

---

# Data Availability

## Agenda

- Assignment
- Backing
- Availability
- Erasure Coding

---

# ELVES

<pba-flex center>

1. Collation: Collect transactions.
1. **Backing: Assign responsibility.**
1. **Availability: Preserve data.**
1. Approval Checking: Verify correctness.
1. Disputes: Resolve escalations.

</pba-flex>

Notes:
Last time we looked into collators and how they are producing collations. At the end they had to send them over to the relay chain. But what does that mean? To who exactly?

---

# Assignment

Notes:
To answer that question we'll look into a procedure I call an assignment or backing assignment.

---v

# Assignment

## Active Validators

<img style="width: 500px" src="./assets/execution-sharding/polkadot-components.svg"/>

**Active validators** are given to the parachain protocol by the NPoS election subsystem.

Notes:
Assignment will be operating only **Active validators**. They are given to the parachain protocol by the NPoS election subsystem. Currently there are 600 validators in the active set in Polkadot and a new active set get's elected every era which is 24h in Polkadot. For those 24h those validators are the core players in the Polkadot game. Others are temporarily sitting on the bench.

---v

# Assignment

## Backing Groups

<img rounded style="width: 1100px" src="./assets/execution-sharding/validator-groups.png" />

Validators are divided into small **Backing Groups**.

Notes:
Every Session (4 hours in Polkadot), the parachains consensus takes in those active validators and _partitions_ them into small **groups** which work together. Those groups are currently around 5 validators each in Polkadot and Kusama. Those groups are called backing groups.

This is only a teaser but backing groups are mapping 1 to 1 to Polkadot **Execution Core**s, and these assignments are rotated around every few blocks. Currently a parachain is connected to a specific execution core, think of it as a core that executes parachain blocks.

---v

# Assignment

## Execution Cores

<img rounded style="width: 500px" src="./assets/execution-sharding/polkadot-architecture-simple.png" />

Each backing group corresponds to a **Polkadot Execution Core**.
One execution core can handle a single parachain block every 6s.

Notes:
In that map the white ovals are the validators. They are grouped into backing groups of size and each group is assigned to a specific execution core - that's the symbolic black square on the purple relay chain ring. Each execution core is capable of serving a single parachain or to be more precise a single parachain block.

---v

# Assignment

## Rotations

<img rounded style="width: 900px" src="./assets/execution-sharding/pairing_backing_groups_with_cores.svg" />

Once every few blocks the backing groups **rotate** around the execution cores.

Notes:
Once every few blocks the backing groups **rotate** around and they change the execution core they are serving. This makes it so even if a backing group is full of malicious they cannot wholly block a specific execution core for too long.

---v

# Assignment

## Para <-> Execution Core

Parachains or tasks get mapped to specific executions cores

More on this in the scheduling lecture (Agile Coretime)

Notes:
Each parachain, parathread or a task in the polkadot ecosystem gets matched with a speciifc execution core when it can produce a block.
We will explore this process in depth in the scheduling lecture.

---v

# Assignment

## Collator Interaction

Collators once they build the parablock need to check who are the validators that are currently assigned to their execution core and send the collations to them over the p2p network.

Notes:
Collators once they build the parablock (often referred to as candidate parablocks or just candidates) need to check who are the validators that are currently assigned to their execution core and send the collations to them.

---

# Backing

Notes:
Now we finally arrive at Backing. We just sent some collations to the validators in our backing group. What happens next?

---v

## Backing - Backers

<img rounded style="width: 700px" src="./assets/execution-sharding/polkadot-architecture-simple.png" />

Validators in the backing group are often called backers.

Notes:
Validators in the backing group are often called backers for those parablocks that are coming in to them from the collators. That is the group we created in the assignment step. The few validators or backers (3 on the image) receive a bunch of parachain blocks / parablocks / collations / candidates (all the same thing).

---v

## Backing - Backers

<img rounded style="width: 700px" src="./assets/execution-sharding/polkadot-architecture-simple-zoom.png" />

Notes:
We can zoom in on a specific backing group. Here we can see that backers are first point of contact to the outside world. They are the like club bouncers for the relay chain.

---v

## Backing - Checks

The backers receiving collations need to perform some initial checks to ensure their validity.

Notes:
But for them to do their job correctly backers after receiving collations need to perform some initial checks to ensure their validity. They simply cannot trust random data blobs coming from the external nodes. And to perform those checks they will be utilising the PVF - parachain validation function we introduced last lecture.
---v

## Backing - PVF definition

<img rounded style="width: 1000px" src="./assets/execution-sharding/runtime_validation_2.svg" />

> **Parachain Validation Function** (PVF) is a function which takes in the current parachain state (PoV), the promised parachain state, and the parachain state transition arguments. It re-executes the parachain logic/runtime/STF using the arguments on the current state and checks if it matches the promised state. If it does, the parachain block is valid.

Notes:
Read definition.
PVF reruns the STF in a sandbox environment to test its outputs.

---v

## Backing - STF reminder

<img style="width: 1200px" src="./assets/execution-sharding/parachain-validation.svg" />

Notes:

---v

## Backing - PVF code

From a Validator's perspective, a parachain is a WebAssembly blob which exposes the following (simplified) function:

```rust
type HeadData = Vec<u8>;

struct ValidationResult {
  /// New head data that should be included in the relay chain state.
  pub head_data: HeadData,
  // more fields, like outgoing messages, updated code, etc.
}

fn validate_block(parent: HeadData, relay_parent: RelayChainHash, pov: Vec<u8>)
  -> Result<ValidationResult, ValidationFailed>;
```

Notes:
That's a slightly simplified code example of how it might look. The validator has access to the parachain Wasm blob which is the parachain state transition logic. They also have the current state as pointed to by the parent HeadData. We only need to provide the transactions, details about the old state (merkle proof) and the new state root and both of those are located in the PoV variable - Proof of Validity.

PoV contains the elements necessary for the state transition and the resulting state so it's something the validator can easily check if it's correct or not.

---v

## Backing - PVF results

In the end the backer performing the `validate_block` knows that the transition is either correct or not.

Notes:
In the end the backer performing the `validate_block` knows that the transition is either correct or not. Actually what even can cause it to fail?

---v

### Backing - PVF failure

**Why might `validate_block` fail?**

1. `parent` or `PoV` is malformed - the implementation can't transform it from an opaque to specific representation
1. `parent` and `PoV` decode correctly but don't lead to a valid state transition
1. `PoV` is a valid block but doesn't follow from the `parent`

```rust
fn validate_block(parent: HeadData, relay_parent: RelayChainHash, pov: Vec<u8>)
  -> Result<ValidationResult, ValidationFailed>;
```

Notes:
Point 1. To verify the state transition we need the pre-state and the transition arguments. If we cannot decode them we cannot verify the transition so it fails by definition.
Point 2. Everything decodes nicely but the state we are reaching using the pre-state and those transition arguments is not allowed by the parachain logic. So it's a bad transition.
Point 3. Transitions seems legal but they are anchored to a different parent so we cannot allow recontextualizing them.

The biggest one that we are concerned about is point 2. We need to make sure that no malicious actors adds a malicious transaction that suddenly for instance pays them a million coins from an empty account. That would be an example of a bad transition.

---v

## Backing - Statements

<pba-flex center>

- Receive collation
- Validate
- Sign a statement
- Share statement with other backers

</pba-flex>

Notes:
Once a backer verifies that the parablock is correct they create and sign their backing statement. This will be a receipt, a proof that they approved this block which is crucial for...

---v

## Backing - Skin in the Game

<pba-flex center>

- The main goal of backing is to provide "skin in the game".

- Backers are agreeing that if the parablock turns out to be bad, they will lose 100% of their stake.

</pba-flex>

Notes:
Making sure that the backers put some skin in the game. Backers are validators so they have a lot of stake, generally millions of DOT, so if they make incorrect statements they are putting a lot on the table.

---v

## Backing - Skin in the Game

<pba-flex center>

- The main goal of backing is to provide "skin in the game".

- Backers are agreeing that if the parablock turns out to be bad, they will lose 100% of their stake.

- Backing on its own does not provide security, only **accountability**.

- Parablock head data as well as the backing statements are embedded into the relay chain (can be multiple parablocks)

</pba-flex>

Notes:
The main goal of backing is not immediate security but accountability. Backers are doing the checks to protect themselves from being punished. Those details will be embedded into the chain so backers can be held accountable.

---v

## Backing - Networking

<img rounded style="width: 1000px" src="./assets/execution-sharding/backing-networking.png" />

Notes:
Once a certain threshold of backers (3 of 5 in Polkadot) in the group approves the parablock it moves to the next stage. It can be broadcasted beyond it's backing group.

---v

## Backing - Onchain

Once a block author spots enough backing statements he puts them **on chain** as part of a block authoring inherent.

<img rounded style="width: 500px" src="./img/backing_onchain.png" />

---v

## Block Production - Candidate Receipts

**Candidate receipt:**

- The parachain ID.
- The collator's ID and signature.
- A hash of the parent block's candidate receipt.
- A Merkle root of the block's erasure-coded pieces.
- A Merkle root of any outgoing messages.
- A hash of the block.
- The state root of the parachain before block execution.
- The state root of the parachain after block execution.

Notes:
I mentioned that authors add a parablock into the relay chain block they are authoring. But in reality adding a whole parablock to the relay chain block is not feasible. We need a compact representation of the parablock/candidate - a candidate receipt. In it we store all the elements needed to identify the block later on and ensure that the data and transitions match to what was approved.

---

# Data Availability

Notes:
Backers accepted the responsibility for the parablock but now there's some more work to be done.

---

# (No) Data Availability

Imagine a situation where a backer discards the PoV once he succesfully backs it and no one else keeps it as well.

He already checked, why would he need to keep it?

This would be equivalent to having NO Data Availability

---v

# (No) Data Availability

## Malicious Collator Attack

- Parachains are blockchains, blocks need to reference their parents
- In the happy case collators share their collations between themselves
- What if a parachain block author withholds the collation from other collators?

Notes:
The problem is this can be severely exploited.
Parachains are blockchains so each block references a specific parent. They cannot build upon something they don't know.
Normally a parachain block author would send the collation to the relay chain for validation but he would also shares it with other collators. Other collators will build blocks on top of this parablock so they need to learn about it and import it.

---v

# (No) Data Availability

## Malicious Collator Attack

<div class="r-stack">
<img src="assets/data-availability-and-sharding/DA_Parachains_1.svg" style="width: 70%" />
<img src="assets/data-availability-and-sharding/DA_Parachains_2.svg" style="width: 70%" />
<!-- .element: class="fragment" data-fragment-index="1" -->
</div>

Notes:
So the attack scenario goes like this. The attack is selected as parachain block author.

They build the collation but they selectively send it only to the relay chain validators and ignore other collators. The relay chain will process the parablock and potentially accept and finalize it.

Other collators in that situation cannot build new blocks because they dont have the exact transactions in that blocks. What ends up on the relay chain is only a hash of them. So they cannot just read it from the relay chain.

What's the solution?

SWITCH

- Validators keep enough info for collators to reconstruct recent parachain blocks

Validators cannot just discard the collations. They need to keep them for some time.
Then in worst case this information can be requested by collators to reconstruct the missing blocks and unstuck other collators.

---

## (Centralised) Data Availability

So PoV need to be recoverable from the validators.

Can we just make backers keep them?

---v

# (Centralised) Data Availability

## Malicious Backer Attack

😬

---v

# (Centralised) Data Availability

## Malicious Backer Attack

- You can prove data being incorrect
- You cannot prove someone refusing to send you data

Notes:
The problem is we cannot trust the backers to distribute it fairly. They can just ignore honest nodes when we need they the data and this can be heavily exploited.

For the parablock to proceed in its lifecycle at one point or another others will need to verify it. To do it they need to have the necessary PoV data. So the data is not only needed for collators in emergencies it is also crucial for upcoming protocol stages.

---v

# (Centralised) Data Availability

## Malicious Backer Attack

<img src="assets/data-availability-and-sharding/DA_Relay_1.svg" style="width: 70%" />

Notes:
The exact attack goes like this.
If the malicious backer is the only entity controlling the PoVs then other nodes cannot validate it and potentially raise an alarm. And even worse the attacker can distribute it to other malicious validators while actively avoiding the honest nodes creating an illusion of safety. Honest nodes know something is suspicious because they cannot retrieve the PoVs but it's impossible to prove.

---v

# Data Availability

## Malicious Backer Attack

<img src="assets/data-availability-and-sharding/DA_Relay_2.svg" style="width: 70%" />

Notes:
If we create a data storage that is not controlled by the attacker then they cannot selectively distribute PoVs. Everyone that needs to check something can check something.

---

# (Naive) Data Availability

Naive approach would be to make every validator store every PoV.

DOES NOT SCALE

<!-- .element: class="fragment" data-fragment-index="1" -->

Notes:
We already discussed the same classic problem of data replication. This simply wouldnt scale.

---

# Data Availability Problem

How do we ensure a piece of data is retrievable without storing it on every single node forever (on-chain)?

---v

## Availability - Erasure Coding

The goal: Avoid storing full PoV in each validator

<pba-flex center>

- Encode data of K chunks into a larger code word of N chunks
<!-- .element: class="fragment" data-fragment-index="1" -->
- Any K-subset of N chunks can be used to recover the data
<!-- .element: class="fragment" data-fragment-index="2" -->

</pba-flex>

<img rounded style="width: 1000px" src="assets/data-availability-and-sharding/erasure-coding-1.png" />

Notes:

- Erasure coding allows storing only 3x PoV size vs 334x for 1000 validators

---v

## Availability - Erasure Coding

<img rounded style="width: 450px" src="./assets/execution-sharding/line.drawio.svg" />

Notes:
It all sounds complicated but trust me it isnt. Let me show you an example.
That's a line. If we have those two red points everyone agrees that there is only 1 specific line we can draw through those. Line is a 1st degree polynomial.

---v

## Availability - Erasure Coding

<img rounded style="width: 450px" src="./assets/execution-sharding/poly-2nd.drawio.svg" />

Notes:
Let's keep going further... thats a quadratic polynomial. So a second degree. We need exactly 3 points to be able to draw it exactly.

---v

## Availability - Erasure Coding

<img rounded style="width: 450px" src="./assets/execution-sharding/poly-3rd.drawio.svg" />

Notes:
You might start seeing a pattern but now that's a 3rd degree polynomial, we need exactly 4 points to draw it.

---v

## Availability - Erasure Coding

<img rounded style="width: 450px" src="./assets/execution-sharding/line-redundant.drawio.svg" />

Notes:
Now let's go back to the nice and simple line. What if they gave us 3 points from the line? We can remove any one of them and we are still able to draw the line. That's a nice property.

Imagine you have 3 friends. Each remembers a single point from the line. But remembering the whole line is too difficult for any of you. If any of you goes missing you can still recreate the line by combining the other two points and remembering a point is much easier than a line.

---v

## Availability - Erasure Coding

<img rounded style="width: 450px" src="./assets/execution-sharding/line-not-enough.drawio.svg" />

Notes:
Of course if two of your friends go missing we have a problem. We can no longer recreate the line because we don't have enough points.

---v

## Availability - Erasure Coding

- Represent the data as a high degree polynomial (very curvy curve)
- Each validator is responsible for one point (small availability chunk)
- Distribute more points than needed
- As long as enough of these points remain recoverable from validators we can recover the data
- Each point is much smaller than the whole data entry

Notes:
In Polkadot instead of a line we have the data needed to verify the parablock but we simply represent it as a very complicated curve. Each validator is responsible for a single point on that curve. And we distribute more points than needed. As long as enough of these points remain recoverable from validators we can recover the data. And that's the core idea of erasure coding. We only need 2/3rds of the points to recover the data. (2/3rds is a custom parameter)

---

## Availability Chunk Distribution

<img src="assets/data-availability-and-sharding/bitfield-chunk-req.svg" style="width: 40%" />

Notes:

- Validator i sees backing statements on chain and requests chunk i for each PoV from its associated backer
- Backers respond with chunks, or availability times out
- After backing something there is a time limit how long it can take to become available. If it fails then it times out and something else will have a chance.

---v

## Availability Statement Format: Bitfields

One structure to sign them all!

<img src="assets/data-availability-and-sharding/availability-gossip.svg" style="width: 70%" />

Notes:

- Number of bits equivalent to the number of `AvailabilityCore`s
- Bit `i` represents one validator's report as to whether it has its chunk of the PoV occupying core `i`
- Condenses a validator's perspective into a minimal structure to be signed and gossiped
- Submitted on-chain by block producers

---v

### Availability On-Chain

<img rounded style="width: 600px" src="assets/data-availability-and-sharding/availability-bitfields.png" />

Notes:

- These statements are gossiped off-chain and included in a block in a ParachainsInherent.
- Why do we need bitfields on-chain?

---v

### Availability Thresholds Visualized

<img src="assets/data-availability-and-sharding/relay-block-construction-I.svg" style="width: 60%"/>

Notes:

- Validator Y is producing a block
- Statements from validators a, f, g, and b determine availability for blocks occupying 5 cores
- Candidates 0, 3, and 4 are marked as included. Approvals start. Cores are freed to repeat process.

What is wrong with this diagram?

---v

### PoV Chunk Validation

What happens if there's a bad chunk in the reconstructed PoV?

Solution: Merkle proofs!

<!-- .element: class="fragment" data-fragment-index="1" -->

<pba-flex center>

- PoV chunks form branches of a merkle tree
<!-- .element: class="fragment" data-fragment-index="2" -->
- Proof distributed with each chunk
<!-- .element: class="fragment" data-fragment-index="3" -->
- Chunks checked against erasure_root from CandidateReceipt
<!-- .element: class="fragment" data-fragment-index="4" -->

</pba-flex>

Notes:

- Corrupted PoV -> PVF failure not attributable to backers
- Can lead to punishment of innocent parties in disputes

---

### Ongoing Work

Reed Solomon is costly, taking 14-20% of validator CPU time.

**Obvious target for optimization!**

<!-- .element: class="fragment" data-fragment-index="1" -->

<pba-flex center>

- Systemic chunks recovery
  <!-- .element: class="fragment" data-fragment-index="2" -->
      - Removes need for decoding
  <!-- .element: class="fragment" data-fragment-index="2" -->
      - Instead, re-encode to check chunk validity
  <!-- .element: class="fragment" data-fragment-index="2" -->
      - ~50% CPU time improvement
  <!-- .element: class="fragment" data-fragment-index="2" -->
- Compiler elision of array bounds checks + inlining
  <!-- .element: class="fragment" data-fragment-index="3" -->
      - ~33-50% CPU time improvement depending on unsafe Rust use
  <!-- .element: class="fragment" data-fragment-index="3" -->
- Better implemented Reed Solomon library (potential 10x improvement!)
<!-- .element: class="fragment" data-fragment-index="4" -->

</pba-flex>

Notes:
Systemic chunks recovery RFC: https://github.com/alindima/RFCs/blob/av-chunk-indices/text/0047-assignment-of-availability-chunks.md

Better implemented Reed Solomon: https://github.com/paritytech/reed-solomon-novelpoly/issues/40

---

# Summary

- Data flows from collators to backers
- Backers run initial checks and back collations
- Collations are stored erasure encoded chunks (DA layer)
- 10Mb per execution core of DA for 24h

---

<!-- .slide: data-background-color="#4A2439" -->

# Questions

---

## References

1. https://www.youtube.com/watch?v=1pQJkt7-R4Q
1. https://github.com/alindima/RFCs/blob/av-chunk-indices/text/0047-assignment-of-availability-chunks.md
