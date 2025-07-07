---
title: Limitations of Polkadot-JS
description: Understanding the challenges and limitations that emerged as the ecosystem evolved
duration: 1 hour
---

# Limitations of Polkadot-JS

---

### Poor Light Client Support

<pba-flex center>

- Heavy reliance on centralized nodes for queries
- Limited support for true decentralized applications
- Challenges for mobile and resource-constrained environments

</pba-flex>

Notes:

One of the most significant limitations of Polkadot-JS is its poor support for light clients. The library is designed to work with full RPC nodes, which typically means relying on centralized infrastructure providers.

This dependency on centralized nodes undermines the decentralized nature of blockchain applications. If these nodes go down or are compromised, applications built on Polkadot-JS may become unavailable or vulnerable.

The lack of efficient light client support also creates challenges for mobile applications and other resource-constrained environments, where downloading and processing large amounts of blockchain data is impractical.

Newer tools and approaches are addressing this limitation by implementing more efficient light client protocols and WASM-based verification that can run directly in browsers and mobile devices.

---

### Overly Coupled Dependencies

<pba-flex center>

- Tightly interconnected packages
- Difficult to use individual components
- Increased bundle sizes for web applications
- Challenges in maintenance and updates

</pba-flex>

Notes:

Polkadot-JS consists of numerous packages that are tightly coupled with each other. This high level of interdependency means that using any single component often requires pulling in large portions of the entire library.

For web applications, this results in larger bundle sizes, which can negatively impact loading times and performance. Even simple applications may end up including substantial amounts of unused code.

The tight coupling also complicates maintenance and updates. Changes to one package often necessitate updates to several others, creating a complex web of dependencies that must be managed carefully.

This architecture made sense in the early days of the ecosystem when rapid development was prioritized over modularity, but it has become increasingly problematic as the ecosystem has matured and more specialized use cases have emerged.

---

### Legacy JSON-RPC Interface

<pba-flex center>

- Designed for full nodes, not light clients
- Runtime-dependent parameters and return values
- Lacks proper DoS protection mechanisms
- Poorly documented edge cases and error handling
- Not load-balancer friendly
- Based on Ethereum's RPC with ad-hoc expansions

</pba-flex>

Notes:

Polkadot-JS was built around the original JSON-RPC interface for Substrate, which has several fundamental limitations:

1. Full-node assumptions: Many functions assume a full node implementation, making them inefficient or impossible to implement properly on light clients (e.g., state_getKeysPaged downloads all keys repeatedly on light clients).
2. Runtime dependencies: Functions like payment_queryInfo depend on specific runtime details, forcing tight coupling between node and runtime versions, undermining the benefits of upgradable runtimes.
3. Subscription inefficiencies: The existing subscription model doesn't properly separate concerns, making it difficult to implement efficient data streaming and resource management.
4. Lack of standardization: Inconsistent error handling across functions (some return null, others return errors for the same condition).
5. Resource management issues: No clear mechanism for clients to signal when they're done with resources, forcing servers to guess about caching strategies.

The new JSON-RPC API addresses these limitations with a consistent naming scheme, proper namespacing, explicit resource management via pinning, clear separation between archive and recent data access patterns, and improved subscription models that provide consistent views of chain state.

---

### Too Many Responsibilities

<pba-flex center>

- Acts as a wallet, RPC client, API, and UI tool
- Jack of all trades, master of none
- Difficult to optimize for specific use cases
- Challenging to maintain and evolve

</pba-flex>

Notes:

Polkadot-JS attempts to fulfill too many diverse functions, from low-level cryptography to high-level UI components. This broad scope makes it difficult to excel in any single area.

For developers with specific needs, this means either accepting suboptimal solutions or building custom implementations. Neither option is ideal, especially for teams with limited resources.

The wide range of responsibilities also complicates maintenance and evolution. Improvements to one aspect of the library might introduce regressions in others, making it challenging to implement significant changes without extensive testing and coordination.

Modern software development generally favors more focused, modular tools that do one thing well and can be combined as needed. Newer tools in the ecosystem are following this approach, providing more specialized solutions for specific use cases.

---

### High Abstraction Complexity

<pba-flex center>

- Many layers of abstraction
- Difficult debugging process
- Steep learning curve for new developers
- Challenges in understanding error sources

</pba-flex>

Notes:

Polkadot-JS employs multiple layers of abstraction to simplify common tasks and hide the complexity of the underlying blockchain protocols. While these abstractions make simple tasks easier, they can make debugging and troubleshooting significantly more complex.

When errors occur, they often happen deep within the library's internals, making it difficult to identify the root cause. The error messages may be cryptic or provide limited information about what went wrong.

This complexity creates a steep learning curve for new developers. Understanding how all the pieces fit together requires significant investment of time and effort, which can be a barrier to entry for those new to the ecosystem.

Even experienced developers may struggle to understand the full implications of certain API calls or the exact data flow through the various abstraction layers.

---

### Handling of Type Information

<pba-flex center>

- Relies on dynamic runtime metadata
- Can be brittle when chains update
- Challenges with type consistency across runtime versions
- Complex type registry management

</pba-flex>

Notes:

Substrate chains provide type information through runtime metadata, which Polkadot-JS uses to interpret the chain's data structures. This approach is flexible but introduces several challenges:

1. Type information is loaded dynamically at runtime, which can lead to errors if the metadata doesn't match the chain's current state.

2. When chains update their types, applications may need to manually update their type definitions to maintain compatibility.

3. Managing custom types across different runtime versions can become complex, especially for applications that need to work with multiple chains or historical data.

4. The type system is intricate and can be difficult to debug when issues arise.

Newer approaches are exploring ways to make type handling more robust and developer-friendly, such as generating static TypeScript definitions from chain metadata or using more structured type systems.

---

## Addressing the Limitations

<pba-flex center>

- PAPI: Next-generation TypeScript SDK with improved modularity
- Substrate Connect: Light client-focused solution
- Subxt: Rust-based alternative with strong typing
- New JSON-RPC API: Addressing fundamental protocol limitations
- Component-specific libraries for targeted use cases

</pba-flex>

Notes:

Several initiatives are underway to address these limitations:

1. PAPI (Polkadot API) is being developed as a more modular replacement for the Polkadot-JS API, with cleaner separation of concerns and improved light client support.

2. Substrate Connect provides light client functionality specifically designed for browser environments, enabling truly decentralized applications.

3. Subxt offers a Rust-based alternative for developers who prefer stronger typing and better performance characteristics.

4. The new JSON-RPC API aims to address the fundamental limitations in the protocol layer, providing a more robust foundation for client libraries.

5. More specialized, focused libraries are emerging for specific use cases, following the principle of doing one thing well.

These initiatives represent the next evolution of the Substrate development ecosystem, building on the lessons learned from Polkadot-JS while addressing its limitations.

---

## Key Takeaways

<pba-flex center>

- Polkadot-JS's design choices reflect early ecosystem priorities
- Current limitations affect scalability, maintainability, and DX
- Understanding these limitations helps with architecture decisions
- Newer tools are specifically addressing these shortcomings
- Historical support remains a unique strength despite limitations
- Gradual migration path exists for developers and applications

</pba-flex>

Notes:

Polkadot-JS's limitations are largely a product of its organic evolution and the priorities of the early Polkadot ecosystem. What began as a flexible solution for a rapidly developing blockchain platform has shown strain as the ecosystem has matured and requirements have become more specialized.

The key limitations around light client support, coupled dependencies, abstraction complexity, and the JSON-RPC interface affect scalability, maintainability, and developer experience. Understanding these limitations helps developers make informed decisions about when to use Polkadot-JS and when alternative approaches might be more appropriate.

While newer tools like PAPI and Substrate Connect are specifically addressing these shortcomings, Polkadot-JS continues to offer unmatched historical support and comprehensive functionality. This creates a gradual migration path where developers can adopt newer tools for new projects while maintaining existing Polkadot-JS implementations.

The lessons learned from Polkadot-JS are informing the design of the next generation of tools, ensuring that they maintain the strengths while addressing the limitations. This evolution reflects the maturation of the Polkadot ecosystem as a whole, as it moves from pioneering innovation to sustainable, production-ready infrastructure.

---

<!-- .slide: data-background-color="#4A2439" -->

# Questions?
