---
title: Migrations and Try Runtime
description: Runtime upgrades and how to survive them
instructors: ["Kian Paimani"]
teaching-assistants: [".."]
---

# Migrations and Try Runtime

---

## Runtime upgrades...

### _and how to survive them_

---

### _At the end of this lecture, you will be able to:_

- Justify when runtime migrations are needed.
- Write a the full a runtime upgrade that includes migrations, end-to-end.
- Test runtime upgrades before executing on a network using `try-runtime` and `remote-externalities`.

---

## When is a Migration Required?

---v

### When is a Migration Required?

- In a typical runtime upgrade, you typically only replace `:code:`. This is _**Runtime Upgrade**_.
- If you change the _storage layout_, then this is also a _**Runtime Migration**_.

> Anything that changes **encoding** is a migration!

---v

### When is a Migration Required?

```rust
#[pallet::storage]
pub type FooValue = StorageValue<_, Foo>;
```

```rust
// old
pub struct Foo(u32)
// new
pub struct Foo(u64)
```

- A clear migration.

<!-- .element: class="fragment" -->

---v

### When is a Migration Required?

```rust
#[pallet::storage]
pub type FooValue = StorageValue<_, Foo>;
```

```rust
// old
pub struct Foo(u32)
// new
pub struct Foo(i32)
// or
pub struct Foo(u16, u16)
```

- The data still _fits_, but the _interpretations_ is almost certainly different!

<!-- .element: class="fragment" -->

---v

### When is a Migration Required?

```rust
#[pallet::storage]
pub type FooValue = StorageValue<_, Foo>;
```

```rust
// old
pub struct Foo { a: u32, b: u32 }
// new
pub struct Foo { a: u32, b: u32, c: u32 }
```

- This is still a migration, because `Foo`'s decoding changed.

<!-- .element: class="fragment" -->

---v

### When is a Migration Required?

```rust
#[pallet::storage]
pub type FooValue = StorageValue<_, Foo>;
```

```rust
// old
pub struct Foo { a: u32, b: u32 }
// new
pub struct Foo { a: u32, b: u32, c: PhantomData<_> }
```

- If for whatever reason `c` has a type that its encoding is like `()`, then this would work.

<!-- .element: class="fragment" -->

---v

### When is a Migration Required?

```rust
#[pallet::storage]
pub type FooValue = StorageValue<_, Foo>;
```

```rust
  // old
  pub enum Foo { A(u32), B(u32) }
  // new
  pub enum Foo { A(u32), B(u32), C(u128) }
```

- Extending an enum is even more interesting, because if you add the variant to the end, no migration is needed.

<!-- .element: class="fragment" -->

- Assuming that no value is initialized with `C`, this is _not_ a migration.

<!-- .element: class="fragment" -->

---v

### When is a Migration Required?

```rust
#[pallet::storage]
pub type FooValue = StorageValue<_, Foo>;
```

```rust
// old
pub enum Foo { A(u32), B(u32) }
// new
pub enum Foo { A(u32), C(u128), B(u32) }
```

- You probably _never_ want to do this, but it is a migration.

<!-- .element: class="fragment" -->

---v

### 🦀 Rust Recall 🦀

Enums are encoded as the variant enum, followed by the inner data:

- The order matters! Both in `struct` and `enum`.
- Enums that implement `Encode` cannot have more than 255 variants.

---v

### When is a Migration Required?

```rust
#[pallet::storage]
pub type FooValue = StorageValue<_, u32>;
```

```rust
// new
#[pallet::storage]
pub type BarValue = StorageValue<_, u32>;
```

- So far everything is changing the _value_ format.<br/>

<div>

- The _key_ changing is also a migration!

</div>

<!-- .element: class="fragment" -->

---v

### When is a Migration Required?

```rust
#[pallet::storage]
pub type FooValue = StorageValue<_, u32>;
```

```rust
// new
#[pallet::storage_prefix = "FooValue"]
#[pallet::storage]
pub type I_can_NOW_BE_renamEd_hahAA = StorageValue<_, u32>;
```

- Handy macro if you must rename a storage type.<br/>
- This does _not_ require a migration.

<!-- .element: class="fragment" -->

---

## Writing Runtime Migrations

- Now that we know how to detect if a storage change is a **migration**, let's see how we write one.

---v

### Writing Runtime Migrations

- Once you upgrade a runtime, the code is expecting the data to be in a new format.
- Any `on_initialize` or transaction might fail decoding data, and potentially `panic!`

---v

### Writing Runtime Migrations

- We need a **_hook_** that is executed **ONCE** as a part of the new runtime...
- But before **ANY** other code (on_initialize, any transaction) with the new runtime is migrated.

> This is `OnRuntimeUpgrade`.

<!-- .element: class="fragment" -->

---v

### Writing Runtime Migrations

- Optional activity: Go into `executive` and `system`, and find out how `OnRuntimeUpgrade` is called only when the code changes!

---

## Pallet Internal Migrations

---v

### Pallet Internal Migrations

One way to write a migration is to write it inside the pallet.

```rust
#[pallet::hooks]
impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
  fn on_runtime_upgrade() -> Weight {
    migrate_stuff_and_things_here_and_there<T>();
  }
}
```

> This approach is likely to be deprecated and is no longer practiced within Parity either.

<!-- .element: class="fragment" -->

---v

### Pallet Internal Migrations

```rust [4-8]
#[pallet::hooks]
impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
  fn on_runtime_upgrade() -> Weight {
    if guard_that_stuff_has_not_been_migrated() {
      migrate_stuff_and_things_here_and_there<T>();
    } else {
      // nada
    }
  }
}
```

- If you execute `migrate_stuff_and_things_here_and_there` twice as well, then you are doomed 😫.

---v

### Pallet Internal Migrations

**Historically**, something like this was used:

```rust [1-7|9-19]
#[derive(Encode, Decode, ...)]
enum StorageVersion {
  V1, V2, V3, // add a new variant with each version
}

#[pallet::storage]
pub type Version = StorageValue<_, StorageVersion>;

#[pallet::hooks]
impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
  fn on_runtime_upgrade() -> Weight {
    if let StorageVersion::V2 = Version::<T>::get() {
      // do migration
      Version::<T>::put(StorageVersion::V3);
    } else {
      // nada
    }
  }
}
```

---v

### Pallet Internal Migrations

- FRAME introduced macros to manage migrations: `#[pallet::storage_version]`.

```rust
// your current storage version.
const STORAGE_VERSION: StorageVersion = StorageVersion::new(2);

#[pallet::pallet]
#[pallet::storage_version(STORAGE_VERSION)]
pub struct Pallet<T>(_);
```

- This adds two function to the `Pallet<_>` struct:

```rust
// read the current version, encoded in the code.
let current = Pallet::<T>::current_storage_version();
// read the version encoded onchain.
Pallet::<T>::on_chain_storage_version();
// synchronize the two.
current.put::<Pallet<T>>();
```

---v

### Pallet Internal Migrations

```rust
#[pallet::hooks]
impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
  fn on_runtime_upgrade() -> Weight {
    let current = Pallet::<T>::current_storage_version();
    let onchain = Pallet::<T>::on_chain_storage_version();

    if current == 1 && onchain == 0 {
      // do stuff
      current.put::<Pallet<T>>();
    } else {
    }
  }
}

```

Stores the version as u16 in [`twox(pallet_name) ++ twox(:__STORAGE_VERSION__:)`](https://github.com/paritytech/substrate/blob/6d84315348d1fca9ca59454b9f37411c80e05ab4/frame/support/src/traits/metadata.rs#L155).

---

## External Migrations

---v

### External Migrations

- Managing migrations within a pallet could be hard.
- Especially for those that want to use external pallets.

Alternative:

- Every runtime can explicitly pass anything that implements `OnRuntimeUpgrade` to `Executive`.
- End of the day, Executive does:
  - `<(COnRuntimeUpgrade, AllPalletsWithSystem) as OnRuntimeUpgrade>::on_runtime_upgrade()`.

<!-- .element: class="fragment" -->

---v

### External Migrations

- The main point of external migrations is making it more clear:
- "_What migrations did exactly execute on upgrade to spec_version xxx_"

---v

### External Migrations

- Expose your migration as a standalone function or struct implementing `OnRuntimeUpgrade` inside a `pub mod v<version_number>`.

```rust
pub mod v3 {
  pub struct Migration;
  impl OnRuntimeUpgrade for Migration {
    fn on_runtime_upgrade() -> Weight {
      // do stuff
    }
  }
}
```

---v

### External Migrations

- Guard the code of the migration with `pallet::storage_version`
- Don't forget to write the new version!

```rust
pub mod v3 {
  pub struct Migration;
  impl OnRuntimeUpgrade for Migration {
    fn on_runtime_upgrade() -> Weight {
      let current = Pallet::<T>::current_storage_version();
      let onchain = Pallet::<T>::on_chain_storage_version();

      if current == 1 && onchain == 0 {
        // do stuff
        current.put::<Pallet<T>>();
      } else {
      }
    }
  }
}
```

---v

### External Migrations

- Pass it to the runtime per-release.

```rust
pub type Executive = Executive<
  _,
  _,
  _,
  _,
  _,
  (v3::Migration, ...)
>;
```

---v

### External Migrations

- Discussion: Can the runtime upgrade scripts live forever? Or should they be removed after a few releases?

Notes:

Short answer is, yes, but it is a LOT of work. See here: https://github.com/paritytech/substrate/issues/10308

---

### Utilities in `frame-support`.

- `translate` methods:
  - For `StorageValue`, `StorageMap`, etc.
- https://paritytech.github.io/substrate/master/frame_support/storage/migration/index.html

* `#[storage_alias]` macro to create storage types for removed for those that are being removed.

Notes:

Imagine you want to remove a storage map and in a migration you want to iterate it and delete all items. You want to remove this storage item, but it would be handy to be able to access it one last time in the migration code. This is where `#[storage_alias]` comes into play.

---

## Case Studies

1. The day we destroyed all balances in Polkadot.
1. First ever migration ([`pallet-elections-phragmen`](https://github.com/paritytech/substrate/pull/3948)).
1. Fairly independent migrations in `pallet-elections-phragmen`.

---

## Testing Upgrades

---v

### Testing Upgrades

- `try-runtime` + `RemoteExternalities` allow you to examine and test a runtime in detail with a high degree of control over the environment.

- It is meant to try things out, and inspired by traits like `TryFrom`, the name `TryRuntime` was chosen.

---v

### Testing Upgrades

Recall:

- The runtime communicates with the client via host functions.
- Moreover, the client communicates with the runtime via runtime APIs.
- An environment that provides these host functions is called `Externalities`.
- One example of which is `TestExternalities`, which you have already seen.

---v

### Testing Upgrades: `remote-externalities`

`remote-externalities` ia a builder pattern that loads the state of a live chain inside `TestExternalities`.

```rust
let mut ext = Builder::<Block>::new()
  .mode(Mode::Online(OnlineConfig {
  	transport: "wss://rpc.polkadot.io",
  	pallets: vec!["PalletA", "PalletB", "PalletC", "RandomPrefix"],
  	..Default::default()
  }))
  .build()
  .await
  .unwrap();
```

Reading all this data over RPC can be slow!

---v

### Testing Upgrades: `remote-externalities`

`remote-externalities` supports:

- Custom prefixes -> Read a specific pallet
- Injecting custom keys -> Read `:code:` as well.
- Injecting custom key-values -> Overwrite `:code:` with `0x00`!
- Reading child-tree data -> Relevant for crowdloan pallet etc.
- Caching everything in disk for repeated use.

---v

### Testing Upgrades: `remote-externalities`

`remote-externalities` is in itself a very useful tool to:

- Go back in time and re-running some code.
- Write unit tests that work on the real-chain's state.

---

## Testing Upgrades: `try-runtime`

- `try-runtime` is a CLI and a set of custom runtime APIs integrated in substrate that allows you to do detailed testing..

- .. including running `OnRuntimeUpgrade` code of a new runtime, on top of a real chain's data.

---v

### Testing Upgrades: `try-runtime`

- A lot can be said about it, the best resource is the [rust-docs](https://paritytech.github.io/substrate/master/try_runtime_cli/index.html).

---v

### Testing Upgrades: `try-runtime`

- You might find some code in your runtime that is featured gated with `#[cfg(feature = "try-runtime")]`. These are always for testing.
- `pre_upgrade` and `post_upgrade`: Hooks executed before and after `on_runtime_upgrade`.
- `try_state`: called in various other places, used to check the invariants the pallet.

---v

### Testing Upgrades: `try-runtime`: Live Demo.

- Let's craft a migration on top of poor node-template 😈..
- and migrate the balance type from u128 to u64.

---

## Additional Resources 😋

> Check speaker notes (click "s" 😉)

Notes:

- additional work on automatic version upgrades: https://github.com/paritytech/substrate/issues/13107
- a Great talk about try-runtime and further testing of your runtime: https://www.youtube.com/watch?v=a_u3KMG-n-I

#### Reference material:

https://docs.google.com/presentation/d/1hr3fiqOI0JlXw0ISs8uV9BXiDQ5mGOQLc3b_yWK6cxU/edit#slide=id.g43d9ae013f_0_82
https://www.crowdcast.io/e/substrate-seminar/41

#### Exercise ideas:

- Find the storage version of nomination pools pallet in Kusama.
- Give them a poorly written migration code, and try and fix it. Things they need to fix:
  - The migration depends on `<T: Config>`
  - Does not manage version properly
  - is hardcoded in the pallet.
- Re-execute the block at which the runtime went OOM in May 25th 2021 Polkadot.
