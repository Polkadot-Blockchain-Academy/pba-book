---
title: Light Clients and Unstoppable Apps
description: Light Clients and Unstoppable Apps, for web3 builders.
duration: 45+ mins
---

# Light clients<br />and<br />Unstoppable Apps

---

#### Traditional Web 2

<img rounded style="width: 80%;" src="./img/web2.png" />
<!-- .element: class="fragment" data-fragment-index="1" -->

Notes:

Before I proceed with anything, let's take a moment to see the current state of the majority of the World Wide Web as we know it.

Welcome to the realm of Web 2.0, where the majority of web applications currently reside.
While I won't be roasting anyone, it's essential to recognize that platforms like Facebook, Twitter, WhatsApp, and many others fall under this category; (Describe image)

---v

#### The Web 3 vision

<img rounded style="width: 80%;" src="./img/web3.png" />
<!-- .element: class="fragment" data-fragment-index="1" -->

Notes:

This represents the vision of what web3 should aspire to become:
a truly interconnected network where validators and end-users from all corners of the world can seamlessly connect, share information, and collaborate.

Now, show of hands:

- how many of you believe that we are close to achieving this vision at the moment?
- And how many think we still have a considerable distance to go?

---v

#### The Web 3 reality

<img rounded style="width: 80%;" src="./img/web3_reality.png" />
<!-- .element: class="fragment" data-fragment-index="1" -->

Notes:

Let's take a closer look at the reality of the situation.
As it stands, our entry into the blockchain network is channeled through a central access point, represented by a JSON-RPC node.
This node serves as the gateway to access the entire blockchain network.

While many applications claim to be decentralized, we must ask ourselves, how truly decentralized are they?

Now, I want to emphasize one crucial point - and I encourage you to take a moment to reflect on it.
I will pause there for a few seconds to let this sink in;

---v

<h1 style="font-size:7rem; font-weight: bold">Blockchain "decentralized” apps are still centralized</h1>

Notes:

I will pause there for a few seconds to let this sink in;

---v

<img rounded style="width: 30%;" src="./img/learn-student.gif" />

Notes:

I will pause there for a few seconds to let this sink in;

---

# Node types in the network

<p style="text-align: left; padding-bottom: 2rem">The type of each node depends on different characteristics:</p>

<ul>
  <li>
    <span class="font-bold underline">Validator:</span> node configured to potentially produce blocks.
  </li>
<!-- .element: class="fragment" data-fragment-index="1" -->
  <li>
    <span class="font-bold underline">JSON-RPC:</span> node which gives public access to its JSON-RPC endpoint.
  </li>
<!-- .element: class="fragment" data-fragment-index="2" -->
  <li>
    <span class="font-bold underline">Bootnode:</span> node whose address can be found in the chain specification file (chainspec).
      Necessary to kick-off the network.
  </li>
<!-- .element: class="fragment" data-fragment-index="3" -->
  <li>
    <span class="font-bold underline">Archive:</span> stores the entire state of the chain at each block since block #0.
      Useful to access historical data.
  </li>
<!-- .element: class="fragment" data-fragment-index="4" -->
  <li>
    <span class="font-bold underline text-[var(--r-heading-color)]">Light client:</span><span class="text-[var(--r-heading-color)]"> doesn’t store the entire state of the chain but requests it on demand.</span>
  </li>
<!-- .element: class="fragment" data-fragment-index="5" -->
</ul>

Notes:

Before anything else – lets remember the node types in the network

Validator Nodes: These nodes are responsible for producing new blocks and validating transactions.
They participate in the consensus mechanism and play a crucial role in securing the network.

JSON-RPC nodes: serve as an interface for developers and applications to interact with the blockchain by sending JSON-formatted requests and receiving JSON-formatted responses.

Bootnodes: Bootnodes are nodes with well-known addresses that serve as entry points for new nodes joining the network.
They help new nodes discover and connect to other peers in the network.

Light Nodes: Light nodes are a lightweight version of full nodes that do not store the entire blockchain but rely on full nodes for transaction verification.
They are useful for users who want to interact with the network without the need to download the entire blockchain.

(......After the Clicks!....)

Any combination of “validator”, “bootnode” and “JSON-RPC node” is possible, except for “light” and “archive” that are mutually incompatible.

---v

#### The reality of blockchains today

<img rounded style="width: 100%;" src="./img/reality_bc_today.png" />
<!-- .element: class="fragment" data-fragment-index="1" -->

Notes:

Here is how this is happening in reality at the moment, or how one could possibly connect to the network today

(read slides)

Make a note that: For simplicity reasons from now on I will be using the word "UI" to refer to a client/user/app etc/
Ask: WHAT ARE THE WAYS to connect to the network from a UI like (e.g. polkadotJS apps or any custom one) today?

---v

#### USER-CONTROLLED NODE

<pba-cols>
  <pba-col left>
    <div>App connects to a node client that the user has installed on their machine</div>
    <div class="bg-green-600 rounded-2xl p-4 !mt-2"><span class="font-bold">Secure</span><br />Trustless: connects to multiple nodes, verifies everything</div>
    <div class="bg-red-600 rounded-2xl p-4 !mt-2"><span class="font-bold">Convenient:</span> Works transparently</div>
  </pba-col>
  <!-- .element: class="fragment" data-fragment-index="2" -->
   <pba-col left>
      <img rounded src="./img/USER-CONTROLLED-NODE.png" />
  </pba-col>
  <!-- .element: class="fragment" data-fragment-index="1" -->
</pba-cols>

Notes:

(Read slides)

---v

#### PUBLICLY-ACCESSIBLE NODE

<pba-cols>
  <pba-col left>
    <div>App connects to a third-party-owned publicly-accessible
node client</div>
    <div class="bg-red-600 rounded-2xl p-4 !mt-2"><span class="font-bold">Centralized and insecure:</span> Publicly-accessible node can be malicious</div>
    <div class="bg-green-600 rounded-2xl p-4 !mt-2"><span class="font-bold">Convenient:</span> Works transparently</div>
  </pba-col>
  <!-- .element: class="fragment" data-fragment-index="2" -->
  <pba-col left>
    <img rounded src="./img/PUBLICLY-ACCESSIBLE-NODE.png" />
  </pba-col>
  <!-- .element: class="fragment" data-fragment-index="1" -->
</pba-cols>

Notes:

(Read slides)

---v

#### Why this needs fixing?

<pba-cols>
  <pba-col left>
      <h4>Reliability</h4>
      <!-- .element: class="fragment" data-fragment-index="1" -->
      <div>"The middleman" can stop working for a reason or another, leaving end users incapable of interacting with the blockchain.</div>
      <!-- .element: class="fragment" data-fragment-index="2" -->
  </pba-col>
  <!-- .element: class="fragment" data-fragment-index="1" -->
   <pba-col left>
    <h4>Possibility of censorship or hijacking</h4>
    <!-- .element: class="fragment" data-fragment-index="3" -->
    <div>"The middleman" can decide to ban some end users or some transactions, or can be taken control of by an attacker.</div>
    <!-- .element: class="fragment" data-fragment-index="4" -->
  </pba-col>
  <!-- .element: class="fragment" data-fragment-index="3 " -->
   <pba-col left>
     <h4>Frontrunning problem</h4>
     <!-- .element: class="fragment" data-fragment-index="5" -->
     <div>"The middleman" knows all the transactions that are submitted before they are actually applied, and can inject its own transactions ahead of time for its own monetary gains.</div>
     <!-- .element: class="fragment" data-fragment-index="6" -->
  </pba-col>
  <!-- .element: class="fragment" data-fragment-index="5" -->
</pba-cols>

Notes:

In the 3rd party case the user relies on the 3rd party node to connect to, in order to communicate with the network.
(audience) With a show of hands Why this needs fixing?
(pause and wait for possible answer)

- (we need) Reliability
- (there is a) Possibility of censorship or hijacking
- Front running is the act of placing a transaction in a queue with the knowledge of a future transaction

---v

## The reality of blockchains we want

<img rounded style="width: 100%;" src="./img/reality_we_want.png" />
<!-- .element: class="fragment" data-fragment-index="1" -->

---v

# The solution

<img rounded src="./img/feather.png" />
<!-- .element: class="fragment" data-fragment-index="1" -->

# Light Clients

<!-- .element: class="fragment" data-fragment-index="1" -->

---

# What is a light client?

<p style="font-size:4rem">It's a client (a node)...</p>

<!-- .element: class="fragment" data-fragment-index="1" -->

<p style="font-size:1.5rem">...but lighter!</p>

<!-- .element: class="fragment" data-fragment-index="2" -->

Notes:

When I joined the team of substrate connect, I asked this same question.
And the response I got was…. (\*)
Back then I was like… “Yeah – thanks I guess”

But that was actually true!

---v

# What is a light client?

<ul>
  <li>
    It is a client that is lighter than a full node, in terms of memory consumption, number of threads, and code size;</li>
<!-- .element: class="fragment" data-fragment-index="1" -->
  <li> Allows a dApp to access and interact with a blockchain in a secure and decentralized manner without having to sync the full blockchain;</li>
<!-- .element: class="fragment" data-fragment-index="2" -->
  <li> It is a node that doesn’t store the entire state of the chain but requests it on demand;</li>
<!-- .element: class="fragment" data-fragment-index="3" -->
  <li> It connects and interacts with the network in a fully trust-less way with it;</li>
<!-- .element: class="fragment" data-fragment-index="4" -->
  </li>
  <!-- .element: class="fragment" data-fragment-index="4" -->
</ul>

Notes:

In the next slides we will explain "What is a light client" in a generic manner but also I will add some extra information around the Polkadot ecosystem solution that is implemented;

Bullet 1) A "light client" is a type of node implementation that allows applications to interact with the network, consuming fewer resources compared to full nodes, making them more suitable for resource-constrained devices like mobile phones, or light enough for running in browsers (see substrate connect);

Bullet 2) Instead of maintaining a complete copy of the blockchain, the node only carries a minimal amount of data necessary for its operations (e.g.chain specs).
It relies on full nodes or other network participants to provide the additional information it needs;

Bullet 3) ....
based on the request it either provides the response from existing data, if any, or propagates the request to a full node and returns the response;

Bullet 4) Light clients can synchronize with the blockchain more quickly since they only need to fetch recent data, using justifications (we will talk about it in a while), reducing the time needed to get up-to-date with the network (few seconds).
They fetch less data from the network and consume less bandwidth.
This is especially advantageous for users on limited data plans or slow internet connections
---v

### Real-life example

<video controls width="100%">
    <source src="./img/LightClients.mp4" type="video/mp4">
    Sorry, your browser doesn't support embedded videos.
</video>

Notes:

"Slow internet connections": lets see a real-life example.
Time: Polkadot decoded 2022; Stage: Co-founder of Talisman wallet, Jonathan Dunne, takes the stage demo of our Light client solution (smoldot) is integrated in the wallet, and what are the benefits - using a very "questionable internet connection" which had a very bad day due to way-too-many-connected people;
Once the [talisman wallet](https://www.talisman.xyz/) loads up, pay attention to the spinners - Polkadot is loading with a light client while Kusama with the usual JSON-RPC method

Full video: <https://www.youtube.com/watch?v=oaidhA5eL_8>

---v

<h2>How does a light client know where to connect to</h2>

<pba-cols>
  <pba-col left>
    <img rounded style="width: 100%" src="./img/where_to_1.png" />
  </pba-col>
  <!-- .element: class="fragment" data-fragment-index="1" -->
  <pba-col left>
    <img rounded style="width: 100%" src="./img/where_to_2.png" />
  </pba-col>
  <!-- .element: class="fragment" data-fragment-index="2" -->
</pba-cols>

Notes:

As you probably already learned a chain specification is a configuration file that defines the parameters and initial settings for a blockchain network.

It serves as a blueprint for launching and running a new blockchain node, providing essential information to set up the network;

Our Substrate nodes can produce what is called a Chain spec which Smoldot then uses in order to spin up a light client node based on that chain-spec;
(Show the chainspec on screen)

---v

### How does a light client know what/who to trust

<img rounded style="margin-top: 150px; width: 70%" src="./img/know-who-to-trust.png" />
<!-- .element: class="fragment" data-fragment-index="1" -->

Notes:

As we know Substrate chains provides the concept of FINALITY which is VERY important for the light clients!
Once a block has been finalized, it is guaranteed to always be part of the best chain.
By extension, the parent of a finalized block is always finalized as well etc etc
For finality Substrate/Polkadot nodes use the GrandPa algorithm.
Authorized nodes, emit votes on the network, when 2/3rds or more have voted for a specific block, it effectively becomes finalized.
These votes are been collected in what is called a **justification**

**Justifications** play a crucial role in providing security and validity guarantees for light clients.
As said before, light clients are nodes that do not store blockchain's data but rely on other full nodes or the network to verify the blockchain's state and transactions.
While light clients offer reduced resource requirements and faster synchronization, they face the challenge of trusting the information they receive from other nodes.

Justifications address this trust issue for light clients by providing cryptographic proofs of the finality and validity of blocks.
When a block is justified, it means that it has been confirmed and agreed upon by a supermajority of validators, making it part of the finalized state of the blockchain.

It is also used by nodes who might not have received all the votes, or for example if they were offline, In order to verify the authenticity of the blocks;

A Light client receives these justifications and this way it verifies the authenticity of a block.

---v

<section>
  <diagram class="mermaid limit size-80">
    sequenceDiagram
        Network->>Justification: Finality and create
        Network->>Justification: Finality and create
        App->>LightClient: Wake up and sync!
        LightClient->>Justification: Hey! I'm here!
        Justification-->>LightClient: Here you go
        Justification-->>LightClient: Here you go
        Justification-->>LightClient: Here you go
        App->>LightClient: Ready?!
        LightClient->>App: Not yet! Syncing
        Justification-->>LightClient: Here you go
        LightClient-->>App: Verified and synced!
        App->>LightClient: 'right! Now gimme stuff
        LightClient->>Network: Lets talk! App wants stuff
        Network-->>LightClient: Ok then!
        LightClient-->>App: Here you go!
  </diagram>
</section>

---v

<pba-cols>
  <pba-col left>
    <h2 style="text-align: center">Full node</h2>
  </pba-col>
  <pba-col left>
    <h2 style="text-align: center">Light client</h2>
  </pba-col>
</pba-cols>
<pba-cols>
  <pba-col left>
    <div class="white, bg-[var(--r-heading-color)] text-base rounded-2xl p-4 !mt-2">Fully verifies all blocks (authenticity/validity)</div>
    <div class="white, bg-[var(--r-heading-color)] text-base rounded-2xl p-4 !mt-2">Holds all the chain’s storage in its database</div>
    <div class="white, bg-[var(--r-heading-color)] text-base rounded-2xl p-4 !mt-2">Holds all past blocks in its database</div>
    <div class="white, bg-[var(--r-heading-color)] text-base rounded-2xl p-4 !mt-2">At initial startup, can take hours to be
ready</div>
  </pba-col>

<pba-col left>
    <div class="white, bg-[var(--r-heading-color)] text-base rounded-2xl p-4 !mt-2">Only verifies the authenticity of blocks</div>
    <div class="white, bg-[var(--r-heading-color)] text-base rounded-2xl p-4 !mt-2">Requests state of the chain on demand</div>
    <div class="white, bg-[var(--r-heading-color)] text-base rounded-2xl p-4 !mt-2">No database whatsoever</div>
    <div class="white, bg-[var(--r-heading-color)] text-base rounded-2xl p-4 !mt-2">Initializes in few seconds</div>
  </pba-col>
</pba-cols>

---

<img src="./img/words.png" />
<!-- .element: class="fragment" data-fragment-index="1" -->

Notes:

Now, let's dive into the Polkadot solution of light clients for all Substrate chains.

As we progress through the slide, you might have come across or heard various terms and concepts related to light clients.

At this point, it's crucial to draw a clear distinction;
Let's proceed with a more focused and detailed exploration of light clients in the Polkadot Ecosystem.

---v

<h1>Smoldot<h1>

<div style="font-size:2.5rem; color: #fff">light client implementation from scratch</div>
  <!-- .element: class="fragment" data-fragment-index="1" -->

## rust

<!-- .element: class="fragment" data-fragment-index="2" -->
<div>
  <div style="font-size:1.75rem; color: #fff">smoldot-light-js (/wasm-node) - npm/deno</div>
  <!-- .element: class="fragment" data-fragment-index="3" -->
  <div style="font-size:1.75rem; color: #fff">smoldot (/lib) - Rust library</div>
  <!-- .element: class="fragment" data-fragment-index="4" -->
  <div style="font-size:1.75rem; color: #fff">smoldot-light (/light-base)</div>
  <!-- .element: class="fragment" data-fragment-index="5" -->
  <div style="font-size:1.75rem; color: #fff">smoldot-full-node (/full-node)</div>
  <!-- .element: class="fragment" data-fragment-index="6" -->
</div>
<!-- .element: class="fragment" data-fragment-index="3" -->

<p class="inline-table">
  <img style="width: 10rem; padding-top: 3rem" src="../../blockchain-contracts/light-clients-bridges/img/tomaka.png" />
  <!-- .element: class="fragment" data-fragment-index="7" -->
  <div style="font-size:1.5rem; color: #fff">Pierre Krieger - tomaka</div>
  <!-- .element: class="fragment" data-fragment-index="7" -->
</p>
<!-- .element: class="fragment" data-fragment-index="7" -->

<a href="https://github.com/smol-dot/smoldot/">https://github.com/smol-dot/smoldot/</a>

<!-- .element: class="fragment" data-fragment-index="8" -->

Notes:

Smoldot - is the light client implementation from scratch - meaning, we did not make substrate lighter.
It was rewritten from scratch, in rust - and it comes with:

- smoldot-light-js (/wasm-node): A JavaScript package that can connect to a Substrate-based chains as a light client.
  Works both in the browser and in NodeJS/Deno.
  This is the main component of this repository.
- smoldot (/lib): An unopinionated Rust library of general-purpose primitives that relate to Substrate and Polkadot.
  Serves as a base for the other components.
- smoldot-light (/light-base): A platform-agnostic Rust library that can connect to a Substrate-based chain as a light client.
  Serves as the base for the smoldot-light-js component explained above.
- smoldot-full-node (/full-node): A work-in-progress prototype of a full node binary that can connect to Substrate-base chains.
  Doesn't yet support many features that the official client supports.

Powered by Pierre Krieger (a.k.a. tomaka)

---v

## Substrate Connect

<div style="font-size:1.75rem; color: #fff">uses smoldot as an implementation detail</div>
<!-- .element: class="fragment" data-fragment-index="2" -->

<div style="font-size:1.5rem; color: #d92f78; margin: 0.5rem 0">javascript/typescript</div>
<!-- .element: class="fragment" data-fragment-index="3" -->
  <img width="900" src="./img/wellknown.png"  style="margin: 1rem 15%" />
  <!-- .element: class="fragment" data-fragment-index="4" -->
<p style="font-size:1.5rem; margin-top: 1rem"><a href="https://github.com/paritytech/substrate-connect/">https://github.com/paritytech/substrate-connect/</a></p>
<!-- .element: class="fragment" data-fragment-index="5" -->

Notes:

- npm package
- rpc provider from polkadotJS
- Chrome and Mozilla extension
- Comes with 4 integrated "Well Known" chains (Kusama, Polkadot, Westend, Rococo) - which means these chains can be used without the need of providing chainspecs;

---v

## On a diagram

<section>
  <diagram class="mermaid">
    stateDiagram-v2
      Smoldot_Light_Client --> Substrate_connect
      Substrate_connect --> PolkadotJS_API
      PolkadotJS_API --> UI_dAPP

    Smoldot_Light_Client --> Custom_Code\n(with_JSON_RPC_API)
    Custom_Code\n(with_JSON_RPC_API) --> UI_dAPP

</diagram>
</section>

---

## Smoldot Light Client

- (As Substrate, it also) supports the new JSON-RPC protocol that has been developed;

<!-- .element: class="fragment" data-fragment-index="1" -->

- Light and fast enough so that it can be embedded into a mobile application or an application in general;

<!-- .element: class="fragment" data-fragment-index="2" -->

Notes:

- new JSON-RPC protocol: https://github.com/paritytech/json-rpc-interface-spec/
- As showcased in Decoded 2023 by Daan van der Plas: "Smoldot in Mobile Apps" (https://www.youtube.com/watch?v=Z7FiFHgotzE&feature=share)

We'll be using substrate connect's TS/JS code as pseudo-code for our examples

---

## Publicly Accessible Node

The dApp (UI) connects to a third-party-owned publicly-accessible node client

<p class="bg-red-600 rounded-2xl p-4 !mt-2"><span class="font-bold">Centralized and insecure:</span> Publicly-accessible node can be malicious</p>
<p class="bg-green-600 rounded-2xl p-4 !mt-2"><span class="font-bold">Convenient:</span> Works transparently</p>

---v

## So what one needs to do

- Find the web-socket url of a 3rd party node (JSON-RPC node) that one trusts;

<!-- .element: class="fragment" data-fragment-index="1" -->

- Add it to the code and use it;

<!-- .element: class="fragment" data-fragment-index="2" -->

---v

## In your dApp

```javascript[0|1|3-5|7-9]
import { ApiPromise, WsProvider } from "@polkadot/api";

// Maybe some more code that does some magic here
const provider = new WsProvider("wss://westend-rpc.polkadot.io");
const api = await ApiPromise.create({ provider });

// Interact using polkadotJS API
const header = await api.rpc.chain.getHeader();
const chainName = await api.rpc.system.chain();
```

---

## User-Controlled Node

The dApp (UI) connects to a node client that the user has installed on their machine

<p class="bg-green-600 rounded-2xl p-4 !mt-2"><span class="font-bold">Secure Trustless:</span> connects to multiple nodes, verifies everything</p>
<p class="bg-red-600 rounded-2xl p-4 !mt-2"><span class="font-bold">Inconvenient:</span> Needs an installation process and having a node up and running, plus maintenance effort</p>

---v

## So what one needs to do

<pba-flex center>

1. Install dependencies<br />
   (e.g. rust, openssl, cmake, llvm etc);

<!-- .element: class="fragment" data-fragment-index="1" -->

1. Clone from github the "polkadot" repo;

<!-- .element: class="fragment" data-fragment-index="2" -->

1. Build the node locally;

<!-- .element: class="fragment" data-fragment-index="3" -->

1. Start the node locally;

<!-- .element: class="fragment" data-fragment-index="4" -->

1. Wait for the node to synchronize;
   <!-- .element: class="fragment" data-fragment-index="5" -->
   <pba-flex>

---v

<p>...wait for the node to synchronize...</p>
<!-- .element: class="fragment" data-fragment-index="1" -->
<p>.......</p>
<!-- .element: class="fragment" data-fragment-index="2" -->
<p>..........</p>
<!-- .element: class="fragment" data-fragment-index="3" -->
<p>..................</p>
<!-- .element: class="fragment" data-fragment-index="4" -->
<p>......wait for it.......</p>
<!-- .element: class="fragment" data-fragment-index="5" -->
<p>..............................</p>
<!-- .element: class="fragment" data-fragment-index="6" -->
<p>ok</p>
<!-- .element: class="fragment" data-fragment-index="7" -->

---v

## In your dApp

```javascript[|1|3-5|7-9]
import { ApiPromise, WsProvider } from "@polkadot/api";

// Maybe some more code that does some magic here
const provider = new WsProvider("wss://127.0.0.1:9944");
const api = await ApiPromise.create({ provider });

// Interact using polkadotJS API
const header = await api.rpc.chain.getHeader();
const chainName = await api.rpc.system.chain();
```

---

## Light Client in the Browser

The uApp (UI) connects to an _integrated_ light client

<p class="bg-green-600 rounded-2xl p-4 !mt-2"><span class="font-bold">Secure Trustless:</span> connects to multiple nodes, verifies everything</p>
<p class="bg-green-600 rounded-2xl p-4 !mt-2"><span class="font-bold">Convenient:</span> Works transparently</p>

---v

## So what one needs to do

<pba-flex center>

1. Install and configure the light client inside the dApp

</pba-flex>

---v

## With PolkadotJS API

```javascript[0|1-2|4-7|9-11]
import { ScProvider } from "@polkadot/rpc-provider/substrate-connect";
import * as Sc from '@substrate/connect';

// Maybe some more code that does some magic here
const provider = new ScProvider(Sc, Sc.WellKnownChain.westend2);
await provider.connect();
const api = await ApiPromise.create({ provider });

// Interact using polkadotJS API
const header = await api.rpc.chain.getHeader();
const chainName = await api.rpc.system.chain();
```

---v

### Or even without PolkadotJS API

```javascript[0|1|4|5-10|12-15]
import { createScClient, WellKnownChain } from "@substrate/connect";

// Maybe some more code that does some magic here
const scClient = createScClient();
const mainChain = await scClient.addWellKnownChain(
  WellKnownChain.polkadot,
  jsonRpcCallback = (response) {
    console.log(response);
  }
);

// Communicate with the network
mainChain.sendJsonRpc(
  '{"jsonrpc":"2.0","id":"1","method":"chainHead_unstable_follow","params":[true]}',
);
```

---v

### Or even without PolkadotJS API

### and with a Custom Chainspec

```javascript[0|1|2,4|7|8,13|9|10-12| 15-18]
import { createScClient, WellKnownChain } from "@substrate/connect";
import myLovelyChainspec from './myLovelyChainspecFromSubstrateChain.json';

const myLovelyChainspecStringified = JSON.stringify(myLovelyChainspec);

// Maybe some more code that does some magic here
const scClient = createScClient();
const mainChain = await scClient.addChain(
  myLovelyChainspecStringified,
  jsonRpcCallback = (response) {
    console.log(response);
  }
);

// Communicate with the network
mainChain.sendJsonRpc(
  '{"jsonrpc":"2.0","id":"1","method":"chainHead_unstable_follow","params":[true]}',
);
```

---v

### Or even only with smoldot

```javascript[0|1|3|5,13|6-12|15|17|18|20|21]
import * as smoldot from "smoldot";

const chainSpec = new TextDecoder("utf-8").decode(fs.readFileSync('./westend-chain-specs.json'));

const client = smoldot.start({
    maxLogLevel: 3,  // Can be increased for more verbosity
    forbidTcp: false,
    forbidWs: false,
    forbidNonLocalWs: false,
    forbidWss: false,
    cpuRateLimit: 0.5,
    logCallback: (_level, target, message) => console.log(_level, target, message)
});

client.addChain({ chainSpec, disableJsonRpc: true });

console.log('JSON-RPC server now listening on port 9944');
console.log('Please visit: https://cloudflare-ipfs.com/ipns/dotapps.io/?rpc=ws%3A%2F%2F127.0.0.1%3A9944');

// Now spawn a WebSocket server in order to handle JSON-RPC clients.
// See JSON-RPC protocol: https://github.com/paritytech/json-rpc-interface-spec/
```

---

# Some demo maybe…?

<p class="inline-table">
  <img src="./img/code.jpg" />
</p>
---

#### Known vulnerabilities

- Eclipse attacks (Full node & Light client)

<!-- .element: class="fragment" data-fragment-index="1" -->

- Long-range attacks (Full node & Light client)

<!-- .element: class="fragment" data-fragment-index="2" -->

- Invalid best block (Only Light client)

<!-- .element: class="fragment" data-fragment-index="3" -->

- Finality stalls (Mostly Light client)

<!-- .element: class="fragment" data-fragment-index="4" -->

Notes:

Stay with me - the next is the last but not the easiest part:

- **Eclipse attacks (full nodes and light clients both affected)**.
  Blockchain is a P2P network - and Smoldot tries to connect to a variety of nodes of this network (from the bootnodes).
  Imagine if all these nodes were to refuse sending data back, that would isolate smoldot from the network - The way that smoldot learns which nodes exist, is from the nodes themselves (bootnodes).
  If smoldot is only ever connected to malicious nodes, it won't ever be able to reach non-malicious nodes - if the list of bootnodes only contains malicious nodes, smoldot will never be able to reach any non-malicious node.
  If the list of bootnodes contains a single honest node, then smoldot will be able to reach the whole network.
  !!! this attack is effectively a denial-of-service, as it will prevent smoldot from accessing the blockchain!

- **Long-range attacks (full nodes and light clients both affected)**.
  If more than 2/3rds of the validators collaborate, they can fork a chain, starting from a block where they were validator, even if they are no longer part of the active validators at the head of the chain.
  If some validators were to fork a chain, the equivocation system would punish them by stealing their staked tokens.
  However, they cannot be punished if they unstake their tokens (which takes 7 days for Kusama or 28 days for Polkadot) before creating the fork.

  If smoldot hasn't been online since the starting point of the fork, it can be tricked (through an eclipse attack) into following the false fork.
  In order to not be vulnerable, smoldot shouldn't stay offline for more than the unstaking delay time (as said 7 days for Kusama or 28 days for Polkadot) in a row.
  Alternatively, smoldot isn't vulnerable if the checkpoint provided in the chain specification, is not older than the unstaking delay.

  Given that this attack -> requires the collaboration of many validators, -> is "all-in", -> is detectable ahead of time, -> it requires being combined with an eclipse attack, and that it doesn't offer any direct reward, it is considered not a realistic threat.

- **Invalid best block (light clients only)**.
  Light clients don't verify validity but only authenticity of blocks.

  A block is authentic if it has been authored by a legitimate validator, at a time when it was authorized to author a block.
  A validator could author a block that smoldot considers as authentic, but that contains completely arbitrary data.

  Invalid blocks aren't propagated by honest full nodes on the gossiping network, but it is possible for the validator to send the block to the smoldot instance(s) that are directly connected to it or its complicit.
  While this attack requires a validator to be malicious and that it doesn't offer any direct reward it is unlikely to happen, but it is still a realistic threat.
  For this reason, when using a light client, do not assume any storage data coming from a best, that hasn't been finalized yet to be accurate.

  Once a block has been finalized, it means that at least 2/3rds of the validators consider the block valid.
  While it is still possible for a finalized block to be invalid, this would require the collaboration of 2/3rds of the validators.
  If that happens, then the chain has basically been taken over, and whether smoldot shows inaccurate data **doesn't really matter anymore**.

- **Finality stalls (mostly light clients)**.
  Because any block that hasn't been finalized yet can become part of the canonical chain in the future, a node, in order to function properly, needs to keep track of all the valid (for full nodes) or authentic (for light clients) non-finalized blocks that it has learned the existence of.
  Under normal circumstances, the number of such blocks is rather low (typically 3 blocks).
  If, however, blocks cease to be finalized but new blocks are still being authored, then the memory consumption of the node will slowly increase over time for each newly-authored block until there is no more memory available and the node is forced to stop.
  Substrate mitigates this problem by forcing blocks authors to gradually slow down the blocks production when the latest known finalized block is too far in the past.
  Since it is normally not possible for finality to stall unless there is a bug or the chain is misconfigured, this is not really an attack but rather the consequences of an attack.
  Full nodes are less affected by this problem because they typically have more memory available than a light client, and have the possibility to store blocks on the disk.

---

<!-- .slide: data-background-color="#4A2439" -->

# Questions
