---
title: JSON-RPC Spec
description: JSON-RPC Spec
---

# JSON-RPC Spec

---

# JSON-RPC Spec

## What you will learn:

- What is JSON-RPC (v2)<!-- .element: class="fragment" -->
  - Its stateless design<!-- .element: class="fragment" -->
  - Conventions to make it stateful<!-- .element: class="fragment" -->
- (Substrate) JSON-RPC Spec<!-- .element: class="fragment" -->
  - Objectives<!-- .element: class="fragment" -->
  - Versioning<!-- .element: class="fragment" -->
  - Groups of functions<!-- .element: class="fragment" -->
  - Overview<!-- .element: class="fragment" -->

---

## JSON-RPC 2.0

- JSON-RPC is a stateless, transport agnostic, light-weight remote procedure call (RPC) protocol.<!-- .element: class="fragment" -->
- Defines basic data-structures and the rules around their processing. It is transport agnostic.<!-- .element: class="fragment" -->

---

## JSON-RPC 2.0 - Request Object

- Must have an `id` property

<!-- .element: class="fragment" -->

- Must have a `method`

<!-- .element: class="fragment" -->

- May have `params`

<!-- .element: class="fragment" -->

---

## JSON-RPC 2.0 - Notification Object

- Like `Request` but without an `id`

---

## JSON-RPC 2.0 - Response

- Must have an `id`

<!-- .element: class="fragment" -->

- Must have either a `result` or an `error`

<!-- .element: class="fragment" -->

- If it has an `error`, it must have the following properties:
  - `code`: number that indicates the type of error. Error codes from -32768 to -32000 are reserved for pre-defined errors.
  - `message`: string providing a short description.
  - `data`: optional data structure with additional information.

<!-- .element: class="fragment" -->

---

## JSON-RPC 2.0 Examples

https://www.jsonrpc.org/specification#examples

---

## JSON-RPC 2.0 For stateful connections and subscriptions

- The client acts as a server<!-- .element: class="fragment" -->
- The server acts as a client<!-- .element: class="fragment" -->
- Use opaque ids that are only relevant in the context of their connection<!-- .element: class="fragment" -->

---

# Why a New JSON-RPC API?

- Standardizing JSON-RPC requests across the Polkadot ecosystem<!-- .element: class="fragment" -->
- Removing inconsistencies<!-- .element: class="fragment" -->
- Providing better support for light clients and alternative runtimes<!-- .element: class="fragment" -->

**Notes:**

- The previous JSON-RPC implementation had inconsistencies that made it harder for developers to maintain compatibility across parachains.
- A key motivation was to improve support for lightweight clients, which require a more optimized and reliable data-fetching mechanism.

---

# JSON-RPC API Objectives

- Accommodate multiple audiences<!-- .element: class="fragment" -->
- Ensure clarity, efficiency, and scalability<!-- .element: class="fragment" -->
- Address needs for security, reliability, and flexibility<!-- .element: class="fragment" -->

**Notes:**

- The JSON-RPC API is designed to cater to a diverse range of audiences, from application developers to node operators.
- It aims to provide a robust and efficient interface for blockchain interaction while maintaining security and clarity.
- The interface emphasizes performance and standardization, ensuring efficient communication across various use cases.

---

# Key Changes & Improvements

- **Groups of functions:** Based on node capabilities

<!-- .element: class="fragment" -->

- **Stability and versioning:** Allowing improvements without breaking contracts.

<!-- .element: class="fragment" -->

- **Better Error Handling:** Clearer and documented errors.

<!-- .element: class="fragment" -->

- **Load Balancer Friendly:** A load balancer can move a client from one server to another (and thus shut down servers that it doesn’t need anymore).

<!-- .element: class="fragment" -->

**Notes:**

- Method names have been changed to be more descriptive and standardized.

- Errors are now structured in a way that makes debugging and handling failures easier.

- The API reduces redundant calls, leading to lower latency and better efficiency.

---

# Grouping Functions & Node Capabilities

- Functions are grouped using a prefix with a version number (e.g., `chainHead_v1_follow`)

<!-- .element: class="fragment" -->

- Node types: Full, Light, Archive, Plain databases

<!-- .element: class="fragment" -->

- Capability detection via the `rpc_methods` function

<!-- .element: class="fragment" -->

**Notes:**

- Grouping functions using prefixes with versioning ensures clear evolution and compatibility.
- Each node type supports functions based on its capabilities, ensuring efficient and relevant operations.
- Clients should always use `rpc_methods` to verify supported functions on a given node.

---

# Upgradability & Versioning

- Groups are versioned and self-contained (e.g., `foo_v1`, `foo_v2`)

<!-- .element: class="fragment" -->

- Higher versions indicate preferred methods, while older versions become soft-deprecated

<!-- .element: class="fragment" -->

- Functions in newer versions don't overlap with older versions

<!-- .element: class="fragment" -->

**Notes:**

- Upgrading function groups ensures that newer functions are clearly distinct from older ones.
- This separation simplifies development and reduces confusion when interacting with nodes of different capabilities.
- Developers can choose which version to rely on, knowing the functional boundaries.

---

# Unstable Functions

- Marked with the `unstable` version prefix

<!-- .element: class="fragment" -->

- No stability guarantees—functions can change without warning

<!-- .element: class="fragment" -->

- Useful for experimental or debugging utilities

<!-- .element: class="fragment" -->

**Notes:**

- Unstable functions are meant for experimental use and may evolve or be removed at any time.
- They are helpful for developers needing temporary functions for debugging or testing.
- Applications should avoid relying on unstable functions for critical features.

---

# Audience:

## End-User Applications

- Focus on reading storage and submitting transactions<!-- .element: class="fragment" -->
- Encourage and support for light-clients<!-- .element: class="fragment" -->
- Minimize DoS attack vectors<!-- .element: class="fragment" -->

**Notes:**

- End-user applications, like wallets, should prioritize using locally-run nodes to enhance security and decentralization.
- Light clients, which don't hold full blockchain storage, are encouraged for better usability.
- The API design mitigates potential DoS vulnerabilities while ensuring precise and efficient operations.

---

# Audience:

## Node Operators

- Focus on monitoring and administrative operations

<!-- .element: class="fragment" -->

- Stable functions for scripting and automation

<!-- .element: class="fragment" -->

- CLI-friendly tools (e.g., `websocat`)

<!-- .element: class="fragment" -->

**Notes:**

- Node operators require tools to monitor and manage nodes efficiently.
- Stability in API functions is essential for reliable scripting and automation.
- Tools like `websocat` facilitate interaction with JSON-RPC through WebSockets.

---

# Audience:

## Oracles & Bridges

- Automated interaction with the blockchain<!-- .element: class="fragment" -->
- Similar requirements as end-user applications<!-- .element: class="fragment" -->
- Focus on stability and reliability<!-- .element: class="fragment" -->

**Notes:**

- Oracles and bridges require reliable and automated blockchain interaction.
- Although automated, their operational needs align with those of end-user-facing applications.
- Ensuring security and stability is paramount.

---

# Audience:

## Archivers / Indexers

- Access to historical blockchain data<!-- .element: class="fragment" -->
- Focus on finalized blocks<!-- .element: class="fragment" -->
- Stable and easy-to-use functions<!-- .element: class="fragment" -->

**Notes:**

- Archivers need to access and analyze past blockchain states, focusing on finalized data.
- The API ensures stability and ease of use, simplifying data retrieval for archival purposes.
- Performance is less critical, but stability and reliability are key.

---

# Overview:

- chainhead<!-- .element: class="fragment" -->
- archive<!-- .element: class="fragment" -->
- chainSpec<!-- .element: class="fragment" -->
- sudo\_\*<!-- .element: class="fragment" -->
- transaction<!-- .element: class="fragment" -->
- transactionWatch<!-- .element: class="fragment" -->

---

# Questions & Discussion

- Feel free to ask any questions!

<!-- .element: class="fragment" -->

- Check the full objectives: [Parity Spec Link](https://paritytech.github.io/json-rpc-interface-spec/objectives.html)

<!-- .element: class="fragment" -->

- Further reading: [New JSON-RPC API mega Q&A](https://forum.polkadot.network/t/new-json-rpc-api-mega-q-a/3048)

<!-- .element: class="fragment" -->

**Notes:**

- Encourage the audience to ask questions about their specific use cases or integration concerns.
- Refer to the full objectives document for detailed insights and ongoing updates.
