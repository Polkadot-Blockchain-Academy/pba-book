---
title: XCM Emulator
description: Introduction to the XCM emulator.
duration: 1 hour
---

# XCM Emulator

Notes:

The emulator will let us use all the knowledge we've gained so far.

---v

## What you'll learn

- How to use the XCM emulator for testing your cross-chain interactions.

---

# What is it?

A collection of macros that create a mock network we can use test our XCM configuration.

---

# Creating a mock network

---v

## Runtimes

We first need to create our runtimes, a relay chain and a minimum of a parachain.

---v

## Relay chains

```rust
decl_test_relay_chains! {
  pub struct Relay {
    runtime = relay_runtime,
    core = {
      SovereignAccountOf: location_converter,
    },
    pallets = {
      ...
    },
    genesis = genesis(),
    on_init = {
      ...
    },
  }
}
```

Notes:

XCM emulator provides a macro for declaring mock relay chains.

We need to specify a couple of things:

- runtime: The actual runtime of our relay chain.
- core: Some core functionality used by the emulator, things like handlers and converters.
- pallets: All pallets you want to use for tests.
- genesis: Genesis configuration for testing, you can register assets for example.
- on_init: You can call some `on_initialize`s here.

---v

## Parachains

```rust
decl_test_parachains! {
  pub struct ParaA {
    runtime = parachain_runtime,
    core = {
      ...
    },
    pallets = {
      ...
    },
    genesis = genesis(),
    on_init = {
      ...
    },
  }
}
```

Notes:

XCM emulator provides a macro for declaring mock parachains.

We need to specify the same things.

---v

## The entire network

```rust
decl_test_networks! {
  pub struct MockNet {
    relay_chain = Relay,
    parachains = vec![
      ParaA,
      ParaB,
    ],
    bridge = (),
  }
}
```

Notes:

We declare our network by specifying our relay chain and parachains.
We can also declare a bridge to test sending messages to different networks.
In that case we also use `decl_test_bridges!` to define the bridges.

---

# Simple Test

```rust
#[test]
fn test_xcm() {
  MockNet::reset();

  ParaA::execute_with(|| {
    let message: Xcm<()> = <some_xcm>;
    let destination = Location::new(1, [Parachain(2001)]);
    parachain_runtime::XcmPallet::execute(
      parachain_runtime::RuntimeOrigin::signed(<some_account>),
      Box::new(VersionedXcm::V5(message)),
      Weight::from_parts(...), // Some max weight.
    );
  });

  ParaB::execute_with(|| {
    // Message arrives, do some asserts.
  })
}
```

Notes:

First we reset the state of the network.
Then we create a message and call the XCM pallet's `execute` extrinsic.
We must wrap the location and XCM in a `Versioned*` type.

---

# Next steps

- Zombienet
- Chopsticks

Notes:

In the realm of testing tools we also have Zombienet and Chopsticks.

Zombienet for spawning a local network and testing against it with either typescript or a DSL.

Chopsticks for forking a live chain and testing against it.

---

# Workshop

We'll now jump on the emulator, configure XCM, execute and send messages.
