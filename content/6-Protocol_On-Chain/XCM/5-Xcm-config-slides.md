---
title: XCM Configuration
description: Introduction to configuring the XCM executor.
duration: 1 hour
---

# XCM Config

Notes:

We mentioned the executor is highly configurable.
What can we configure?

---v

## What you'll learn

<pba-flex center>

- Different configuration items available in the XCM executor.
- The tools available to help you configure them.

---

# Config items

<pba-flex center>

- AssetTransactor
- Barrier
- IsReserve
- IsTeleporter
- LocationConverter
- Weigher

Notes:

We'll briefly introduce you to all these concepts.
It might be a bit dry, but the idea is to jump straight to the workshop afterwards.

These are all associated types on an `XcmConfig` struct.

---

# Asset Transactor

---v

<pba-flex center>

Assets can live in different pallets:

- pallet-balances
- pallet-assets
- pallet-nfts

You can even have multiple instances of these pallets!

---v

How does the XCM executor know where each asset lives?

You configure it!

<!-- .element: class="fragment" data-fragment-index="1" -->

---v

## Available adapters

<pba-flex center>

- Fungible(s) adapter: pallet-balances/pallet-assets
<!-- .element: class="fragment" data-fragment-index="1" -->
- Nonfungible(s) adapter: pallet-uniques/pallet-nfts
<!-- .element: class="fragment" data-fragment-index="2" -->

Notes:

Although you can write your own logic for this, we have some available adapters
in the `polkadot-sdk` repo in the `xcm-builder` crate.
Each can be configured to support different assets.

---v

## Example (continued)

<img src="img/Example Flow - Asset Transactor.png">

Notes:

DOT lives in `pallet-balances`.
USDT lives in an instance of `pallet-assets`.
`AssetTransactor` is a sequence of transactors, we try them one by now until one
matches the asset id.
We'll see how we can configure these later.

---

# Barrier

By default we can receive any message from chains we are connected to.

Do we want to?

<!-- .element: class="fragment" data-fragment-index="1" -->

Notes:

The barrier lets us filter incoming messages based on their structure.
It's a firewall.

---v

## A firewall for XCMs

Chains can configure its barrier to immediately filter unwanted messages.

---v

<img src="img/Example Flow - Barrier (intro).png">

Notes:

Our example would need to pass the barrier to reach its destination and be executed.

---v

<img src="img/Example Flow - Barrier (wrong origin).png">

Notes:

We can filter messages from unwanted origins.
We can also filter messages with unwanted instructions.
For example, this "Random Action" doesn't have to be a transfer, could be trying to execute anything.

---v

## Available barriers

<pba-flex center>

- AllowTopLevelPaidExecutionFrom
<!-- .element: class="fragment" data-fragment-index="1" -->
- AllowExplicitUnpaidExecutionFrom
<!-- .element: class="fragment" data-fragment-index="2" -->
- ...
<!-- .element: class="fragment" data-fragment-index="3" -->

Notes:

These are two of the most relevant barriers provided.
There are more.
TODO: Would be good to have good documentation on barriers to link here.
You can of course write your own and filter whatever you want.
They come with some degree of customizability.

---v

### AllowTopLevelPaidExecutionFrom

<img src="img/Example Flow - Barrier (no fees).png">

Notes:

As long as they include the `PayFees` instruction, meaning they intend to pay fees,
this barrier will allow them to pass.

---v

### AllowExplicitUnpaidExecutionFrom

<img src="img/Example Flow - Barrier (explicit unpaid).png">

Notes:

This barrier will allow messages from some origins when they use the `UnpaidExecution`
instruction.
These could be to allow messages from the system.
It's used a lot between system chains.

---

# LocationConverter

Way to convert from `Location` to `AccountId`.

This is how we get sovereign accounts.

Notes:

We always deal with locations on XCM, never accounts.
This config item converts locations to accounts.
This means pallets can have account, external locations can also have accounts.

---v

## Available converters

<pba-flex center>

- AccountId32Aliases
<!-- .element: class="fragment" data-fragment-index="1" -->
- HashedDescription
<!-- .element: class="fragment" data-fragment-index="2" -->

Notes:

The first just grabs an AccountId32 junction and gets the local account from it.
The second is very generic and can be used to generate accounts for any type of junction.

---

## IsReserve and IsTeleporter

Specifies reserve locations for particular assets.

---v

<img src="img/Example Flow - IsReserve.png">

---v

<img src="img/Example Flow - IsTeleporter.png">

---

## Weigher

The weigher weighs XCMs, it assigns a weight to each instruction.

---v

### Available weighers

<pba-flex center>

- FixedWeightBounds (testing)
<!-- .element: class="fragment" data-fragment-index="1" -->
- WeightInfoBounds (production)
<!-- .element: class="fragment" data-fragment-index="2" -->

Notes:

FixedWeightBounds is only for testing, it assigns a constant weight to each instruction.

WeightInfoBounds uses benchmarks for assigning different weights to different instructions.

---v

### Benchmarks!

XCM instructions need to be weighed by each runtime!

Different configuration items change the benchmarks.

---v

<img src="img/Example Flow - Weigher.png">

Notes:

If you're using `pallet-assets` for example, instructions that use the `AssetTransactor`
will inevitably be more expensive.
Always benchmark the worst case.

---

# Next steps

We'll look at the XCM emulator, a tool to let us experiment configuring XCM, sending and executing messages.
