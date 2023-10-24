---
title: Introduction to Cross Consensus Messaging (XCM)
description: XCM Core Concepts, Terms, and Logic for web3 builders
duration: 1 hour
---

# Introduction to Cross-Consensus Messaging (XCM)

## _Core Concepts, Terms, and Logic_

Notes:

**Pre-requisites**

- FRAME (Storage Items, Dispatchables, Event, Errors, etc.)
- Polkadot & parachains conceptually
- Assets (NFTs and fungibles)

---v

## _At the end of this lecture, you will be able to:_

<pba-flex center>

- Define the concepts, syntax, and terms of XCM
- Navigate existing resources that relate to XCM
- Differentiate between XCM and message-passing protocols like XCMP

---

# Cross-chain use cases

Why would we want to perform operations on different blockchains?

Notes:

EXERCISE: ask the class to raise hands and postulate on generally what one might do.
We are expecting them to say transfers, but there are so many other things you could do, so many more problems worth solving with cross-chain:

- One contract calling another contract
- Credential checking
- Voting

---v

## 🎬 Some Concrete Use-cases

<pba-flex center>

- Cross-consensus asset transfers
- Execute platform-specific actions such as governance voting
- Enables single use-case chains
  - [Collectives](https://github.com/paritytech/polkadot-sdk/tree/72c4535/cumulus/parachains/runtimes/collectives)
  - Identity chains

Notes:

While the goal of XCM is to be general, flexible and future-proof, there are of course practical needs which it must address, not least the transfer of tokens between chains.
We need a way to reason about, and pay for, any required fees on the receiving CS.
Platform-specific action; for example, within a Substrate chain, it can be desirable to dispatch a remote call into one of its pallets to access a niche feature.
XCM enables a single chain to direct the actions of many other chains, which hides the complexity of multi-chain messaging behind an understandable and declarative API.

---

> XCM is a **language** for communicating **intentions** between **consensus systems**.

---v

### Consensus systems

A chain, contract or other global, encapsulated, state machine singleton.

<pba-flex center>

It does not even have to be a _distributed_ system, only that it can form _some_ kind of consensus.

Notes:

A consensus system does not necessarily have to be a blockchain or a smart contract.
It can be something that already exists in the Web 2.0 world, such as an EC2 instance in an AWS server.
XCM is Cross-Consensus since it's much more than cross chain.

---v

### ✉️ A Format, not a Protocol

XCM is a **_messaging format_**.

It is akin to the post card from the post office.

It is _not_ a messaging protocol!

A post card doesn't send itself!

Notes:

It cannot be used to actually "send" any message between systems; its utility is only in expressing what should be done by the receiver.
Like many aspects core to Substrate, this separation of concerns empowers us to be far more generic and enable much more.
A post card relies on the postal service to get itself sent towards its receivers, and that is what a messaging protocol does.

The transport layer concerns itself with sending arbitrary blobs, it doesn't care about the format.
A common format has its benefits though, as we'll see next.

---v

### Versioning

XCM is a **versioned** language.

It's currently in version 3.

What goes in each version is defined via an RFC process.

---v

### Terminology: XCMs

**XCM**, Cross-Consensus Messaging, is the format.

**An XCM** is a Cross-Consensus Message.

It's not called an XCM message,

the same way it's not called an ATM machine.

---

## 😬 Why not _native_ messages?

Drawbacks of relying on native messaging or transaction format:

<pba-flex center>

- Native message format changes from system to system, it also could change within the _same_ system, e.g. when upgrading it
- Common cross-consensus use-cases do not map one-to-one to a single transaction
- Different consensus systems have different assumptions e.g. fee payment

Notes:

- A system which intends to send messages to more than one destination would need to understand how to author a message for each.
  On that note, even a single destination may alter its native transaction/message format over time.
  Smart contracts might get upgrades, blockchains might introduce new features or alter existing ones and in doing so change their transaction format.
- Special tricks may be required to withdraw funds, exchange them and then deposit the result all inside a single transaction.
  Onward notifications of transfers, needed for a coherent reserve-asset framework, do not exist in chains unaware of others.
  Some use-cases don't require accounts.
- Some systems assume that fee payment had already been negotiated, while some do not.
  <!--
  FIXME TODO: Why not just send EVM programs. Why XCVM instead of EVM?
  Add Shawn's picture.
  -->
  It's up to the interpreter to interpret the intention how it makes sense.

---v

### Message format changes

<img style="width: 1050px;" src="./img/against-native-messaging.svg" />

---v

### Message format changes

<img style="width: 1050px;" src="./img/against-native-messaging-2.svg" />

---v

### Message format changes

<img rounded style="width: 1050px" src="./img/xcm-executor-routing-calls.png" />

Notes:

XCM abstracts away the actual on-chain operation that will be called, which lets the recipient redirect calls to always make them valid.

---v

### No one-to-one mapping

<diagram class="mermaid limit size-50">
graph TD
    subgraph Message
        WithdrawAsset(WithdrawAsset)-->DepositAlice("DepositAsset(Alice)")
        DepositAlice-->DepositBob("DepositAsset(Bob)")
    end
</diagram>

Notes:

You might want to withdraw some assets and deposit some amount to one account and another to another.
Using transactions, you'd have to send many messages to achieve this.

---v

### Different assumptions

<diagram class="mermaid">
graph LR
    A(Chain A)--"Pays for fees"-->B(Chain B)
    A--"Doesn't pay for fees"-->C(Chain C)
</diagram>

Notes:

Different systems have different assumptions.
Using native messages, you'd have to tailor your messages to all systems you want to message.

---

## Four 'A's

XCM assumes the following things from the underlying environment.

<pba-flex center>

- **Agnostic**
- **Absolute**
- **Asynchronous**
- **Asymmetric**

Notes:

The 4 'A's are assumptions XCM makes over the transport protocol and overall the environment where these messages are sent and processed.

---v

## Agnostic

XCM makes no assumptions about the nature of the Consensus System between which messages are being passed.

Notes:

XCM is not restricted to Polkadot, it's a language that can be used for communication between any systems.
For example, EVM-chains or Cosmos hubs.

---v

## Absolute

XCM assumes that the environment guarantees delivery, interpretation, and ordering of messages.

Notes:

The message format does not do much about the message possibly not being delivered.
In IBC, for example, you factor in fallibility of the transport protocol into your messages.

---v

## Asynchronous

XCMs crossing the barrier between a single consensus system cannot generally be synchronous.

XCM in no way assume that the sender will be blocking on messages.

Notes:

You can't just block execution in the middle of a block, it has to be asynchronous.
Different systems have different ways of tracking time.
No assumption of blocking for sender/receiver.

Generally, consensus systems are not designed to operate in sync with external systems.
They intrinsically need to have a uniform state to reason about and do not, by default, have the means to verify states of other consensus systems.
Thus, each consensus system cannot make any guarantees on the expected time required to deliver results; doing so haphazardly would cause the recipient to be blocked waiting for responses that are either late or would never be delivered, and one of the possible reasons for that would be an impending runtime upgrade that caused a change in how responses are delivered.

---v

## Asymmetric

XCM doesn't assume there'll be messages flowing in the other direction.

If you want to send responses, you have to make it explicitly.

Notes:

There are no results or callbacks.
Any results must be separately communicated to the sender with an additional message.
The receiver side can and does handle errors, but the sender will not be notified, unless the error handler specifically tries to send back an XCM that makes some sort of XCM that notifies status back to the origin, but such an action should be considered as constructing a separate XCM for the sole purpose of reporting information, rather than an intrinsic functionality built into XCM.
XCM is a bit like REST.
XCMP is a bit like TCP/IP but not quite.
Analogies can often hurt more than they help.

---

## 📍 Locations in XCM

<pba-flex center>

Before sending a message to another system, we need a way to address it.

<diagram class="mermaid">
graph LR
    Message(Message)
    Alice(Alice)--"?"-->Bob(Bob)
    Alice--"?"-->AssetHub(Asset Hub)
    Alice--"?"-->Pallet(Pallet)
    Alice--"?"-->SmartContract(Smart Contract)
</diagram>

Notes:

XCM defines a `Location` type that acts as a URL for consensus systems.

The `Location` type identifies any single _location_ that exists within the world of consensus.
Representing a scalable multi-shard blockchain such as Polkadot, an ERC-20 asset account on a parachain, a smart contract on some chain, etc.
It is usually represented as a location _relative_ to the current consensus system.
Relative locations are easier to handle due to the fact that the network structure can change.

Locations don't define the actual path to get there, just a way of addressing.

---v

## Interior locations

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
    Westend,
    Rococo,
    Wococo,
    Ethereum { chain_id: u64 },
    BitcoinCore,
    BitcoinCash,
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

`../Parachain(50)`

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

<diagram class="mermaid">
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

---

## Sovereign Accounts

Locations external to the local system can be represented by a local account.

We call this the **sovereign account** of that location.

They are a mapping from a `Location` to an account id.

<diagram class="mermaid">
graph TD
    Polkadot(Polkadot)-->A(A) & B(B)
    A-->Alice(Alice)
    B-->AliceSA("Alice's sovereign account")
</diagram>

Notes:

A sovereign account is an account on one system that is controlled by another on a different system.
A single account on a system can have multiple sovereign accounts on many other systems.
In this example, Alice is an account on AssetHub, and it controls a sovereign account on Collectives.

When transferring between consensus systems, the sovereign account is the one that gets the funds on the destination system.

---v

## Sovereign Accounts again

<diagram class="mermaid">
graph TD
    Polkadot(Polkadot)-->A(A) & B(B)
    A-->Alice(Alice)
    B-->AliceSA("Alice's sovereign account")
    B-->ASA("Asset Hub's sovereign account")
    A-->BSA("Collective's sovereign account")
</diagram>

---

<pba-col>

## 💰 Assets in XCM

Most messages will deal with assets in some way.

How do we address these assets?

---v

### Asset Representation

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

## Reanchoring

How do different locations reference the same asset?

<diagram class="mermaid limit size-70">
graph TD
    Polkadot(Polkadot)-->AssetHub("Asset Hub (1000)")
    Polkadot-->BridgeHub("Bridge Hub (1002)")
    AssetHub-->Alice(Alice)
    AssetHub-->AssetsPallet(Pallet Assets)
    AssetsPallet-->Asset(USDT)
</diagram>

Notes:

Locations are relative, so they must be updated and rewritten when sent to another chain, for them to be interpreted correctly.

<!-- FIXME TODO: Move elsewhere. -->

Native tokens are referenced by the location to their system.

---v

### USDT from Asset Hub

`PalletInstance(50)/GeneralIndex(1984)`

<diagram class="mermaid limit size-70">
graph TD
    Polkadot(Polkadot):::disabled-->AssetHub("📍 Asset Hub (1000)")
    Polkadot-->BridgeHub("Bridge Hub (1002)"):::disabled
    AssetHub-->Alice(Alice):::disabled
    AssetHub-->AssetsPallet(Pallet Assets)
    AssetsPallet-->Asset(USDT)
    linkStyle 0 opacity:0.3
    linkStyle 1 opacity:0.3
    linkStyle 2 opacity:0.3
    classDef disabled opacity:0.3
</diagram>

---v

### USDT from Bridge Hub

`../Parachain(1000)/PalletInstance(50)/GeneralIndex(1984)`

<diagram class="mermaid limit size-70">
graph TD
    Polkadot(Polkadot)-->AssetHub("Asset Hub (1000)")
    Polkadot-->BridgeHub("📍 Bridge Hub (1002)")
    AssetHub-->Alice(Alice):::disabled
    AssetHub-->AssetsPallet(Pallet Assets)
    AssetsPallet-->Asset(USDT)
    BridgeHub-->Polkadot
    linkStyle 1 opacity:0.3
    linkStyle 2 opacity:0.3
    linkStyle 5 stroke-dasharray:5
    classDef disabled opacity:0.3
</diagram>

---v

### Reanchoring to the rescue

<diagram class="mermaid">
graph LR
    subgraph OutgoingMessage[Outgoing message from Bridge Hub]
        USDTBridgeHub(USDT from Bridge Hub's perspective)
    end
    USDTBridgeHub--Reanchoring-->USDTAssetHub
    subgraph IncomingMessage[Incoming message in Asset Hub]
        USDTAssetHub(USDT from Asset Hub's perspective)
    end
</diagram>

---

## 🤹 Cross-consensus transfers

Notes:

The two ways of transferring assets between consensus systems are teleports and reserve transfers.

---v

### 1. Asset teleportation

<img rounded style="width: 500px;" src="./img/teleport.png" />

Notes:

Teleportation works by burning the assets on the source chain and minting them on the destination chain.
This method is the simplest one, but requires a lot of trust, since failure to burn or mint on either side will affect the total issuance.

---v

### 1.1. Example: System parachains?

<diagram class="mermaid">
graph LR
    BridgeHub(Bridge Hub)--"Trust"-->AssetHub(Asset Hub)
</diagram>

---v

### 1.2. Example: Polkadot and Kusama?

<diagram class="mermaid">
graph LR
    Polkadot(Polkadot)--"No trust"-->Kusama(Kusama)
</diagram>

---v

### 2. Reserve asset transfers

<img rounded style="width: 400px;" src="./img/reserve-tx.png" />

Notes:

Reserve asset transfers are more complicated, since they bring in a third actor called the reserve chain.
Chain A and B needn't trust each other, they only need to trust the reserve chain.
The reserve chain holds the real assets, A and B deal only with derivatives.
The transfer is made by burning derivatives from A, moving them from A's SA to B's SA in R, then minting on B.

In some cases, the sender, A, can also be the reserve for a particular asset, in which case the process is simplified, there's no burning of derivatives.
This usually happens with parachains' native tokens.

You always trust the issuer of the token to not mint infinite tokens.

---v

### 2.1. Example: Parachain native tokens

<diagram class="mermaid">
graph LR
    subgraph A [A = R]
        Sender(Sender account)--"Move X real asset"-->BSovereignAccount(B's Sovereign Account)
    end
    A--"Mint X derivatives"-->B(B)
</diagram>

Notes:

Most parachains act as the reserve for their own token.
To transfer their token to other chains, they move the real assets to a sovereign account and then tell the chain to mint equivalent derivatives.

---v

### 2.2. Example: Polkadot to Kusama

<diagram class="mermaid">
graph LR
    Polkadot(Polkadot)-->AssetHubP
    subgraph AssetHubP [Asset Hub Polkadot]
        Sender(Sender account)--"Move X real DOT"-->KusamaSovereignAccount("Kusama's sovereign account")
    end
    AssetHubP--"Mint X DOT derivatives"-->Kusama(Kusama)
</diagram>

Notes:

AssetHub Kusama acts as the reserve for KSM.
Kusama doesn't trust Polkadot to teleport KSM to it, but it does trust its own reserve, the AssetHub.
Polkadot has a sovereign account in Kusama's AssetHub with some amount of KSM.
Whenever some user in Polkadot wants to get KSM on Kusama, they just give the DOT to Polkadot and the KSM are moved from one sovereign account to another.
No new trust relationships are added.

---

## Summary

- XCM
- XCM vs XCMP
- Locations
- Sovereign Accounts
- Assets
- Reanchoring
- Cross-consensus transfers
  - Teleports
  - Reserve asset transfers

---v

## Next steps

<pba-flex center>

1. Blog series introducing XCM: Parts [1](https://medium.com/polkadot-network/xcm-the-cross-consensus-message-format-3b77b1373392), [2](https://medium.com/polkadot-network/xcm-part-ii-versioning-and-compatibility-b313fc257b83), and [3](https://medium.com/polkadot-network/xcm-part-iii-execution-and-error-management-ceb8155dd166).
2. XCM Format [repository](https://github.com/paritytech/xcm-format)
3. XCM [Docs](https://paritytech.github.io/xcm-docs/)

---v

<figure>
    <img rounded style="width: 50%;" src="./img/subscan-xcm-dashboard.png" />
    <figcaption>Source: <a href="https://polkadot.subscan.io/xcm_dashboard">Subscan</a></figcaption>
</figure>
