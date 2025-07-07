# Module 3: Blockchain Fundamentals

#### Prerequisites:
Latest stable rust compiler. You can install [the entire substrate developer environment](https://docs.substrate.io/install/).

## Learning Outcomes

This module introduces students to the shortcomings of centralized infrastructure including censorship, equity, reliability, and trustworthiness. It then demonstrates through lectures, coding activities, and games, how a peer to peer network can achieve consensus on a shared history that is recorded in the blockchain data structure.

The module is designed to 3 days of contact time lasting about 6-8 hours each day. Some out-of-class study is expected, and a graded assignment is recommended.

## Learning Outcomes
* Students will understand the goals of decentralized systems, the tradeoffs made to achieve those goals, and the place for centralized alternatives.
* Understand the importance of agreeing on history to agree on a notion of a present state.
* Students will understand the fundamentals of distributed consensus.
* Students will understand the blockchain datastructure, why each piece of data is in it, and how it tracks the history of a shared state machine.
* Students will be able to start a blockchain on their own.
* Students will be able to write blockchain related code.
* Students will be able to demonstrate how the grandpa finality gadget works.
* Students will understand how to grandpa finality gadget work.

## Hands on Activities

- The primary coding activity is [Blockchain From Scratch](https://github.com/JoshOrndorff/blockchain-from-scratch/). Each student should close this repository to work through it in and after class. This will occupy 8-10 hours of class time.

- Grandpa Board Game - TODO find the repo, it may have gotten deleted. For now the content was copied into https://polkadot-blockchain-academy.github.io/pba-book/blockchain-contracts/_materials/grandpa-board-game.html

- Start a Blockchain and Fork Off - This activity is written up in [its own repo](https://github.com/Polkadot-Blockchain-Academy/Academy-PoW).

## Schedule

### Day 1

#### Morning

- 🗣️ [Coordination and Centralization: Past, Present, Future](./1-Coordination_And_Centralization-slides.md)
- 🗣️ [P2P Networks](./2-P2P_Networks-slides.md)
- ☕ Break
- 🗣️ [State Machines](./2.5-State_Machines-slides.md)
- ⌨️ Begin working on [Blockchain from Scratch](https://github.com/JoshOrndorff/blockchain-from-scratch/)

#### Afternoon

- 🗣️ [Blockchain Datastructure](./3-Blockchain_Structure-slides.md)
- ⌨️ Continue BFS
- ☕ Break
- 🗣️ [Accounts vs UTXOs](./4-Accounts_and_UTXOs-slides.md)
- ⌨️ Continue BFS

### Day 2

#### Morning

- 🗣️ [Consensus Part 1 - Author Selection](./5-Consensus-Authoring-slides.md)
- ⌨️ Continue BFS
- ☕ Break
- 🗣️ [Economics and Game Theory in Blockchain](./6-Econ_and_Game_Theory_in_Blockchain-slides.md)
- ⌨️ Continue BFS

#### Afternoon

- ⌨️ Start A Blockchain and Perform Forks ([Repo](https://github.com/Polkadot-Blockchain-Academy/Academy-PoW))
- 🗣️ [Types of Forks](./7-Forks-slides.md)
- ☕ Break
- 🗣️ Randomness in Blockchain
- ⌨️ Continue BFS

### Day 3

- 🗣️ [Consensus Part 2 - Finality](./8-Consensus_Finality-slides.md)
- ☕ Break
- 🎲 [Grandpa Board Game](https://polkadot-blockchain-academy.github.io/pba-book/blockchain-contracts/_materials/grandpa-board-game.html) TODO Find proper repo

### Day 3

#### Morning

- 🗣️ Maybe Rollups, and Blockspace vs Blobspace
- ⌨️ Continue BFS

#### Afternoon

- 🗣️ [Light Clients Bridges](./9-Light_Clients_and_Bridges-slides.md)
- ⌨️ Continue BFS
- ☕ Break
- 🗣️ Guest Speaker (Jonas - Economics of Polkadot)
