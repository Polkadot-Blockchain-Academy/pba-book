---
title: The history of contracts on Polkadot
description: Historical evolution of smart contracts and runtime modules (pallets) on Polkadot, including Frontier, ink!, and pallet-revive.
duration: 15 minutes
---

# dAppChains vs. Contracts

Notes:

Intro  
We'll trace the historical evolution of execution environments on Polkadot, focusing explicitly on smart contract and pallet development. Specifically: Frontier (initial EVM compatibility), ink! (WASM-based contracts), and recent developments like pallet-revive (RISC-V compatible VM with Solidity compilation).

---

# Early Polkadot Execution Models

Initially, Polkadot had two primary approaches to application execution:

- **Runtime modules (pallets)**:

  - Native Substrate-based execution.
  - Maximum efficiency, full runtime customization.
  - Integrated directly into parachain runtime.
  - Rust, steep developer learning curve.

- **Smart contracts (ink!, WASM-based)**:
  - Flexible, easier developer onboarding compared to pallets.
  - Permissionless deployments without runtime upgrades.
  - Limited initial adoption: lacked mature tooling, smaller developer community compared to Solidity.

Notes:

---

# Frontier: Naive Solidity Compatibility

- A Substrate pallet providing EVM compatibility.
- Enabled direct deployment of Solidity smart contracts on Substrate-based chains (e.g., Moonbeam).
- Immediately leveraged existing Ethereum developer tooling (MetaMask, Remix, Truffle).
- Dramatically accelerated adoption by tapping into Solidity's mature ecosystem.

Limitations:

- Frontier initially naïve: straightforward EVM compatibility layer, limited deeper runtime integration.
- Efficiency less than native pallets or pure WASM execution.

Notes:

---

# ink!: WASM Smart Contracts

ink! aimed to provide native WASM smart contracts on Polkadot/Substrate:

- Technically powerful: WASM flexibility, theoretically superior performance.
- Designed explicitly for Substrate: deep runtime integration possible.

Adoption limited in practice:

- Ethereum’s Solidity community strongly entrenched.
- Limited initial tooling, smaller community engagement than Solidity.
- Despite technical advantages, ink! struggled with widespread real-world adoption.

Notes:

---

# Pallet-Revive: A RISC-V Compatible VM with Solidity Compiler Support

Recent evolution: pallet-revive introduces a new VM approach:

- RISC-V compatible VM integrated as a Substrate pallet.
- Direct compilation path from Solidity.
- Combines Solidity ecosystem advantage with native runtime efficiency improvements.

Advantages of pallet-revive:

- Improved efficiency over traditional EVM implementations (Frontier).
- Leverages existing Solidity ecosystem (tooling, community).
- Deeper integration into runtime than traditional smart contracts.

Notes:

---

# Pallets vs. Contracts: Historical Context

Throughout Polkadot’s evolution:

- **Pallets** have always represented maximum performance and customization but required steep learning curves, explicit runtime upgrades, and parachain integration.
- **Smart contracts** prioritized simplicity, ease of onboarding, and permissionless deployment but sacrificed deep customization and efficiency.

Historical lessons learned:

- Developer tooling and community adoption critically important.
- Technically superior solutions (WASM-based ink!) struggled against entrenched ecosystems (Solidity).
- Practicality and familiarity often outweighed theoretical technical advantages in developer adoption.

Notes:

---
