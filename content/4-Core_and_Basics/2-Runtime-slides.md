---
title: Runtime
description: Runtime (AKA State Transition Function(s))
---

# Runtime

---

# Runtime

## What you will learn:

- What is the Runtime?<!-- .element: class="fragment" -->
- Where is located?<!-- .element: class="fragment" -->
- How does it work?<!-- .element: class="fragment" -->
- Tradeoffs of having a dynamic STF.<!-- .element: class="fragment" -->
- Who does it cather to?<!-- .element: class="fragment" -->
- Types of Runtime APIs.<!-- .element: class="fragment" -->

---

# Runtime: Introduction

## Runtime: What is it?

- It's the logic of the State Transition Fuctions that defines the business logic of the chain.<!-- .element: class="fragment" -->
- It currently is WASM code that's stored on the state of the chain.<!-- .element: class="fragment" -->
- It's part of the state, and it also determines how this state changes 🤯.<!-- .element: class="fragment" -->

---

## Runtime: part of the state of the chain.

- Why?
  - In an ideal world it allows for forkless upgrades.<!-- .element: class="fragment" -->

---

## Runtime: part of the state of the chain.

- How does it work<!-- .element: class="fragment" -->

  - Every time that the WASM changes, the node instantiates the new WASM.<!-- .element: class="fragment" -->

- Host Functions:<!-- .element: class="fragment" -->

  - When the WASM is instaniated, the node provides an environment so that the WASM can interact with the node.<!-- .element: class="fragment" -->
  - Eg: Access storage, Expensive cryptografic functions, etc.<!-- .element: class="fragment" -->

- Can a new WASM version require newer/different host functions?<!-- .element: class="fragment" -->

Notes:

- Discuss implications of changing host functions.

---

## Tradeoffs of having a dynamic runtime.

- The good: enables forkless upgrades.<!-- .element: class="fragment" -->
- The not so good:<!-- .element: class="fragment" -->
  - State migrations are complicated.<!-- .element: class="fragment" -->
  - Transactions become invalid.<!-- .element: class="fragment" -->
  - Makes DApps and tooling more complicated.<!-- .element: class="fragment" -->

---

## Tradeoffs of having a dynamic runtime.

- The good: enables forkless upgrades.
- The not so good:
  - State migrations are complicated.
  - Transactions can:
    - Become invalid.
    - Have a different outcome.
  - Makes DApps and tooling more complicated.

---

## Runtime functions:

| How it started                              |            How it's going            |
| ------------------------------------------- | :----------------------------------: |
| Core_initialize_block                       |        DryRunApi_dry_run_call        |
| Core_execute_block                          |          Metadata_metadata           |
| Block_builder_apply_extrinsic               | NominationPoolsApi_points_to_balance |
| TaggedTransactionQueue_validate_transaction | Inflation_inflation_prediction_info  |

Notes:

- Discuss why it's a good idea to expose via runtime-apis functions that
  prevent DApps from re-implementing on-chain logic.

---

## Runtime: who does it cather to?

Runtime functions can be divided in 3 different groups:

- Functions that cather the node: applying extrinsics, handling blocks, etc.<!-- .element: class="fragment" -->
- Functions that cather DApps: by exposing logic that otherwise would have to be duplicated offchain.<!-- .element: class="fragment" -->
- Functions that cather both: transaction validation transactions, calculating fees, etc.<!-- .element: class="fragment" -->

---

## Non Host-specific runtime-apis:

- Metadata<!-- .element: class="fragment" -->
- Fees<!-- .element: class="fragment" -->
- Dry-Runs<!-- .element: class="fragment" -->
- View Functions<!-- .element: class="fragment" -->
