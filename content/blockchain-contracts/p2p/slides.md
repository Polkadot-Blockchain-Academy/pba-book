---
title: Peer-to-Peer (P2P) Networking
description: Peer-to-Peer (P2P) networking for web3 builders
---

# Peer-to-Peer Networking

---

## Introduction/Agenda

- History of p2p networks <!-- .element: class="fragment" data-fragment-index="1" -->
- Discuss the network layer and network conditions that blockchains operate on(Mostly) <!-- .element: class="fragment" data-fragment-index="2" -->
- Talk about traditional web2 network overlays pros vs cons with web3 network overlays <!-- .element: class="fragment" data-fragment-index="3" -->
- Discuss attacks and how to address along with the underlying threat model <!-- .element: class="fragment" data-fragment-index="4" -->

---

## ARPANET

- First operational packet-switching network <!-- .element: class="fragment" data-fragment-index="2" -->
- Developed in the late 1960s by DARPA(The Defense Advanced Research Projects Agency) <!-- .element: class="fragment" data-fragment-index="3" -->
- Laid the foundation for the modern internet <!-- .element: class="fragment" data-fragment-index="4" -->

Notes:

Total Information Awareness (TIA): In the early 2000s, DARPA initiated the TIA program aimed at developing technologies for mass surveillance and data analysis. The project raised concerns about privacy and civil liberties, eventually leading to its cancellation in 2003 due to public outcry.

---

## Packet Switching

- Mode of data transmission in which a message is broken into a number of parts that are sent independently(Packets) <!-- .element: class="fragment" data-fragment-index="2" -->
- Packets are sent over whatever route is optimal <!-- .element: class="fragment" data-fragment-index="3" -->
- Packets are reassembled at the destination <!-- .element: class="fragment" data-fragment-index="4" -->

---

## Packet Switching

<img style="width: 600px" src="./img/message_packet.svg" />

Notes:

Mention that headers contain some addressing, destination information, and ordering typically depending

---

## Packet Switching

<img style="width: 600px" src="./img/packet_switching_1.svg"  />

---

## Packet Switching

<img  style="width: 600px" src="./img/packet_switching_2.svg" />

---

## Packet Switching

<img style="width: 600px" src="./img/packet_switching_3.svg" />

---

## Packet Switching

<img style="width: 600px" src="./img/packet_switching_4.svg" />

---

## Peer-to-Peer (P2P) Networks

- P2P is a decentralized form of network structure <!-- .element: class="fragment" data-fragment-index="2" -->
- Unlike client-server model, all nodes (peers) are equal participants <!-- .element: class="fragment" data-fragment-index="3" -->
- Data is shared directly between systems without a central server <!-- .element: class="fragment" data-fragment-index="4" -->
- Peers contribute resources, including bandwidth, storage space, and processing power <!-- .element: class="fragment" data-fragment-index="5" -->

---

## Historical P2P applications

Notes:

Napster, Limewire

---

## Napster

- Launched in 1999, popular P2P platform <!-- .element: class="fragment" data-fragment-index="2" -->
- Central server for indexing, P2P for transfers <!-- .element: class="fragment" data-fragment-index="3" -->
- Shutdown in 2001 due to legal issues <!-- .element: class="fragment" data-fragment-index="4" -->

Notes:

Napster's story is closely tied with the band Metallica.
In 2000, Metallica discovered that a demo of their song "I Disappear" was being circulated via Napster before its official release.
This led to Metallica filing a lawsuit against Napster for copyright infringement.
Napster had to comply by banning hundreds of thousands of users from their platform who were sharing Metallica's music.
This was a turning point in digital copyright law and played a significant role in Napster's eventual shutdown in 2001.

---

## Napster Setup

<img style="width: 600px" src="./img/napster_1.svg" />

---

## Napster Setup

<img style="width: 600px" src="./img/napster_2.svg" />

---

## Napster Setup

<img style="width: 600px" src="./img/napster_3.svg" />

---

## Napster Setup

<img style="width: 600px" src="./img/napster_4.svg" />

---

## Gnutella(Limewire)

- Each node serves as both a client and a server no central server <!-- .element: class="fragment" data-fragment-index="2" -->
- Query all connected nodes for files <!-- .element: class="fragment" data-fragment-index="3" -->
- Gain peer connections to the network via Bootnodes <!-- .element: class="fragment" data-fragment-index="4" -->
- Ordered to shutdown in 2010 by United States Court <!-- .element: class="fragment" data-fragment-index="5" -->

Notes:

- Check local filestore for file and if it is not available, forward the request to all connected peers.
- Gnutella generates a significant amount of network traffic by flooding the network with requests.

---

<section>
    <h2>Client-Server vs Peer-to-Peer (P2P) Networks</h2>
    <table>
        <thead>
            <tr>
                <th></th>
                <th>Client-Server Network</th>
                <th>P2P Network</th>
            </tr>
        </thead>
        <tbody>
            <tr class="fragment">
                <td>Structure</td>
                <td>Centralized: One or more central servers control the network</td>
                <td>Decentralized: All nodes (peers) participate equally</td>
            </tr>
            <tr class="fragment">
                <td>Data Flow</td>
                <td>Server provides data to clients</td>
                <td>Peers directly share data with each other</td>
            </tr>
            <tr class="fragment">
                <td>Resource Management</td>
                <td>Servers manage resources and control access</td>
                <td>Peers contribute resources including bandwidth, storage space, and processing power</td>
            </tr>
            <tr class="fragment">
                <td>Scalability</td>
                <td>Can be limited by server capacity</td>
                <td>Highly scalable due to the distribution of resources</td>
            </tr>
            <tr class="fragment">
                <td>Security</td>
                <td>Centralized security measures, single point of failure</td>
                <td>Potential for some security issues, malware(Depending on how it is implemented)</td>
            </tr>
        </tbody>
    </table>
</section>

---

## Centralized vs Decentralized Networks

<img style="width: 600px" src="./img/client_server_1.svg" />

Notes:

Talk about how when a partition happens in P2P vs Centralized.
In p2p, only one node needs to have a full copy in order for the file to be able to be distributed across the network.

---

## Centralized vs Decentralized Networks

<img style="width: 600px" src="./img/client_server_2.svg" />

---

## Centralized vs Decentralized Networks

<img style="width: 600px" src="./img/p2p_topology_1.svg" />

---

## Centralized vs Decentralized Networks

<img style="width: 600px" src="./img/p2p_topology_2.svg" />

---

## Advantages to Decentralized Networks

- No privileged nodes <!-- .element: class="fragment" data-fragment-index="2" -->
- Less bottlenecks with bandwidth <!-- .element: class="fragment" data-fragment-index="3" -->
- DOS resistant <!-- .element: class="fragment" data-fragment-index="4" -->
- No centralized infrastructure necessary (Except internet for now...) <!-- .element: class="fragment" data-fragment-index="5" -->

Notes:

1. No single node or nodes (CDN) have access to all of the content or files or is critical for operating the network.
   Each node has a copy of the data.
1. No central node carrying all of the load of traffic.
   Block production and Block peering/importing can be mentioned here.
1. Difficult to overload the network or DOS (Not a single node is privileged).
1. Although many nodes are run on Centralized cloud compute platforms, they don't have to be (Typically).

---

## Difficulties or Disadvantages

- Since it is permissionless, a node can share malicious resources <!-- .element: class="fragment" data-fragment-index="2" -->
- Latency <!-- .element: class="fragment" data-fragment-index="3" -->
- Difficult to regulate illicit activity <!-- .element: class="fragment" data-fragment-index="4" -->
- The network is limited by nodes with the weakest hardware <!-- .element: class="fragment" data-fragment-index="5" -->

Notes:

1. Latency may be an issue if we need to wait for many peers to receive the data produced from a single node since everyone may not have a direct connection.
   Mention finality time!
1. No central point to go and snoop all users data (for better or for worse).
1. Why we have hardware requirements for blockchain networks.

---

## Gossip Protocol

<img style="width: 500px" src="./img/3.7-p2p-gossip-1.svg" />

Notes:

- Talk about how we have and want block 45 being peered to others

---v

## Gossip Protocol

<img style="width: 85s0px" src="./img/3.7-p2p-gossip-2.svg" />

Notes:

Talk about advertising vs just blind sending and how that can be inefficient

---

<section>
    <h2>Structured vs Unstructured P2P Networks</h2>
    <table>
        <thead>
            <tr>
                <th></th>
                <th>Structured P2P Networks</th>
                <th>Unstructured P2P Networks</th>
            </tr>
        </thead>
        <tbody>
            <tr class="fragment">
                <td>Organization</td>
                <td>Nodes are organized following specific protocols and structures (like Distributed Hash Tables)</td>
                <td>Nodes are connected in an ad-hoc manner without any particular organization</td>
            </tr>
            <tr class="fragment">
                <td>Search Efficiency</td>
                <td>Efficient search operations due to structured nature</td>
                <td>Search operations may be less efficient and can involve flooding the network</td>
            </tr>
            <tr class="fragment">
                <td>Flexibility</td>
                <td>Less flexible as changes in topology require restructuring</td>
                <td>Highly flexible as nodes can freely join, leave, and reorganize</td>
            </tr>
            <tr class="fragment">
                <td>Privacy</td>
                <td>Data location is predictable due to structured organization</td>
                <td>Greater potential for anonymity</td>
            </tr>
        </tbody>
    </table>
</section>

---

## Discovery

1. Connect to a peer <!-- .element: class="fragment" data-fragment-index="2" -->
1. Ask peer for a list of their known nodes <!-- .element: class="fragment" data-fragment-index="3" -->
1. Connect to random subset of peers from the list <!-- .element: class="fragment" data-fragment-index="4" -->
1. Repeat steps 2 and 3 <!-- .element: class="fragment" data-fragment-index="5" -->

---

## Applications

Notes:

1. What are some of the types of applications that lend themselves to this kind of network topology? Can anyone think of any?
1. File sharing(Music)?
1. Messaging and communication?

---

## Initial Discovery

- Bootnode/bootnodes (More on this later in Substrate)

Notes:

1. Must know someone who is participating in the network initially(Bootnode)

---

## Attacks

Notes:

- Can anyone think of a way to exploit some of these networks?
- What would be some things to try to take advantage of?

---

## Attacks

<img style="width: 600px" src="./img/eclipse_attack_1.svg"/>

Notes:

1. Distorts view of the healthy normal honest state of the network
1. Transaction confirmations can be fictions

---

## Attacks

<img style="width: 600px" src="./img/eclipse_attack_2.svg"/>

---

## Eclipse Attack Execution

1. Flood a target node with a bunch of malicious peer addresses <!-- .element: class="fragment" data-fragment-index="2" -->
1. The targeted node then stores these malicious peers and utilizes them when re-syncing on next bootup <!-- .element: class="fragment" data-fragment-index="3" -->
1. DOS targeted node to take it offline to force a resync with these new malicious peers <!-- .element: class="fragment" data-fragment-index="4" -->

---

## Preventing Attacks

- Restrict inbound connections in some way <!-- .element: class="fragment" data-fragment-index="2" -->
- Random selection of peers to connect with <!-- .element: class="fragment" data-fragment-index="3" -->
- Deterministic node selection (Bootnodes) <!-- .element: class="fragment" data-fragment-index="4" -->
- Restricting new nodes (Probably not what we want...) <!-- .element: class="fragment" data-fragment-index="5" -->

Notes:

1. Be wary of new connections with other nodes
1. Don't just take the most recent request for connections to avoid the flooding
1. Bootnodes with higher credibility and trust (Can be a bottleneck) - Rotate bootnodes as they are also subject to attacks

---

## Conclusion

P2P networks offer us a path forward towards applications which are more decentralized and censorship resilient
