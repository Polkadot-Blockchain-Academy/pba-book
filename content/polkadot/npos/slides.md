---
title: Nominated Proof of Stake
description: An introduction to Nominated Proof of Stake in Polkadot
duration: 1 hour
---

# Nominated Proof of Stake

## _in Polkadot_

---

## Why Proof of Stake Ser?

Why do we use PoS?

<div>

- Tokens locked + prone to being slashed.
- Economic Security 💸🤑.

</div>

<!-- .element: class="fragment" -->

- Everything else (finality, parachains, etc.) is built on top of this base layer of economic security.

<!-- .element: class="fragment" -->

---v

### Why Proof of Stake Ser?

- Remember that Polkadot is at the end of the day a validator-set-as-a-service.
- Secure Blockspace, the main product of Polkadot is provided by these validators.

<img style="width: 1000px" src="./img/dev-6-x-npos-vsas.svg" />

---

## What is NPoS: Assumptions

Assumptions:

- **Validators**: those who intend to author blocks. i.e. _Validator candidate_.

<!-- .element: class="fragment" -->

- **Nominators/Delegators**: Those who intend to support wanna-be authors.

<!-- .element: class="fragment" -->

- Validation and nomination intentions can change, therefore we need **periodic elections** to
  always choose the **active/winner validators/delegators** + hold them slashable.

<!-- .element: class="fragment" -->

- Every election period is called an **_Era_**, e.g. 24hrs in Polkadot.

<!-- .element: class="fragment" -->

---

### What is NPoS: Re-inventing the Wheel

---v

**Solo-POS**

<img style="width: 1000px" src="./img/dev-6-x-npos-0.svg" />

---v

### What is NPoS: Re-inventing the Wheel

- Authority-wanna-bees aka. validators bring their own stake. No further participation. Top validators are elected.

- Problems?

Notes:

Low amount of stake that we can capture, impossible for those who don't want to run the hardware to join.

---v

**Single-Delegation-POS**

<img style="width: 1000px" src="./img/dev-6-x-npos-1.svg" />

---v

### What is NPoS: Re-inventing the Wheel

- Anyone can dilute themselves in any given validators. Top validator based on total stake are
  elected.
- Voters are called **delegators**.

- Problems?

Notes:

- Better, but funds might be delegated to non-winners, which get wasted.
- In other words, there is no incentive to delegate to those that are non-winners.

---v

**Multi-Delegation-POS**

<img style="width: 1000px" src="./img/dev-6-x-npos-2.svg" />

---v

### What is NPoS: Re-inventing the Wheel

Your stake is divided $\frac{1}{N}$ (or arbitrarily) among $N$ validators.

Problems?

Notes:

Same issue as before.

---v

**Nominated Proof of Stake**

<img style="width: 1000px" src="./img/dev-6-x-npos-3.svg" />

---v

**Nominated Proof of Stake**

<img style="width: 1000px" src="./img/dev-6-x-npos-4.svg" />

---v

### What is NPoS: Re-inventing the Wheel

- You name up to `N` nominees, an _algorithm_, computed either onchain or offchain, decides
  the **winners** and **how to distribute the stake among them**.
- Voters are called **Nominators**.

---v

### What is NPoS: Re-inventing the Wheel

- ✅ As a nominator, you are free to express your desire to back non-winners as well. Once enough people have expressed the same desire, the non-winner will become a winner.

<!-- .element: class="fragment" -->

- ✅ Has a much higher chance to make sure staked tokens won't get wasted.

<!-- .element: class="fragment" -->

- ✅ Can optimize other criteria other than "who had more approval votes".

<!-- .element: class="fragment" -->

---

## NPoS Drawbacks

- We decided to solve an np-hard, multi-winner, approval based, election problem onchain 🤠.

<pba-flex center>

- scalability. <!-- .element: class="fragment" -->
- scalability. <!-- .element: class="fragment" -->
- scalability. <!-- .element: class="fragment" -->
- scalability. <!-- .element: class="fragment" -->
- and scalability. <!-- .element: class="fragment" -->

</widget-text>

- But we (strive to) get much better economic security measures in return 🌈.

<!-- .element: class="fragment" -->

- Long term, this can in itself be solved by what Polkadot provides best, more Blockspace 🎉!

## <!-- .element: class="fragment" -->

### NPoS Protocol Overview

- The current NPoS protocol revolves around an **election round**, which is itself made up of 4
  episodes.
- This gives you an idea about how we solved the scalability issue for the time being.

---v

### NPoS Protocol Overview: Episode 1

**Snapshot**

- Enables multi-block election.
- Allows us to not need to "freeze" the staking system.
- Allows us to index stakers, not `AccountIds`.

---v

### NPoS Protocol Overview: Episode 2

**Signed Submissions**

- Any signed account can come up with a **NPoS solution** based on that snapshot.
- Deposits, rewards, slash, other game-theoretic tools incorporated to make to secure.

---v

### NPoS Protocol Overview: Episode 3

**Validator Submissions as Fallback**

- As the first backup, any validator can also submit a solution as a part of their block authoring.

---v

### NPoS Protocol Overview: Episode 4

**Fallbacks**

- If all of the above fails, the chain won't rotate validators and the governance can either:
  - dictate the next validator set.
  - trigger an onchain election (limited in what it can do).

## This was recently [used in Kusama](https://forum.polkadot.network/t/kusama-era-4543-slashing/1410) 🦜.

## NPoS Objective

- Given the powerful tool of NPoS, what should we aim for?
- Let's first recap:

1. Polkadot validators are the source of truth for the state transition of both the relay chain and all of the parachains + bridges.

<!-- .element: class="fragment" -->

2. Polkadot validator are assigned to parachains as backing group, and swapped over time.

<!-- .element: class="fragment" -->

3. Polkadot validators all author the same number of blocks, i.e. they are of same importance.

<!-- .element: class="fragment" -->

Notes:

Point 2 is not to imply that the polkadot validator set's security is partitioned among parachains,
security comes from approval voters.
<https://www.polkadot.network/blog/polkadot-v1-0-sharding-and-economic-security/>

---v

### NPoS Objective: Election Score

```rust
pub struct ElectionScore {
  /// The minimal winner, in terms of total backing stake.
  ///
  /// This parameter should be maximized.
  pub minimal_stake: u128,
  /// The sum of the total backing of all winners.
  ///
  /// This parameter should maximized
  pub sum_stake: u128,
  /// The sum squared of the total backing of all winners, aka. the variance.
  ///
  /// Ths parameter should be minimized.
  pub sum_stake_squared: u128,
}
```

---v

### NPoS Objective: Election Score

- NPoS allows us to incentivize the formation of a validator set that optimized the aforementioned `ElectionScore`.

- This score is ALWAYS calculate and checked onchain. This is why we can accept solutions from the outer world.

Notes:

A common example: we allow signed submissions. What if they send solutions that are censoring a particular validator? if it can achieve a better score, so be it! we don't care.

---v

### NPoS Objective: Election Score

- The default algorithm used in both the onchain/offchain solvers is the [Phragmen algorithm](https://en.wikipedia.org/wiki/Phragmen%27s_voting_rules).
- Proved to provide high fairness and justified representation properties whilst being verifiable in
  linear time.

---

## NPoS Future

- Fresh from the oven (Jan 2023): [Future of Polkadot Staking in the Polkadot forum](https://forum.polkadot.network/t/the-future-of-polkadot-staking/1848/2).
- [Github issue-tracker/project](https://github.com/orgs/paritytech/projects/33)

<hr>

- Nomination Pools
- Multi-page election submission
- Operators as first class citizens.
- fast-unstake.

---

## Additional Resources! 😋

<img width="300px" rounded src="../../substrate/scale/img/thats_all_folks.png" />

> Check speaker notes (click "s" 😉)

Notes:

### Further Reading

- Recent Kusama slashing: https://forum.polkadot.network/t/kusama-era-4543-slashing/1410
- [A verifiably secure and proportional committee election rule](https://arxiv.org/abs/2004.12990)
- 4.1 in [Overview of Polkadot and its Design Considerations](https://arxiv.org/abs/2005.13456)
- [Proportional Justified Representation](https://arxiv.org/abs/1611.09928)
- [Justified representation - Wikipedia](https://en.wikipedia.org/wiki/Justified_representation)

### NPoS Protocol: More Details, Backup Slides

- `bags-list`: how to store an unbounded semi-sorted linked-list onchain.
- Nomination pools: best of both.
- Minimum-untrusted score.
- PJR checking: why we don't do it.
- `reduce` optimization.

### Feedback After Lecture:
