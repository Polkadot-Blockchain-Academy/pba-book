---
title: FRAME/Pallet Hooks
description: FRAME/Pallet Hooks
duration: 1 hour
---

# 🪝 FRAME/Pallet Hooks 🪝

---

## Hooks: All In One

- Onchain / STF
  - `on_runtime_upgrade`
  - `on_initialize`
  - `poll` (WIP)
  - `on_finalize`
  - `on_idle`
- Offchain:
  - `genesis_build`
  - `offchain_worker`
  - `integrity_test`
  - `try_state`

Notes:

<https://paritytech.github.io/substrate/master/frame_support/traits/trait.Hooks.html>

---v

### Hooks: All In One

```rust
#[pallet::hooks]
impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
  fn on_runtime_upgrade() -> Weight {}
  fn on_initialize() -> Weight {}
  fn on_finalize() {}
  fn on_idle(remaining_weight: Weight) -> Weight {}
  fn offchain_worker() {}
  fn integrity_test() {}
  #[cfg(feature = "try-runtime")]
  fn try_state() -> Result<(), &'static str> {}
}

#[pallet::genesis_build]
impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
	fn build(&self) {}
}
```

Notes:

Many of these functions receive the block number as an argument, but that can easily be fetched from
`frame_system::Pallet::<T>::block_number()`

---

## Hooks: `on_runtime_upgrade`

- Called every time the `spec_version`/`spec_name` is bumped.
- Why would might you be interested in implementing this?

Notes:

Because very often runtime upgrades needs to be accompanied by some kind of state migration.
Has its own lecture, more over there.

---

## Hooks: `on_initialize`

- Useful for any kind of **automatic** operation.
- The weight you return is interpreted as `DispatchClass::Mandatory`.

---v

### Hooks: `On_Initialize`

- `Mandatory` Hooks should really be lightweight and predictable, with a bounded complexity.

```rust
fn on_initialize() -> Weight {
  // any user can create one entry in `MyMap` 😱🔫.
  <MyMap<T>>::iter().for_each(do_stuff);
}
```

<!-- .element: class="fragment" -->

---v

### Hooks: `On_Initialize`

- &shy;<!-- .element: class="fragment" --> Question: If you have 3 pallets, in which order their `on_initialize` are called?
- &shy;<!-- .element: class="fragment" --> Question: If your runtime panics `on_initialize`, how can you recover from it?
- &shy;<!-- .element: class="fragment" --> Question: If your `on_initialize` consumes more than the maximum block weight?

Notes:

- The order comes from `construct_runtime!` macro.
- Panic in mandatory hooks is fatal error. You are pretty much done.
- Overweight blocks using mandatory hooks, are possible, ONLY in the context of solo-chains. Such a
  block will take longer to produce, but it eventually will. If you have your eyes set on being a
  parachain developer, you should treat overweight blocks as fatal as well.

---

## Hooks: `on_finalize`

- Extension of `on_initialize`, but at the end of the block.
- Its weight needs to be known in advance. Therefore, less preferred compared to `on_initialize`.

```rust
fn on_finalize() {} // ✅
fn on_finalize() -> Weight {} // ❌
```

<!-- .element: class="fragment" -->

- Nothing to do with _finality_ in the consensus context.

<!-- .element: class="fragment" -->

---v

### Hooks: `on_finalize`

> Generally, avoid using it unless if something REALLY needs to be happen at the end of the block.

Notes:

Sometimes, rather than thinking "at the end of block N", consider writing code "at the beginning of block N+1"

---

## Hooks: `poll`

- The non-mandatory version of `on_initialize`.
- In the making 👷

Notes:

See: <https://github.com/paritytech/substrate/pull/14279> and related PRs

---

## Hooks: `on_idle`

- **_Optional_** variant of `on_finalize`, also executed at the end of the block.
- Small semantical difference: executes one pallet's hook, per block, randomly, rather than all
  pallets'.

---v

## The Future: Moving Away From Mandatory Hooks

- `on_initialize` -> `poll`
- `on_finalize` -> `on_idle`
- New primitives for multi-block migrations
- New primitives for optional service work via extrinsics.

Notes:

This is all in the agenda of the FRAME team at Parity for 2023.

<https://github.com/paritytech/polkadot-sdk/issues/206>
<https://github.com/paritytech/polkadot-sdk/issues/198>

---

## Recap: Onchain/STF Hooks

<diagram class="mermaid">
%%{init: {'theme': 'dark', 'themeVariables': { 'darkMode': true }}}%%
graph LR
    subgraph AfterTransactions
        direction LR
        OnIdle --> OnFinalize
    end

    subgraph OnChain
        direction LR
        Optional --> BeforeExtrinsics
        BeforeExtrinsics --> Inherents
        Inherents --> Poll
        Poll --> Transactions
        Transactions --> AfterTransactions
    end

    subgraph Optional

OnRuntimeUpgrade
end

    subgraph BeforeExtrinsics
        OnInitialize
    end

    subgraph Transactions
        Transaction1 --> UnsignedTransaction2 --> Transaction3
    end

    subgraph Inherents
        Inherent1 --> Inherent2
    end

</diagram>

Notes:

implicit in this:

Inherents are only first, which was being discussed: <https://github.com/polkadot-fellows/RFCs/pull/13>

---

## Hooks: `genesis_build`

- Means for each pallet to specify a $f(input): state$ at genesis.
- This is called only once, by the client, when you **create a new chain**.
  - &shy;<!-- .element: class="fragment" --> Is this invoked every time you run `cargo run`?
- `#[pallet::genesis_build]`.

---v

### Hooks: `genesis_build`

```rust
#[pallet::genesis_build]
pub struct GenesisConfig<T: Config> {
  pub foo: Option<u32>,
  pub bar: Vec<u8>,
}
```

<!-- .element: class="fragment" -->

```rust
impl<T: Config> Default for GenesisConfig<T> {
  fn default() -> Self {
    // snip
  }
}
```

<!-- .element: class="fragment" -->

```rust
#[pallet::genesis_build]
impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
  fn build(&self) {
    // use self.foo, self.bar etc.
  }
}
```

<!-- .element: class="fragment" -->

---v

### Hooks: `genesis_build`

- `GenesisConfig` is a composite/amalgamated item at the top level runtime.

```rust
construct_runtime!(
  pub enum Runtime where {
    System: frame_system,
    Balances: pallet_balances,
  }
);
```

```rust
struct RuntimeGenesisConfig {
  SystemConfig: pallet_system::GenesisConfig,
  PalletAConfig: pallet_a::GenesisConfig,
}
```

<!-- .element: class="fragment" -->

Notes:

<https://paritytech.github.io/substrate/master/node_template_runtime/struct.RuntimeGenesisConfig.html>

---v

### Hooks: `genesis_build`

- Recent changes moving `genesis_build` to be used over a runtime API, rather than native runtime.
- `#[cfg(feature = "std")]` in pallets will go away.

Notes:

<https://github.com/paritytech/polkadot-sdk/issues/25>

---

## Hooks: `offchain_worker`

**Fully offchain application**:

- Read chain state via RPC.
- submit desired side effects back to the chain as transactions.

**Runtime Offchain Worker**:

- &shy;<!-- .element: class="fragment" --> Code lives onchain, upgradable only in synchrony with the whole runtime 👎
- &shy;<!-- .element: class="fragment" --> Ergonomic and fast state access 👍
- &shy;<!-- .element: class="fragment" --> State writes are ignored 🤷
- &shy;<!-- .element: class="fragment" --> Can submit transactions back to the chain as well ✅
- &shy;<!-- .element: class="fragment" --> Source of many confusions!

Notes:

People have often thought that they can do magic with things with OCW, please don't. BIG warning to
founders to be careful with this!

<https://paritytech.github.io/substrate/master/pallet_examples/index.html>

---v

### Hooks: `offchain_worker`

- Execution entirely up to the client.
- Has a totally separate thread pool than the normal execution.

```
--offchain-worker <ENABLED>
    Possible values:
    - always:
    - never:
    - when-authority

--execution-offchain-worker <STRATEGY>
    Possible values:
    - native:
    - wasm:
    - both:
    - native-else-wasm:
```

---v

### Hooks: `offchain_worker`

- Threads can **overlap**, each is reading the state of its corresponding block

<img style="height: 500px" src="./img/ocw.svg"  />

<!-- .element: class="fragment" -->

Notes:

<https://paritytech.github.io/substrate/master/sp_runtime/offchain/storage_lock/index.html>

---v

### Hooks: `offchain_worker`

- &shy;<!-- .element: class="fragment" -->Offchain workers have their own **special host
  functions**: http, dedicated storage, time, etc.
- &shy;<!-- .element: class="fragment" -->Offchain workers have the same **execution limits** as
  Wasm (limited memory, custom allocator).

- &shy;<!-- .element: class="fragment" -->Source of confusion, why OCWs cannot write to state.

Notes:

These are the source of the confusion.

Word on allocator limit in Substrate Wasm execution (subject to change).

- Max single allocation limited
- Max total allocation limited.

---

## Hooks: `integrity_test`

- Put into a test by `construct_runtime!`.

```rust
__construct_runtime_integrity_test::runtime_integrity_tests
```

<!-- .element: class="fragment" -->

```rust
fn integrity_test() {
  assert!(
    T::MyConfig::get() > 0,
    "Are all of the generic types I have sensible?"
  );
  // notice that this is for tests, std is available.
  assert!(std::mem::size_of::<T::Balance>() > 4);
}
```

<!-- .element: class="fragment" -->

Notes:

I am in fan of renaming this. If you are too, please comment here

---

## Hooks: `try_state`

- A means for you to ensure correctness of your $STF$, after each transition.
- &shy;<!-- .element: class="fragment" -->Entirely offchain, custom runtime-apis, conditional
  compilation.
  - &shy;<!-- .element: class="fragment" -->Called from `try-runtime-cli`, which you will learn about next week, or anyone else
- &shy;<!-- .element: class="fragment" -->Examples from your assignment?

Notes:

What is a transition? Either a block, or single extrinsic

---

## Hooks: Recap

<diagram class="mermaid">
%%{init: {'theme': 'dark', 'themeVariables': { 'darkMode': true }}}%%
graph LR
    subgraph Offchain
        OffchainWorker
        TryState
    end

    subgraph Genesis
        GenesisBuild
    end

    subgraph AfterTransactions
        direction LR
        OnIdle --> OnFinalize
    end

    subgraph OnChain
        direction LR
        Optional --> BeforeExtrinsics
        BeforeExtrinsics --> Inherents
        Inherents --> Poll
        Poll --> Transactions
        Transactions --> AfterTransactions
    end

    subgraph Optional

OnRuntimeUpgrade
end

    subgraph BeforeExtrinsics
        OnInitialize
    end

    subgraph Transactions
        Transaction1 --> UnsignedTransaction2 --> Transaction3
    end

    subgraph Inherents
        Inherent1 --> Inherent2
    end

</diagram>

- What other hooks can you think of?

Notes:

What other ideas you can think of?

- a hook called once a pallet is first initialized.
  <https://github.com/paritytech/polkadot-sdk/issues/109>
- Local on Post/Pre dispatch: <https://github.com/paritytech/polkadot-sdk/issues/261>
- Global on Post/Pre dispatch is in fact a signed extension. It has to live in the runtime, because you have to specify order.

---

## Additional Resources! 😋

> Check speaker notes (click "s" 😉)

Notes:

## Post lecture Notes

Regarding this drawback to offchain workers that you can only upgrade in cadence with the network.
Offchain worker, like tx-pool api, is only called from an offchain context. Node operators can
easily use the runtime overrides feature to change the behavior of their offchain worker anytime
they want.
