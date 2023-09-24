---
title: Construct Runtime
description: Deep dive into the Construct Runtime macro
duration: 1 hour
---

# `construct_runtime!` and Testing 🔨

---

# Part 1: Runtime Construction

---

<img style="height: 600px" src="../intro/img/frame1.svg" />

---

## Pallet <=> Runtime

A runtime is really ✌️ things:

1. A struct that implements `Config` of all pallets.
2. A type that helps `Executive` implement `RuntimeApis`.

---v

### Pallet <=> Runtime

We build a runtime, using `construct_runtime`, typically twice:

1. Per pallet, there is a mock runtime.
2. A real runtime elsewhere.

Note:

Benchmarking can then use both of these runtimes.

---

## `construct_runtime`: `Runtime` type

```rust [1-100|2]
frame_support::construct_runtime!(
  pub struct Runtime {
    System: frame_system,
    Timestamp: pallet_timestamp,
    Balances: pallet_balances,
    Aura: pallet_aura,
    Dpos: pallet_dpos,
  }
);
```

---v

### `Runtime` type

- It implements [A LOT OF STUFF](https://paritytech.github.io/substrate/master/kitchensink_runtime/struct.Runtime.html)!
- But most importantly, the `Config` trait of all of your pallets 🫵🏻.

```rust
impl frame_system::Config for Runtime { .. }
impl pallet_timestamp::Config for Runtime { .. }
impl pallet_dpos::Config for Runtime { .. }
```

---v

### `<T: Config>` ==> `Runtime`

> Anywhere in your pallet code that you have `<T: Config>` can now be replaced with `Runtime`.

```rust[1-2|3-4|5-6]
// a normal pub function defined in
frame_system::Pallet::<Runtime>::block_number();
// a storage getter of a map.
frame_system::Pallet::<Runtime>::account(42u32);
// A storage type.
frame_system::Account::<Runtime>::get(42u32);
```

---

## `construct_runtime`: Pallet List

```rust [3-7|8|1-100]
frame_support::construct_runtime!(
  pub struct Runtime {
    System: frame_system,
    Timestamp: pallet_timestamp,
    Balances: pallet_balances,
    Aura: pallet_aura,
    Dpos: pallet_dpos,
    <NameYouChoose>: path_to_crate,
  }
);
```

---v

### Pallet List

- Crucially, under the hood, this generates:

```rust
type System = frame_system::Pallet<Runtime>;
type Balances = pallet_balances::Pallet<Runtime>;
..
type DPos = pallet_dpos::Pallet<Runtime>;
```

- Recall that `Runtime` implements `<T: Config>` of all pallets.

---v

### Pallet List

```rust
frame_system::Pallet::<Runtime>::block_number(); // 🤮
System::block_number(); // 🥳

frame_system::Pallet::<Runtime>::account(42u32); // 🤮
System::account(42u32); // 🥳
```

---v

### Pallet List

- Next crucial piece of information that is generated is:

```rust
type AllPallets = (System, Balances, ..., Dpos);
```

<div>

- This is used in `Executive` to dispatch pallet hooks.

```rust
<AllPallets as OnInitialize>::on_initialize();
<AllPallets as OnInitialize>::on_finalize();
```

</div>

<!-- .element: class="fragment" -->

Notes:

Question: What will be the order of `fn on_initialize()`?
There's also `type AllPalletsWithoutSystem` and some other variants that are no longer

---v

### Pallet List + Outer Enums

- Generates some outer types:

  - `RuntimeCall`
  - `RuntimeEvent`
  - `RuntimeOrigin`
  - `RuntimeGenesisConfig`

Notes:

See the lecture on individual item, and the "Outer Enum" lecture.

---v

### Pallet List: `RuntimeCall` Example

```rust
// somewhere in your pallet, called `my_pallet`
#[pallet::call]
impl<T: Config> Pallet<T> {
  fn transfer(origin: OriginFor<T>, from: T::AccountId, to: T::AccountId, amount: u128);
  fn update_runtime(origin: OriginFor<T>, new_code: Vec<u8>);
}
```

```rust
// expanded in your pallet
enum Call {
  transfer { from: T::AccountId, to: T::AccountId, amount: u128 },
  update_runtime { new_code: Vec<u8> },
}
```

<!-- .element: class="fragment" -->

```rust
// in your outer runtime
enum RuntimeCall {
  System(frame_system::Call),
  MyPallet(my_pallet::Call),
}
```

<!-- .element: class="fragment" -->

---v

### Pallet List: Pallet Parts

```rust [1-100|3-5]
frame_support::construct_runtime!(
  pub struct Runtime {
    System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
    Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
    Dpos: pallet_dpos,
  }
);
```

- Omitting them will exclude them from the metadata, or the "outer/runtime types"

<!-- .element: class="fragment" -->

---v

### Pallet List: Pallet Index

```rust [3-5]
frame_support::construct_runtime!(
  pub struct Runtime {
    System: frame_system::{Pallet, Call, Config, Storage, Event<T>} = 1,
    Balances: pallet_balances = 0,
    Dpos: pallet_dpos = 2,
  }
);
```

---

## `construct_runtime`: Final Thoughts

- Order in the `construct_runtime` matters!
- Recall `integrity_test()` is called upon `construct_runtime`.

```sh
test mock::__construct_runtime_integrity_test::runtime_integrity_tests ... ok
```

---v

### Preview

Of the next potential syntax:

```rust
#[frame::construct_runtime]
mod runtime {
  #[frame::runtime]
  pub struct Runtime;

  #[frame::executive]
  pub struct Executive;

  #[frame::pallets]
  #[derive(RuntimeGenesisConfig, RuntimeCall, RuntimeOrigin)]
  pub type AllPallets = (
    System = frame_system = 0,
    BalancesFoo = pallet_balances = 1,
    BalancesBar = pallet_balances = 2,
    Staking = pallet_staking = 42,
  );
}
```

Notes:

See: <https://github.com/paritytech/polkadot-sdk/issues/232>

---

# Part 2: Testing

---

## Testing and Mocks

A test requires a mock runtime, so we need to do a full `construct_runtime` 😱

.. but luckily, most types can be mocked 😮‍💨

<!-- .element: class="fragment" -->

---v

### Testing and Mocks

- `u32` account id.
- `u128` balance.
- `u32` block number.
- ...

---

## Testing: `Get<_>`

- Next, we want to supply some value to those `Get<_>` associated types.

```rust
#[pallet::config]
pub trait Config: frame_system::Config {
  type MaxVoters: Get<u32>;
}
```

---v

### Testing: `Get<_>`

```rust
parameter_types! {
  pub const MyMaxVoters: u32 = 16;
}
```

```rust
impl pallet_template::Config for Runtime {
  type MaxVoters = MyMaxVoters;
}
```

<!-- .element: class="fragment" -->

---v

### Testing: `Get<_>`

- Or, if your value is always constant:

```rust
impl pallet_dpos::Config for Runtime {
  type MaxVoters = frame_support::traits::ConstU32<16>;
}
```

---v

### Testing: `Get<_>`

- Or, if you want to torture yourself:

```rust
pub struct MyMaxVoters;
impl Get<u32> for MyMaxVoters {
  fn get() -> u32 {
    100
  }
}

impl pallet_dpos::Config for Runtime {
  type MaxVoters = MyMaxVoters;
}
```

---

## Testing: Genesis and Builder

- Next, if you want to feed some data into your pallet's genesis state, we must first setup the
  genesis config correctly.

```rust
#[pallet::genesis_config]
#[derive(frame_support::DefaultNoBound)]
pub struct GenesisConfig<T: Config> {
	pub voters: Vec<(T::AccountId, Option<Vote>)>,
}

#[pallet::genesis_build]
impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
  fn build(&self) {
    for (voter, maybe_vote) in &self.voters {
      // do stuff.
    }
  }
}
```

---v

### Testing and Mocks: Genesis and Builder

- Then, we build a builder pattern to construct the genesis config.

```rust
#[derive(Default)]
pub struct Builder {
  pub voters: Vec<(u64, Option<Vote>)>,
}
```

```rust
impl Builder {
  pub fn add_voter(mut self, who: u64) -> Self {
    self.voters.push((who, None));
    self
  }
}
```

<!-- .element: class="fragment" -->

---v

### Testing and Mocks: Genesis and Builder

- Finally:

```rust
impl Builder {
  pub fn build(self) -> TestExternalities {
    let system = frame_system::GenesisConfig::<Runtime>::default();
    let template_module = crate::GenesisConfig { voters: self.voters, ..Default::default() };
    RuntimeGenesisConfig { system, template_module }.build_storage().unwrap().into()
  }

  pub fn build_and_execute(self, f: impl FnOnce()) {
    let mut ext = self.build();
    ext.execute_with(f);
    // any post checks can come here.
  }
}
```

---v

### Testing and Mocks

- Finally, this allows you to write a test like this:

```rust
#[test]
fn test_stuff() {
  let mut ext = Builder::default()
    .add_voter_with_vote(2, Vote::Aye)
    .add_voter(3)
    build_and_execute(|| {
      // do stuff
    });
}
```

---

## Testing: static `parameter_types!`

- What if you want to change that `MyMaxVoters`?

<div>

```rust
parameter_types! {
  pub static MyMaxVoters: u32 = 100;
}
```

```rust
MyMaxVoters::set(200);
MyMaxVoters::get();
```

<!-- .element: class="fragment" -->

</div>

---

## Test ing: Progressing Blocks

- Often times, in your test, you want mimic the progression of an empty block.
- De-nada! We can fake everything in tests 🤠

<!-- .element: class="fragment" -->

---v

### Progressing Blocks

```rust
pub fn next_block() {
  let now = System::block_number();
  Dpos::on_finalize(now);
  System::on_finalize(now);

  System::set_block_number(now + 1);

  System::on_initialize(now + 1)
  Dpos::on_initialize(now + 1);
}
```

---v

### Progressing Blocks

```rust
pub fn next_block() {
  let now = System::block_number();
  AllPallets::on_finalize(now);

  System::set_block_number(now + 1);

  AllPallets::on_initialize(now + 1)
}
```

---v

### Progressing Blocks

````rust
```rust
#[test]
fn test() {
  let mut ext = Builder::default()
    .add_validator(1)
    .set_minimum_delegation(200)
    .build();
  ext.execute_with(|| {
    // initial stuff
    next_block();

    // dispatch some call
    assert!(some_condition);

    next_block();

    // repeat..
  });
}
````

```
---

## Additional Resources 😋

> Check speaker notes (click "s" 😉)

Notes:

- This PR was actually an outcome Cambridge PBA: <https://github.com/paritytech/substrate/pull/11932>
- <https://github.com/paritytech/substrate/pull/11818>
- <https://github.com/paritytech/substrate/pull/10043>
- On usage of macros un Substrate: <https://github.com/paritytech/substrate/issues/12331>
- Discussion on advance testing: <https://forum.polkadot.network/t/testing-complex-frame-pallets-discussion-tools/356>
- Reserve topic: Reading events.
- Reserve-topic: try-state.

### Original Lecture Script

this is your bridge from a pallet into a runtime.

a runtime amalgamator is composed of the following:

1. all pallet's `Config` implemented by a `struct Runtime`;
1. construct `Executive` and use it to implement all the runtime APIs
1. Optionally, some boilerplate to setup benchmarking.
1. invoke `construct_runtime!`.
1. Alias for each pallet.

The `construct_runtime!` itself does a few things under the hood:

1. crate `struct Runtime`.
1. amalgamate `enum RuntimeCall`; // passed inwards to some pallets that want to store calls.
1. amalgamate `enum RuntimeEvent`; // passed inwards to all pallets.
1. amalgamate `enum RuntimeOrigin` (this is a fixed struct, not an amalgamation);
1. Create a very important type alias:

- `type AllPallets` / `type AllPalletsWithoutSystem`

1. run `integrity_test()`.

> Note that there is no such thing as `RuntimeError`. Errors are not amalgamated, they just are. This should be in the error lecture.

- Ordering in `construct_runtime` matters.
- Pallet parts can be optional in `construct_runtime!`.
```
