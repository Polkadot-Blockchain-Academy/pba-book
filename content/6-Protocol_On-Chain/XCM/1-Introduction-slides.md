---
title: Introduction to XCM
description: What it is, why it's necessary, what we'll cover
duration: 45 minutes
---

# XCM

---

## On this module, we'll see

<pba-flex center>

- What XCM is
- What primitives make up XCMs
- How XCMs are executed
- The XCM Pallet
- How to configure systems to work with XCM
- How to craft XCMs

---

# The problem

Substrate and FRAME encourage an ecosystem of many blockchains.

We don't want every chain to have to do everything.

---v

## Specialization

Specialization allows chains to focus on their particular value proposition and do it well.

---

# The solution

Messaging.

Notes:

In order for chains to use functionality from other chains,
we need to pass messages from one chain to another.

---v

## How?

<pba-flex center>

- UMP/DMP: Relay <-> Parachains
- HRMP: Parachain <-> Parachain
- Bridges: Polkadot <-> Ethereum and more

Notes:

UMP/DMP stands for Upward/Downward Message Passing.
HRMP piggybacks on UMP and DMP to send messages to sibling parachains, so it uses the relay chain as a relayer.
These are the current transport protocols we have in Polkadot.
An upgrade to HRMP is XCMP which won't use the relay chain at all.
It's not live yet, still being researched and prototyped.

---v

## What?

What messages do we actually send?

---v

### Native messages

The messages could just be the native transactions of each chain.

Notes:

If we have a message passing protocol, we can just allow one chain
to post transactions to another chain.

This is very simple and useful, but it has some disadvantages.

---v

### Native messages: disadvantages

- Format changes from system to system, it could also change within the same system, e.g. on a runtime upgrade.
- Common cross-consensus use-cases don't map one-to-one to a single transaction.
- Different consensus systems have different assumptions e.g. fee payment.

Notes:

- A system which intends to send messages to more than one destination would need to understand how to author a message for each.
  On that note, even a single destination may alter its native transaction/message format over time.
  Smart contracts might get upgrades, blockchains might introduce new features or alter existing ones and in doing so change their transaction format.
- Special tricks may be required to withdraw funds, exchange them and then deposit the result all inside a single transaction.
  Onward notifications of transfers, needed for a coherent reserve-asset framework, do not exist in chains unaware of others.
  Some use-cases don't require accounts.
- Some systems assume that fee payment had already been negotiated, while some do not.

---v

### A better solution

- A domain-specific language that is agnostic to the underlying chain implementing it.
- Able to express multiple actions in a single message.
- Flexible enough to accomodate multiple different paradigms.

---

# XCM

> XCM is a **language** for communicating **intentions** between **consensus systems**.

---v

## Language

Cross-chain DSL.

Each message is a script that allows expressing multiple actions.

---v

## Intentions

Blockchains are sovereign entities, the language only expresses what we _want_
the receiver to do, it can't enforce it.

Verification can be done on another layer, e.g. checking the expected result with a light client.

Notes:

It's up to the receiver to interpret the message and execute the corresponding actions.

Not every chain might implement every intention, for example swaps.

---v

## Consensus systems

We extend the actors to be consensus systems, not only chains.

This includes smart contracts and any system that achieves consensus in some way.

Notes:

Proof-of-authority web2 systems can also be considered consensus systems.

---v

## Versioned

XCM is a **versioned** language.

Each system declares what version they support.

The latest is version 5.

What goes in each version is defined via an RFC process.

Notes:

This protects against runtime upgrades breaking everything.

---

# XCM vs Message Passing

<img src="img/XCM Post truck.png">

Notes:

XCM is akin to the post card from the post office.

It is _not_ a transport protocol.

A post card doesn't send itself!

As we mentioned, XCM != {UMP, DMP, HRMP, XCMP, Bridges}.

It cannot be used to actually "send" any message between systems; its utility is only in expressing what should be done by the receiver.
Like many aspects core to Substrate, this separation of concerns empowers us to be far more generic and enable much more.
A post card relies on the postal service to get itself sent towards its receivers, and that is what a messaging protocol does.

The transport layer concerns itself with sending arbitrary blobs, it doesn't care about the format.

---

### Terminology: XCMs

**XCM**, Cross-Consensus Messaging, is the format.

**An XCM** is a Cross-Consensus Message.

It's not called an XCM message,

the same way it's not called an ATM machine.

We can also call them XCM programs, since they are executable.

Notes:

More about their executable nature later.

---

# Summary

<pba-flex center>

- We learned what XCM is and why it's useful.
- We learned what transport protocols exist in Polkadot.

---

# Questions?

---

# Example Flow

<img src="img/Example Flow - Introduction.png">

Notes:

We'll build towards this during this module.

---

# Next steps

We'll learn about the primitives that make up any XCM.
