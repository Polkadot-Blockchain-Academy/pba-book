---
title: When Might You Want to Use Polkadot-JS?
description: When Might You Want to Use Polkadot-JS
duration: 30 - 45 minutes
---

## When Might You Want to Use Polkadot-JS?

---

### Historical Metadata Queries

- Essential for analyzing past chain state
- Comprehensive support for all historical runtime versions
- Crucial for block explorers and data analysis tools

Notes:

One of the standout features of Polkadot-JS is its comprehensive support for historical metadata. This makes it the tool of choice for applications that need to analyze or display data from any point in the chain's history.

Block explorers, data analysis tools, and historical auditing applications all benefit from this capability. No other tool currently provides the same level of historical compatibility, making Polkadot-JS irreplaceable for these use cases.

As chains evolve and undergo runtime upgrades, the structure of their data and the methods for interacting with them change. Polkadot-JS maintains backward compatibility with all these changes, allowing developers to decode and interpret data from any block height.

---

### As a Reference or Utility

- Useful for learning Substrate interactions before implementing custom solutions
- Provides working examples of common blockchain operations
- Source code serves as documentation for Substrate interaction patterns

Notes:

Even when building custom solutions, Polkadot-JS serves as an invaluable reference. Its source code effectively documents how to interact with Substrate chains at a low level, making it easier to understand the underlying protocols and data structures. That being said, there are definitely exceptions (rpc interface, extension interface, etc).

Many developers start by prototyping their applications with Polkadot-JS before implementing more specialized solutions. This approach allows them to validate their concepts quickly before investing in custom development.

The various utilities provided by Polkadot-JS, particularly in the @polkadot/util and @polkadot/util-crypto packages, remain useful even when building applications that don't use the full API. These well-tested utilities save developers from having to reimplement common cryptographic and formatting functions.

---

### Legacy System Maintenance

- Many existing tools still rely on Polkadot-JS
- Understanding the library is crucial for maintaining these systems
- Required for debugging issues in Polkadot-JS-dependent applications

Notes:

A significant portion of the Polkadot ecosystem was built with Polkadot-JS, and these systems will continue to require maintenance for years to come. Understanding Polkadot-JS is essential for developers tasked with maintaining or updating these existing applications.

When issues arise in these systems, debugging often requires a deep knowledge of how Polkadot-JS interacts with the blockchain. Without this understanding, diagnosing and fixing problems becomes significantly more challenging.

Even as new applications increasingly adopt newer tools, the legacy of Polkadot-JS will persist in the ecosystem for the foreseeable future, ensuring that expertise in this library remains valuable.

---

### For Comprehensive Development UIs

- Polkadot.js Apps remains the most generalized UI for chain interaction
- Provides a sandbox for testing chain functionality
- Polkadot.js Extension for a airgapped developer first extension

Notes:

The Polkadot.js Apps interface remains one of the most comprehensive UIs for interacting with Substrate-based chains. It provides access to virtually every aspect of chain functionality, from basic transfers to complex governance operations.

This interface serves as an invaluable sandbox for testing and debugging chain functionality. Developers can use it to verify that their on-chain logic works as expected before integrating it into their own applications.

While more specialized and user-friendly interfaces exist for specific chains and use cases, none match the comprehensiveness of Polkadot.js Apps for development and debugging purposes.

<!-- .slide: data-background-color="#4A2439" -->

# Questions?
