---
title: PAPI SDKs
description: TODO
duration: 4 hours
owner: Carlo Sala
---

# Polkadot-API SDKs

---

## PAPI SDKs

#### Content

- Motivation
- Goals
- An example: Governance

---

## PAPI SDKs

#### Motivation

- PAPI is agnostic to the runtime (_aka_ business logic).
- PAPI SDKs fill the gap for common dApp needs.
- Avoid reinventing the wheel.

---

## PAPI SDKs

#### Methodology

1. `createXXXSdk(typedApi)`. It defines the expected TS types for descriptors in order to work.
2. Make incremental releases. Instead of starting with a big API, improve it during the development.
3. Listen the community.

---

## PAPI SDKs

#### Goals

- Simplify dApp development.
- Let the dApp developer add value instead of reimplementing the same common logic.
- Track and adapt to pallet upgrades in advance to ensure seamless transitions.
- Keep it atomic to avoid and avoid bloated APIs.

---

## PAPI SDKs

#### Governance

- We want to abstract as much as possible the interaction with the chain.
- We discovered ~_3.5_~ 4 main groups of functionality:
  - Referenda
  - ConvictionVoting
  - Bounties / ChildBounties

---

#### Governance - Referenda

**Which problems were identified?**

- State is split among different storage and constant entries; e.g. `Referenda.ReferendumInfoFor`, `Referenda.Tracks`. They
  have to be matched and combined to get the full picture.
- There are parts of the state that do not directly appear in the chain state (e.g. when a poll is expected to end) and
  require some extra computation.

---

#### Governance - Referenda

**What do we want to include in the SDK?**

- Fetch referenda and get information about them (abstract storage) <!-- .element: class="fragment" -->
  - With extra info. E.g. when is a poll expected to end, resolve preimage, decode call, etc
- Create referenda <!-- .element: class="fragment" -->
  - Figure out the track where a referendum should go <!-- .element: class="fragment" -->
  - Abstract preimages <!-- .element: class="fragment" -->

---

#### Governance - Referenda

**What we do NOT want to include in the SDK?**

- Interactions where we cannot improve the API / add value.

  - E.g. `Referenda.place_decision_deposit(index: u32)`. We cannot add value ==> we do not add the API.

    <span style="font-size: 0.6em; opacity: 0.6">(We are still discussing if this is the best behaviour, take it with a pinch of salt.)</span>

---

#### Governance - ConvictionVoting

**Which problems were identified?**

- Again, state is split among several entries. Besides that, every voting track goes on its own.
- `Vote` type
- Derived state: which votes can be removed, and when.

---

#### Governance - ConvictionVoting

**Features:**

- Get votes and delegations for an account. <!-- .element: class="fragment" -->
  - With enhanced information; e.g. unlock schedule of all votes.
- Vote <!-- .element: class="fragment" -->
  - Abstract complex `Vote` type and conviction.

---

#### Governance - Bounties (and ChildBounties)

**Which problems were identified?**

- A bounty can be in a gazillion different states. <!-- .element: class="fragment" -->

---v

![Image](./img/bounties.svg)

---

#### Governance - Bounties (and ChildBounties)

**Which problems were identified?**

- A bounty can be in a gazillion different states.
- There are many actions to bounties that are performed through referenda. <!-- .element: class="fragment" -->

---

#### Governance - Bounties (and ChildBounties)

**Features:**

- Abstract state of bounties (or child bounties), and expose only relevant actions to the state. <!-- .element: class="fragment" -->
- Find referenda that change the state of the bounty. <!-- .element: class="fragment" -->
