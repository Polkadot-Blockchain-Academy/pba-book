---
title: Substrate/FRAME Tips and Tricks
description: Substrate and FRAME Tips and Tricks for Web3 Engineers
---

# Substrate / FRAME Tips and Tricks

Notes:

- A random collection of things that you should probably know about.
- These are relevant for coding in FRAME and Substrate.

---

# Part 0 FRAMELess Assignment Retro

---v

## Formatter

`rustfmt` It is a good thing, please use it!

---v

## Options --> Results

```rust
let signer_balance = Runtime::get_state::<AccountBalance>(&Self::get_storage_key(&signer));
match signer_balance {
	Some(_) => (),
	None => {return Err(TransactionValidityError::Invalid(InvalidTransaction::BadSigner))},
};
```

```rust
let signer_balance = Runtime::get_state::<AccountBalance>(&Self::get_storage_key(&signer))
	.ok_or(TransactionValidityError::Invalid(InvalidTransaction::BadSigner))?;
```

<!-- .element: class="fragment" -->

Notes:

https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or
https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or_else

---v

## `expect` for Impossible Panics

```rust
if ext.function.tip.is_some() {
	let tip_amount = ext.function.tip.unwrap();
}
```

```rust
if ext.function.tip.is_some() {
  // ..
  // ..
  // ..
  // ..
	let tip_amount = ext.function.tip.expect("checked to be some; qed");
}
```

<!-- .element: class="fragment" -->

---

# Part 1 Substrate Stuff

---

## `<Type as Trait>::AssociatedType`

- The single most useful Rust syntactic detail that you _MUST_ know.

Notes:

what is a type? A struct is a type. An unum is a type. all primitives are type. A lot of things are
types.

---v

### `<Type as Trait>::AssociatedType`

Example:

```rust [1-4|1-8|1-12|1-100]
trait Config {
  type Extrinsic
  type Header: HeaderT
}

pub type ExtrinsicFor<C> = <C as Config>::Extrinsic;
fn process_extrinsic<C>(e: ExtrinsicFor<C>) { .. }

trait HeaderT {
  type Number;
}

pub type NumberFor<C> = <<C as Config>::Header as HeaderT>::Number;
```

Notes:

turbo fish
fully qualified syntax.

---

## The `std` Paradigm

`std` <-> `core` <-> `no_std`

Notes:

- Recap:
  - `std` is the interface to the common OS-abstractions.
  - `core` is a subset of `std` that makes no assumption about the operating system.
- a `no_std` crate is one that relies on `core` rather than `std`.

---v

### Cargo Features

- Way to compile different code via flags.
- Crates define some features in their `Cargo.toml`
- Crates can conditionally enable features of their dependencies as well.

```toml
[dependencies]
other-stuff = { version = "1.0.0" }

[features]
default = [""]
additional-features = ["other-stuff/on-steroids"]
```

Notes:

imagine that you have a crate that has some additional features that are not always needed. You put
that behind a feature flag called `additional-features`.

---v

### Cargo Features: Substrate Wasm Crates

```toml
[dependencies]
dep1 = { version = "1.0.0", default-features = false }
dep2 = { version = "1.0.0", default-features = false }

[features]
default = ["std"]
std = [
  "dep1/std"
  "dep2/std"
]
```

Notes:

every crate will have a feature "std". This is a flag that you are compiling with the standard
library. This is the default.

Then, bringing a dependency with `default-features = false` means by default, don't enable this
dependencies "std".

Then, in `std = ["dep/std"]` you are saying "if my std is enabled, enable my dependencies std as
well".

- The name "`std`" is just an idiom in the rust ecosystem.
- `no_std` does NOT mean Wasm!
- `std` does not mean native!

But in substrate, it kinda means like that:

std => native
no_std => wasm

---v

### The `std` Paradigm

- All crates in substrate that eventually compile to Wasm:

```rust
#![cfg_attr(not(feature = "std"), no_std)]
```

---v

### The `std` Paradigm: Adding dependencies

```sh
error: duplicate lang item in crate sp_io (which frame_support depends on): panic_impl.
  |
  = Notes:


 the lang item is first defined in crate std (which serde depends on)

error: duplicate lang item in crate sp_io (which frame_support depends on): oom.
  |
  = Notes:


 the lang item is first defined in crate std (which serde depends on)
```

---v

### The `std` Paradigm

A subset of the standard types in rust that also exist in rust `core` are re-exported from
[`sp_std`](https://paritytech.github.io/polkadot-sdk/master/sp_std/index.html).

```rust
sp_std::prelude::*;
```

Notes:

Hashmap not exported due to non-deterministic concerns.
floats are usable, but also non-deterministic! (and I think they lack `encode`, `decode` impl)
interesting to look at `if_std` macro in `sp_std`.

---

## Logging And Prints In The Runtime.

- First, why bother? let's just add as many logs as we want into the runtime.

<!-- .element: class="fragment" -->

- Size of the wasm blob matters..

<!-- .element: class="fragment" -->

- Any logging increases the size of the Wasm blob. **String literals** are stored somewhere in your
  program!

<!-- .element: class="fragment" -->

---v

### Logging And Prints In The Runtime.

```rust
#[derive(sp_std::fmt::Debug)]
struct LONG_AND_BEAUTIFUL_NAME {
  plenty: u32,
  of: u32,
  fields: u32,
  with: u32,
  different: u32
  names: u32
}
```

will add a lot of string literals to your wasm blob.

Notes:

- `wasm2wat polkadot_runtime.wasm > dump | rg stripped`
- Should get you the `.rodata` (read-only data) line of the wasm blob, which contains all the logging
  noise.
- This contains string literals form errors, logs, metadata, etc.

---v

### Logging And Prints In The Runtime.

`sp_std::fmt::Debug` vs `sp_debug_derive::RuntimeDebug`

Notes:

https://paritytech.github.io/substrate/master/sp_debug_derive/index.html

---v

### Logging And Prints In The Runtime.

```rust
#[derive(RuntimeDebug)]
pub struct WithDebug {
    foo: u32,
}

impl ::core::fmt::Debug for WithDebug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        #[cfg(feature = "std)]
        {
          fmt.debug_struct("WithRuntimeDebug")
            .field("foo", &self.foo)
            .finish()
        }
        #[cfg(not(feature = "std))]
        {
          fmt.write("<wasm:stripped>")
        }
    }
}
```

---v

### Logging And Prints In The Runtime.

- Once types implement `Debug` or `RuntimeDebug`, they can be printed. Various ways:
- If you only want something in tests, native builds etc

```rust
sp_std::if_std! {
  println!("hello world!");
  dbg!(foo);
}
```

---v

### Logging And Prints In The Runtime.

- Or you can use the common `log` crate

```rust
log::info!(target: "foo", "hello world!");
log::debug!(target: "bar", "hello world! ({})", 10u32);
```

---v

### Logging And Prints In The Runtime.

- But `log` crate doesn't doo much in itself! it needs two additional steps to work:

1. `// $ RUST_LOG=foo=debug,bar=trace cargo run`
2. `sp_tracing::try_init_simple()`

Notes:

https://paritytech.github.io/polkadot-sdk/master/sp_tracing/index.html

Log statements are only evaluated if the corresponding level and target is met.

```rust
/// only executed if `RUST_LOG=KIAN=trace`
frame_support::log::trace!(target: "KIAN", "({:?})", (0..100000).into_iter().collect());
```

`log` in rust does not do anything -- it only tracks what needs to be logged. Then you need a logger
to actually export them. In rust this is often `env_logger` or `sp_tracing` in substrate tests.

In the runtime, the log messages are sent via the host functions to the client to be printed.

If the interface is built with `disable-logging`, it omits all log messages.

---

## Arithmetic Helpers, and the `f32`, `f64` Story.

- Floating point numbers have different standards, and (**_slightly_**) different implementations on
  different architectures and vendors.

- If my balance is `10.000000000000001` DOT on one validator and `10.000000000000000` DOT on another validator, game over for your consensus 😮‍💨.

---v

### PerThing.

```python
> .2 + .2 + .2 == .6
> false
```

```
> a = 10
> b = 0.1
> c = 0.2
> a*(b+c) == a*b + a*c
> false
```

- Google "weird float behavior" for more entertainment around this.

---v

### PerThing.

- We store ratios and such in the runtime with "Fixed-Point" arithmetic types.

```rust
struct Percent(u8);

impl Percent {
  fn new(x: u8) {
    Self(x.min(100));
  }
}

impl Mul<u32> for Percent {
  ...
}

```

---v

### PerThing.

```rust
use sp_arithmetic::Perbill;

let p = Perbill::from_part_parts(1_000_000_000u32 / 4);
let p = Perbill::from_percent(25);
let p = Perbill::from_rational(1, 4);

> p * 100u32;
> 25u32;
```

- Some precision concerns exist, but that's a story for another day.

---v

### Fixed Point Numbers

`Per-thing` is great for representing `[0, 1]` range.

What if we need more?

```
100 ~ 1
200 ~ 2
300 ~ 3
350 ~ 3.5
```

---v

### Fixed Point Numbers

```rust
use sp_arithmetic::FixedU64;

let x = FixedU64::from_rational(5, 2);
let y = 10u32;
let z = x * y;
> 25
```

---v

### Arithmetic Types

- See [`sp-arithmetic`](https://paritytech.github.io/polkadot-sdk/master/sp_arithmetic/index.html) to
  learn more.

Notes:

- [`U256`](https://paritytech.github.io/polkadot-sdk/master/sp_core/struct.U256.html), `U512`: battle-tested since the ethereum days.
- [substrate-fixed](https://github.com/encointer/substrate-fixed): community project. Supercharged `PerThing` and `Fixed`.
- [`big_uint.rs`](https://paritytech.github.io/polkadot-sdk/master/sp_arithmetic/biguint/index.html) (unaudited)

```rust

pub struct BigUint {
	/// digits (limbs) of this number (sorted as msb -> lsb).
	pub(crate) digits: Vec<Single>,
}
```

---

### Fallibility: Math Operations

Things like **addition**, **multiplication**, **division** could all easily fail.

<div>

- Panic
  - `u32::MAX * 2 / 2` (in debug builds)
  - `100 / 0`
  s
  </div>

<!-- .element: class="fragment" -->

<div>

- Overflow
  - `u32::MAX * 2 / 2` (in release builds)

</div>

<!-- .element: class="fragment" -->

---v

### Fallibility: Math Operations

- `Checked` -- prevention ✋🏻

  ```
  if let Some(outcome) = a.checked_mul(b) { ... } else { ... }
  ```

- `Saturating` -- silent recovery 🤫

  ```
  let certain_output = a.saturating_mul(b);
  ```

Notes:

Why would you ever want to saturate? only in cases where you know if the number is overflowing,
other aspects of the system is so fundamentally screwed that there is no point in doing any kind of
recovery.

There's also `wrapping_op` and `carrying_op` etc on all rust primitives, but not quite
relevant.

https://doc.rust-lang.org/std/primitive.u32.html#method.checked_add
https://doc.rust-lang.org/std/primitive.u32.html#method.saturating_add

Also substrate's version of the same:
https://paritytech.github.io/polkadot-sdk/master/sp_arithmetic/traits/trait.SaturatedConversion.html

---v

### Fallibility: Conversion

```rust
fn main() {
    let a = 1000u32 as u8;
    println!("{}", a); //
}
```

Notes:

conversion of primitive number types is also a common point of error. Avoid `as`.

---v

### Fallibility: Conversion

- Luckily, rust is already pretty strict for the primitive types.
- `TryInto` / `TryFrom` / `From` / `Into`

```rust
impl From<u16> for u32 {
  fn from(x: u16) -> u32 {
    x as u32 // ✅
  }
}
```

<!-- .element: class="fragment" -->

```rust
impl TryFrom<u32> for u16 {
  fn try_from(x: u32) -> Result<u16, _> {
    if x >= u16::MAX { Err(_) } else { Ok(x as u16) }
  }
}
```

<!-- .element: class="fragment" -->

Notes:

typically you don't implement `Into` and `TryInto`, because of blanket impls. See:
https://doc.rust-lang.org/std/convert/trait.From.html

for any T and U, `impl From<T> for U` implies `impl Into<U> for T`

---v

### Fallibility: Conversion

- `struct Foo<T: From<u32>>`

T is u32 or larger.

<!-- .element: class="fragment" -->

- `struct Foo<T: Into<u32>>`

`T` is u32 or smaller.

<!-- .element: class="fragment" -->

- `struct Foo<T: TryInto<u32>>`

`T` can be any of numeric types.

<!-- .element: class="fragment" -->

---

# Part 2: FRAME Stuff

---

## `trait Get`

A very basic, yet very substrate-idiomatic way to pass _values_ through _types_.

```rust
pub trait Get<T> {
  fn get() -> T;
}
```

```rust
// very basic blanket implementation, which you should be very versed in reading.
impl<T: Default> Get<T> for () {
  fn get() -> T {
    T::default()
  }
}
```

<!-- .element: class="fragment" -->

```rust
struct Foo<G: Get<u32>>;
let foo = Foo<()>;
```

<!-- .element: class="fragment" -->

Notes:

implementing defaults for `()` is a very FRAME-idiomatic way of doing things.

---v

### `trait Get`

```rust
parameter_types! {
  pub const Foo: u32 = 10;
}
```

```rust
// expands to:
pub struct Foo;
impl Get<u32> for Foo {
  fn get() -> u32 {
    10;
  }
}
```

<!-- .element: class="fragment" -->

Notes:

You have implemented this as a part of your rust exam.

---

## `bounded`

- `BoundedVec`, `BoundedSlice`, `BoundedBTreeMap`

```rust
#[derive(Encode, Decode)]
pub struct BoundedVec<T, S: Get<u32>>(
  pub(super) Vec<T>,
  PhantomData<S>,
);
```

Notes:

`PhantomData`?

---v

### `bounded`

- Why not do a bounded type like this? 🤔

```rust
#[cfg_attr(feature = "std", derive(Serialize))]
#[derive(Encode)]
pub struct BoundedVec<T>(
  pub(super) Vec<T>,
  u32,
);
```

Notes:

_`Get` trait is a way to convey values through types. The type system is mostly for compiler, and
has minimal overhead at runtime._

---

## `trait Convert`

```rust
pub trait Convert<A, B> {
	fn convert(a: A) -> B;
}
```

```rust
pub struct Identity;
// blanket implementation!
impl<T> Convert<T, T> for Identity {
	fn convert(a: T) -> T {
		a
	}
}
```

<!-- .element: class="fragment" -->

Notes:

this one's much simpler, but good excuse to teach them blanket implementations.

---v

### Example of `Get` and `Convert`

```rust
/// Some configuration for my module.
trait Config {
  /// Something that gives you a `u32`.
  type MaximumSize: Get<u32>;
  /// Something that is capable of converting `u64` to `u32`,
  /// which is pretty damn hard.
  type Convertor: Convertor<u64, u32>;
}
```

```rust
struct Runtime;
impl Config for Runtime {
  type MaximumSize = ();
  type Convertor = SomeType
}
```

<!-- .element: class="fragment" -->

```rust
<Runtime as Config>::Convertor::convert(_, _);
```

<!-- .element: class="fragment" -->

```rust
fn generic_fn<T: Config>() { <T as Config>::Convertor::convert(_, _)}
```

<!-- .element: class="fragment" -->

Notes:

often times, in examples above, you have to use this syntax: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name

---

## Implementing Traits For Tuples

```rust
struct Module1;
struct Module2;
struct Module3;

trait OnInitialize {
  fn on_initialize();
}

impl OnInitialize for Module1 { fn on_initialize() {} }
impl OnInitialize for Module2 { fn on_initialize() {} }
impl OnInitialize for Module3 { fn on_initialize() {} }
```

How can I easily invoke `OnInitialize` on all 3 of `Module1, Module2, Module3`?

Notes:

Alternative, but this needs allocation:

```
struct Module1;
struct Module2;
struct Module3;

trait OnInitializeDyn {
  fn on_initialize(&self);
}

impl OnInitializeDyn for Module1 { fn on_initialize(&self) {} }
impl OnInitializeDyn for Module2 { fn on_initialize(&self) {} }
impl OnInitializeDyn for Module3 { fn on_initialize(&self) {} }

fn main() {
    let x: Vec<Box<dyn OnInitializeDyn>> = vec![Box::new(Module1), Box::new(Module2)];
    x.iter().for_each(|i| i.on_initialize());
}
```

---v

### Implementing Traits For Tuples

```rust
Impl<A, B, C> OnInitialize for (A, B, C)
where
  A: OnInitialize,
  B: OnInitialize,
  C: OnInitialize,
{
  fn on_initialize() {
    A::on_initialize();
    B::on_initialize();
    C::on_initialize();
  }
}
```

```
// fully-qualified syntax - turbo-fish.
<(Module1, Module2, Module3) as OnInitialize>::on_initialize();
```

<!-- .element: class="fragment" -->

Notes:

**Tuples** are the natural way to group **types** together (analogous to have a **vector** is the natural way to group **values** together..)

---v

### Implementing Traits For Tuples

Modern syntax of making a trait "_tuple-call-able_"

```rust
// In the most basic form:
#[impl_for_tuples(30)]
pub trait OnTimestampSet<Moment> {
	fn on_timestamp_set(moment: Moment);
}
```

Notes:

https://docs.rs/impl-trait-for-tuples/latest/impl_trait_for_tuples/

Only problem: A lot of boilerplate. Macros!

Historically, we made this work with `macro_rules!`

```rust
macro_rules! impl_for_tuples {
    ( $( $elem:ident ),+ ) => {
        impl<$( $elem: OnInitialize, )*> OnInitialize for ($( $elem, )*) {
            fn on_initialize() {
                $( $elem::on_initialize(); )*
            }
        }
    }
}

impl_for_tuples!(A, B, C, D);
impl_for_tuples!(A, B, C, D, E);
impl_for_tuples!(A, B, C, D, E, F);
```

---

### Debugging Macros Generated Code

<img rounded src="./img/frame-macro-error.gif" />

Notes:

- Crucial to start debugging from the first macro expansion, not the end!
- This is only when you are working with a framework like FRAME that is a lot of macro-generated code. If you are writing macros, there are different tricks that you can learn about online.

---

## Defensive Programming

> ..is a form of defensive design to ensure the continuing function of a piece of software under
> unforeseen circumstances... where **high availability**, **safety**, or **security** is needed.

---v

### Defensive Programming

```
let maybe_value: Option<u32> = Some(42);
// ...
let value = maybe_value.unwrap(); // aggressive programming 🤠
let value = maybe_value.expect("written evidence") // better ✅
let value = maybe_value.ok_or(Error::DefensiveError)?; // best ✅
let value = maybe_value.ok_or(Error::IKnowThisWillNeverHappenButIAmOnlyAnApeAndCanMakeMistakes)?; // best ✅
```

---v

### Defensive Programming

```rust
let value = maybe_value.ok_or(Error::IKnowThisWillNeverHappenButIAmOnlyAnApeAndCanMakeMistakes)
  .map_err(|e| {
    #[cfg(test)]
    panic!("An impossible error happened!: {:?}", e);
    log::error!(target: "..", "defensive error happened: {:?}", e);
    e
  })?;
```

---v

### Defensive Programming

[Defensive traits](https://paritytech.github.io/substrate/master/frame_support/traits/trait.Defensive.html)

```rust
use frame_support::DefensiveOption;
let value = maybe_value.defensive_ok_or(Error::IKnowThisWillNeverHappenButIAmOnlyAnApeAndCanMakeMistakes)
```

---

## Final Words on Panics

1. Panics can happen other than `Err(_).unwrap()`

Notes:

- slice/vector indexing can panic if out of bound
- `.insert`, `.remove`
- division by zero.

---v

### Final Words on Panics

2. Panics in Internal API --> Well Documented Code.

---v

### Final Words on Panics

````rust
/// Multiplies the given input by two.
///
/// Some further information about what this does, and where it could be used.
///
/// ```
/// fn main() {
///   let x = multiply_by_2(10);
///   assert_eq!(10, 20);
/// }
/// ```
///
/// ## Panics
///
/// Panics under such and such condition.
fn multiply_by_2(x: u32) -> u32 { .. }
````

Notes:

- Speaking of documentation, [here's a very good guideline](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html)!

---v

### Final Words on Panics

- Try and not be this guy:

```rust
/// This function works with module x and multiples the given input by two. If
/// we optimize the other variant of it, we would be able to achieve more
/// efficiency but I have to think about it. Probably can panic if the input
/// overflows u32 lolololooooo
fn multiply_by_2(x: u32) -> u32 { .. }
```

---

## Additional Resources! 😋

<img width="300px" rounded src="../../../assets/img/5-Substrate/thats_all_folks.png" />

> Check speaker notes (click "s" 😉)

> Good luck with FRAME!

Notes:

- Rust didn't have u128 until not too long ago! https://github.com/paritytech/substrate/pull/163/files
- `TryFrom`/`TryInto` are also not too old! https://github.com/paritytech/substrate/pull/163/files#r188938077
- Remove `As`, which tried to fill the lack of `TryFrom/TryInto` https://github.com/paritytech/substrate/pull/2602
- Runtime Logging PR: https://github.com/paritytech/substrate/pull/3821

- Impl trait for tuples:

  - https://stackoverflow.com/questions/64332037/how-can-i-store-a-type-in-an-array
  - https://doc.rust-lang.org/book/ch17-02-trait-objects.html#trait-objects-perform-dynamic-dispatch
  - https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name
  - https://turbo.fish/
  - https://techblog.tonsser.com/posts/what-is-rusts-turbofish
  - https://docs.rs/impl-trait-for-tuples/latest/impl_trait_for_tuples/

- std/no_std
  - https://paritytech.github.io/substrate/master/sp_std/index.html
  - https://doc.rust-lang.org/core/index.html
  - https://doc.rust-lang.org/std/index.html
  - https://rust-lang.github.io/api-guidelines/naming.html#feature-names-are-free-of-placeholder-words-c-feature

### Feedback After Lecture:

- Update on defensive ops: https://github.com/paritytech/substrate/pull/12967
- Next time, talk about making a storage struct be `<T: Config>`.
- SignedExtension should technically be part of the substrate module. Integrate it in the
  assignment, perhaps.
- A section about `XXXNoBound` traits.

---

# Appendix

---

## Generics vs. Associated Types

- What is the difference between generics and associated types?

```rust
trait Block<Extrinsic> {
  fn execute(e: Extrinsic)
}
```

vs

```rust
trait Block {
  type Extrinsic;
  fn execute(e: Self::Extrinsic)
}
```

Notes:

In cambridge, I did this this. But since students should now know traits really well, I will drop it.

```rust
trait Engine {
    fn start() {}
}

struct BMW;
impl Engine for BMW {}

trait Brand {
    fn name() -> &'static str;
}

trait Car<E: Engine> {
    type Brand: Brand;
}

struct KianCarCo;
impl Brand for KianCarCo {
  fn name() -> &'static str {
    "KianCarCo!"
    }
}

struct MyCar;
impl<E: Engine> Car<E> for MyCar {
    type Brand = MyBrand;
}

fn main() {
    // Car<E1>, Car<E2> are different traits!

    // Generics can be bounded, or constrained
    // impl<E: Engine> Car<E> {}
    // impl Car<BMW> {}

    // Associated types can:
    // only be bounded when being defined,
    // Can be constrained when being implemented, or when the trait is being used.
    fn some_fn<E: Engine, C: Car<E, Brand = MyBrand>>(car: C) {
      // and we are told associated types are more like output types, lets get the brand of car
      let name = <<C as Car<E>>::Brand as Brand>::name();
    }
    fn other_fn<C: Car<BMW, Brand = MyBrand>>(car: C) {

    }

    // now, check this out
}
```

---v

### Speaking of Traits..

Both generics and associated types can be specified, but the syntax is a bit different.

```rust
trait Block<Extrinsic> {
  type Header
}

fn process_block<B: Block<E1, Header = H1>>(b: B)
```

---v

### Speaking of Traits..

- Anything that can be expressed with associated types can also be expressed with generics.
- Associated Types << Generics
- Associated types usually lead to less boilerplate.

</div>

---

## Defensive Programming Contd.

- First reminder: don't panic, unless if you want to punish someone!
- `.unwrap()`? no no

<br/>

- be careful with implicit unwraps in standard operations!
  - slice/vector indexing can panic if out of bound
  - `.insert`, `.remove`
  - division by zero.

---v

### Defensive Programming

- When using operations that could panic, comment exactly above it why you are sure it won't panic.

```rust
let pos = announcements
  .binary_search(&announcement)
  .ok()
  .ok_or(Error::<T, I>::MissingAnnouncement)?;
// index coming from `binary_search`, therefore cannot be out of bound.
announcements.remove(pos);
```

---v

### Defensive Programming: QED

Or when using options or results that need to be unwrapped but are known to be `Ok(_)`, `Some(_)`:

```rust
let maybe_value: Option<_> = ...
if maybe_value.is_none() {
  return "..."
}

let value = maybe_value.expect("value checked to be 'Some'; qed");
```

- Q.E.D. or QED is an initialism of the Latin phrase "quod erat demonstrandum", meaning "**which was to be demonstrated**".

---v

### Defensive Programming

When writing APIs that could panic, explicitly document them, just like the core rust documentation.

```rust
/// Exactly the same semantics as [`Vec::insert`], but returns an `Err` (and is a noop) if the
/// new length of the vector exceeds `S`.
///
/// # Panics
///
/// Panics if `index > len`.
pub fn try_insert(&mut self, index: usize, element: T) -> Result<(), ()> {
  if self.len() < Self::bound() {
    self.0.insert(index, element);
    Ok(())
  } else {
    Err(())
  }
}
```

---v

### Defensive Programming

- Speaking of documentation, [here's a very good guideline](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html)!

````rust
/// Multiplies the given input by two.
///
/// Some further information about what this does, and where it could be used.
///
/// ```
/// fn main() {
///   let x = multiply_by_2(10);
///   assert_eq!(10, 20);
/// }
/// ```
///
/// ## Panics
///
/// Panics under such and such condition.
fn multiply_by_2(x: u32) -> u32 { .. }
````

---v

### Defensive Programming

- Try and not be this guy:

```rust
/// This function works with module x and multiples the given input by two. If
/// we optimize the other variant of it, we would be able to achieve more
/// efficiency but I have to think about it. Probably can panic if the input
/// overflows u32.
fn multiply_by_2(x: u32) -> u32 { .. }
```

---v

### Defensive Programming

- The overall ethos of defensive programming is along the lines of:

```rust
// we have good reasons to believe this is `Some`.
let y: Option<_> = ...

// I am really really sure about this
let x = y.expect("hard evidence; qed");

// either return a reasonable default..
let x = y.unwrap_or(reasonable_default);

// or return an error (in particular in dispatchables)
let x = y.ok_or(Error::DefensiveError)?;
```

Notes:

But, for example, you are absolutely sure that `Error::DefensiveError` will never happen, can we enforce it better?

---v

### Defensive Programming

```rust
let x = y
  .ok_or(Error::DefensiveError)
  .map_err(|e| {
    #[cfg(test)]
    panic!("defensive error happened: {:?}", e);

    log::error!(target: "..", "defensive error happened: {:?}", e);
  })?;
```

---v

### Defensive Programming

- Yes: [Defensive traits](https://paritytech.github.io/substrate/master/frame_support/traits/trait.Defensive.html):

```
// either return a reasonable default..
let x = y.defensive_unwrap_or(reasonable_default);

// or return an error (in particular in dispatchables)
let x = y.defensive_ok_or(Error::DefensiveError)?;
```

It adds some boilerplate to:

1. Panic when `debug_assertions` are enabled (tests).
1. append a `log::error!`.
