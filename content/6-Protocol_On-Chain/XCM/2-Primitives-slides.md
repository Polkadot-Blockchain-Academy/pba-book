---
title: XCM Primitives
description: Locations, assets and instructions
duration: 2 hours
---

# XCM Primitives

---v

## What you'll learn

<pba-flex center>

- Locations
- Assets
- Instructions

Notes:

Locations: how an XCM references various entities in the network.
Assets: how an XCM can specify fungible and non-fungible assets.
Instructions: how an XCM expresses what actions it wants the receiver to perform.

---

# Locations

Way of referencing any entity in the network, for example, the recipient of the message.

<diagram class="mermaid limit size-50">
graph LR
    Message(Message)
    Alice(Alice)--"?"-->Bob(Bob)
    Alice--"?"-->AssetHub(Asset Hub)
    Alice--"?"-->Pallet(Pallet)
    Alice--"?"-->SmartContract(Smart Contract)
</diagram>

Notes:

The `Location` is sort of the URL for consensus systems.
A Location can be a scalable multi-shard blockchain such as Polkadot, an ERC-20 asset account on a parachain, a smart contract, etc.
They can be relative or absolute.
They don't need to correspond with the actual path the message will take, that's the job of the transport protocol.

---v

## Interior Locations

> Given two consensus systems, A and B. A is **interior** to B if a state change in A implies a state change in B.

Notes:

An example, a smart contract in Ethereum would be interior to Ethereum itself.

---v

## Location hierarchy

<diagram class="mermaid">
graph TD;
    Relay(Relay)-->A(Parachain A)
    Relay-->B(Parachain B)
    B-->Alice(Account A)
    B-->Bob(Account B)
    A-->Pallet(Pallet Contracts)
    Pallet-->SCA(Smart Contract A)
    Pallet-->SCB(Smart Contract B)
</diagram>

Notes:

Locations form a hierarchy using the interior relation.

---v

## Location Representation

<pba-flex center>

```rust
struct Location {
    parents: u8,
    junctions: Junctions,
}
```

<div style="margin-bottom: 2rem;"></div>

```rust
enum Junction {
    Parachain(u32),
    AccountId32 { id: [u8; 32], network: Option<NetworkId> },
    PalletInstance(u8),
    GeneralIndex(u128),
    GlobalConsensus(NetworkId),
    ...
}
```

Notes:

Right now Junctions are limited to 8 because of stack space.
We also don't expect Junctions being more than 8 levels deep.

It's perfectly possible to create locations that don't point anywhere.

---v

### Network Id

<pba-flex center>

```rust
enum NetworkId {
    ByGenesis([u8; 32]),
    ByFork { block_number: u64, block_hash: [u8; 32] },
    Polkadot,
    Kusama,
    Ethereum { chain_id: u64 },
    BitcoinCore,
    BitcoinCash,
    PolkadotBulletin,
}
```

</pba-col>

</pba-cols>

Notes:

Junctions are ways to descend the location hierarchy

---v

## Text notation

<pba-flex center>

<pba-cols>

<pba-col>

```rust
Location {
    parents: 1,
    interior: Parachain(50)
}
```

</pba-col>
<pba-col>

-->

</pba-col>
<pba-col>

```
../Parachain(50)
```

</pba-col>

Notes:

This notation comes from an analogy to a file system.

---v

## Universal Location

> The Universal Location is a **theoretical** location. It's the parent of all locations which generate their own consensus. It itself has no parents.

---v

## Universal Location

<diagram class="mermaid limit size-50">
graph TD;
    UniversalLocation(Universal Location)-->Polkadot(Polkadot)
    UniversalLocation-->Kusama(Kusama)
    UniversalLocation-->Ethereum(Ethereum)
    UniversalLocation-->Bitcoin(Bitcoin)
</diagram>

Notes:

We can imagine a hypothetical location that contains all top-level consensus systems.

---v

## Absolute locations

<pba-flex center>

```rust
pub type InteriorLocation = Junctions;
```

Sometimes, absolute locations are necessary, e.g. for bridges.

They don't have parents.

The first junction has to be a `GlobalConsensus`.

Notes:

To write an absolute location, we need to know our location relative to the Universal Location.

---v

## What are `Location`s used for?

<pba-flex center>

- Addressing
- Origins
- Assets
- Fees
- Bridging

---v

## Cross-Chain Origins

When a receiver gets an XCM, a `Location` specifies the sender.

This `Location` is _relative_ to the receiver.

Can be converted into a pallet origin in a FRAME runtime

Used for determining privileges during XCM execution.

Notes:

Reanchoring:

Since `Location`s are relative, when an XCM gets sent over to another chain, the origin location needs to be rewritten from the perspective of the receiver, before the XCM is sent to it.

---

## Location Examples

---v

### Sibling parachain

`../Parachain(1001)`

<diagram class="mermaid">
graph TD
    Polkadot(Polkadot)-->AssetHub("📍 AssetHub (1000)")
    Polkadot-->Collectives("Collectives (1001)")
</diagram>

Notes:

What does the location resolve to if evaluated on Parachain(1000)?

---v

### Sibling parachain

`../Parachain(1001)`

<diagram class="mermaid">
graph TD
    Polkadot(Polkadot)-->AssetHub("📍 AssetHub (1000)")
    Polkadot-->Collectives("Collectives (1001)")
    AssetHub-->Polkadot
    linkStyle 0 opacity:0.3
    linkStyle 2 stroke-dasharray:5
</diagram>

---v

### Parachain account

`Parachain(1000)/AccountId32(0x1234...cdef)`

<diagram class="mermaid">
graph TD
    Polkadot("📍 Polkadot")-->AssetHub("AssetHub (1000)")
    Polkadot-->Collectives("Collectives (1001)")
    AssetHub-->Account("AccountId32 (0x1234...cdef)")
</diagram>

Notes:

What does the location resolve to if evaluated on the relay chain?

---v

### Parachain account

`Parachain(1000)/AccountId32(0x1234...cdef)`

<diagram class="mermaid">
graph TD
    Polkadot("📍 Polkadot")-->AssetHub("AssetHub (1000)")
    Polkadot-->Collectives("Collectives (1001)"):::disabled
    AssetHub-->Account("AccountId32 (0x1234...cdef)")
    linkStyle 1 opacity:0.3
    classDef disabled opacity:0.3
</diagram>

---v

### Bridge

`../../GlobalConsensus(Kusama)/Parachain(1000)`

<diagram class="mermaid">
graph TD
    Universe(Universal Location)-->Polkadot(Polkadot)
    Universe-->Kusama(Kusama)
    Polkadot-->PolkaA("📍 Asset Hub (1000)")
    Polkadot-->PolkaB(Bridge Hub)
    PolkaA-->Alice(Alice)
    PolkaA-->AssetsPallet(Pallet Assets)
    AssetsPallet-->Asset(USDT)
    Kusama-->KusamaA("Asset Hub (1000)")
    Kusama-->KusamaB(Bridge Hub)
</diagram>

Notes:

Speak to an example of non-parachain multi-location that would use a bridge
XCM reasons about addressing (as in a postal address) that must include understanding where you are, not just where you are going!
This will be very powerful later on (Origins)

---v

### Bridge

`../../GlobalConsensus(Kusama)/Parachain(1000)`

<diagram class="mermaid">
graph TD
    Universe(Universal Location)-->Polkadot(Polkadot)
    Universe-->Kusama(Kusama)
    Polkadot-->PolkaA("📍 Asset Hub (1000)")
    Polkadot-->PolkaB(Bridge Hub):::disabled
    PolkaA-->Alice(Alice):::disabled
    PolkaA-->AssetsPallet(Pallet Assets):::disabled
    AssetsPallet-->Asset(USDT):::disabled
    Kusama-->KusamA("Asset Hub (1000)")
    Kusama-->KusamB(Bridge Hub):::disabled
    PolkaA-->Polkadot
    Polkadot-->Universe
    linkStyle 0 opacity:0.3
    linkStyle 2 opacity:0.3
    linkStyle 3 opacity:0.3
    linkStyle 4 opacity:0.3
    linkStyle 5 opacity:0.3
    linkStyle 6 opacity:0.3
    linkStyle 8 opacity:0.3
    linkStyle 9 stroke-dasharray:5
    linkStyle 10 stroke-dasharray:5
    classDef disabled opacity:0.3
</diagram>

Notes:

Even with Bridge Hubs, the relative location is what you'd expect.
Bridge Hubs are just a way for routing messages.
They are an implementation detail of the transport layer.

---v

### Bridge (actual routing)

<diagram class="mermaid limit size-50">
graph TD
    Universe(Universal Location):::disabled-->Polkadot(Polkadot):::disabled
    Universe-->Kusama(Kusama)
    Polkadot-->PolkaA("📍 Asset Hub (1000)")
    Polkadot-->PolkaB(Bridge Hub)
    PolkaA-->Alice(Alice):::disabled
    PolkaA-->AssetsPallet(Pallet Assets):::disabled
    AssetsPallet-->Asset(USDT):::disabled
    Kusama-->KusamB(Bridge Hub)
    Kusama-->KusamA("Asset Hub (1000)")
    PolkaA-->PolkaB
    PolkaB--"Bridge"-->KusamB
    KusamB-->Kusama
    linkStyle 0 opacity:0.3
    linkStyle 1 opacity:0.3
    linkStyle 2 opacity:0.3
    linkStyle 3 opacity:0.3
    linkStyle 4 opacity:0.3
    linkStyle 5 opacity:0.3
    linkStyle 6 opacity:0.3
    linkStyle 7 opacity:0.3
    linkStyle 11 stroke-dasharray:5
    classDef disabled opacity:0.3
</diagram>

Notes:

The actual message is routed through Bridge Hub.

---v

## Sovereign Accounts

Locations external to the local system can be represented by a local account.

We call this the **sovereign account** of that location.

They are a mapping from a `Location` to an account id.

---v

<diagram class="mermaid">
graph TD
    Polkadot(Polkadot)-->A(AssetHub) & B(Collectives)
    A-->Alice(Alice)
    B-->AliceSA("Alice's sovereign account")
</diagram>

Notes:

A sovereign account is an account on one system that is controlled by another on a different system.
A single account on a system can have multiple sovereign accounts on many other systems.
In this example, Alice is an account on AssetHub, and it controls a sovereign account on Collectives.

When transferring between consensus systems, the sovereign account is the one that gets the funds on the destination system.

---v

<diagram class="mermaid">
graph TD
    Polkadot(Polkadot)-->A(AssetHub) & B(Collectives)
    A-->Alice(Alice)
    B-->AliceSA("Alice's sovereign account")
    B-->ASA("Asset Hub's sovereign account")
    A-->BSA("Collective's sovereign account")
</diagram>

---

# Assets

Most messages will deal with assets in some way.

How do we reference these assets?

---v

### Asset Representation

<pba-flex center>

```rust
struct Asset {
    pub id: AssetId,
    pub fun: Fungibility,
}

struct AssetId(Location); // <- We reuse the location!

enum Fungibility {
    Fungible(u128),
    NonFungible(AssetInstance),
}
```

Notes:

We use locations, which we've already discussed, to refer to assets.

A Asset is composed of an asset ID and an enum representing the fungibility of the asset.
Asset IDs are the location that leads to the system that issues it, this can be just an index in an assets pallet, for example.

Assets can also either be fungible or non-fungible:
Fungible - each token of this asset has the same value as any other
NonFungible - each token of this asset is unique and cannot be seen as having the same value as any other token under this asset

---v

### Asset filtering and wildcards

<pba-flex center>

```rust
enum AssetFilter {
    Definite(Assets),
    Wild(WildAsset),
}

enum WildAsset {
    All,
    AllOf { id: AssetId, fun: WildFungibility },
    // Counted variants
}

enum WildFungibility {
    Fungible,
    NonFungible,
}
```

Notes:

Sometimes we don't want to specify an asset, but rather filter a collection of them.
In this case, we can either list all the assets we want or use a wildcard to select all of them.
In reality, it's better to use the counted variant of the wildcards, for benchmarking.

---

# Instructions

Every XCM is a sequence of instructions.

---v

## Kinds of instructions

<pba-flex center>

- Command
- Trusted Indication
- Information
- System Notification

Notes:

Most instructions are commands.
We have a few trusted indications mainly for cross-chain transfers.
Information instructions are meant to carry over information from other systems.
System notifications notify when particular events happen, we have these for channels and versioning.

---v

### Instruction Examples

<pba-flex center>

```rust
WithdrawAsset(Assets)
```

Notes:

This instruction is a command.
Gets assets from an account to use them during the execution of the message.
We'll see registers in the next lecture when we talk about the executor.
It takes the assets from the account specified in the origin register and puts them in the holding register.

---v

### Instruction Examples

<pba-flex center>

```rust
DepositAsset { assets: AssetFilter, beneficiary: Location }
```

Notes:

Another command.
Will deposit all assets matched by the `assets` filter to the account of `beneficiary`.

---v

### Instruction Examples

<pba-flex center>

```rust
ReceiveTeleportedAsset(Assets)
```

Notes:

This instruction is a trusted indication.
It tells the receiver that the sender has burnt some assets and they should be minted here.
This is used for teleports, which we'll look into in the next lecture.
A lot of trust is needed between both systems.

---v

### Instruction Examples

<pba-flex center>

```rust
ReserveAssetDeposited(Assets)
```

Notes:

Another trusted indication.
Used for reserve asset transfers, the second type of cross-chain transfer we'll see in next lecture.
It tells the current system that assets have been deposited to this system's sovereign account somewhere
and derivatives matching them should be minted here.

---v

### Instruction Examples

<pba-flex center>

```rust
QueryResponse {
    query_id: QueryId,
    response: Response,
    max_weight: Weight,
    querier: Option<Location>,
}
```

Notes:

This instruction is reporting back information.
Different things can be reported, like a certain pallet, the result of an operation, etc.

---

# Summary

<pba-flex center>

- Locations
- Assets
- Instructions

---

# Questions?

---

# Workshop

We'll play around with these primitives.

---

# Next steps

We'll look at an example and how these primitives are used.
