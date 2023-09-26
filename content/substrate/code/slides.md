---
title: Substrate; Show Me The Code
description: A hands-on dive into practical matters of substrate, such as docs, CLI and folder structure.
duration: 60 minutes
---

# Substrate; Show Me The Code 👨‍💻

---

## Substrate; Show Me The Code 👨‍💻

Previous lecture was all about high level information; now we want to bridge that to real code.

---

## A Word on Previous Versions

- This is a brand new lecture replacing two old ones, with more focus on rust-docs.
- Since this is the first time, I have kept the old versions around for you to look into

> Cambridge-y (adj) - Overall good quality, but with rough edges or imperfections. Especially when related to PBA content.

<!-- .element: class="fragment" -->

---v

### A Word on the Rust Exam

Two main takeaways from the previous cohort:

- Write more rust-docs, expect them to be read.
- Extensive use of type-system, to prepare you better for FRAME.

Notes:

Personally doing my best to make it hard, but reasonable, such that it prepares you best for
in-depth development of Substrate.

---v

### Interactive

- This lecture will be interactive.
- Try and learn the technique, not the specific topic. <!-- .element: class="fragment" -->
- Try and repeat the process later. <!-- .element: class="fragment" -->

Notes:

what I am trying to do here is to teach you how to plant a tree rather than giving you the apple.

---

## Documentation Resources

Core

- paritytech.github.io
  - `substrate` crate
  - WIP: `frame`, `cumulus` and `polkadot` crate.
- Github
- Substrate/Polkadot StackExchange

High level

- `substrate.io`\*
- Discord, Telegram, etc.

---

## Exploring the `substrate` crate.

<https://paritytech.github.io/substrate/master/substrate/index.html>

---v

### Substrate From Within

Division of substrate when seen from inside:

1. `sp`
2. `sc`
3. `frame/pallet/runtime`

Notes:

this should be covered

---v

### Substrate Binaries

Notes:

alternative way is to search for `[[bin]]` in all toml files.

---v

### Structure of a Binary Crate

Division of a typical substrate-based project:

1. `node`
   1. Contains a `main.rs`
   2. `service.rs`
   3. and more!
2. `runtime`
   1. Contains a `/src/lib.rs` ("_runtime amalgamator_")
3. more!

Notes:

node is client side entry point, `runtime amalgamator for the runtime`.

- looking at node-template, it only has the two.
- node has even more
- polkadot has even more.

---v

### Substrate CLI

Study in the docs:

- `--dev`
- `--chain`
- `--tmp`, `--base-path`, `purge-chain`.

Notes:

all commands: <https://paritytech.github.io/substrate/master/sc_cli/commands/index.html>
all args to a typical run command <https://paritytech.github.io/substrate/master/sc_cli/commands/struct.RunCmd.html>

But then each node can decide which subset of these it chooses, and how it implements it.

<https://paritytech.github.io/substrate/master/node_template/cli/enum.Subcommand.html>
<https://paritytech.github.io/substrate/master/node_cli/enum.Subcommand.html>

- execution strategies
- database type
- logs
- RPC
- pruning
- sync modes

---v

## Wasm Build + `std` feature.

- How to compile to wasm? `build.rs`!
- just get your `std` features right please!

Notes:

<https://crates.io/crates/substrate-wasm-builder> (seen env variables, pretty useful!)
<https://docs.substrate.io/build/build-process/>

---v

## Chain Specification

Notes:

raw vs not-raw

---

## #1 Rust-Docs Tip Of All Time

- Search traits, find implementations.
- Examples: `trait Block`, `trait Extrinsic`, `trait Header`.

```rust
trait Config {
  type Foo: SomeTrait<Bar>
}
```

<!-- .element: class="fragment" -->

Notes:

Especially in FRAME, oftentimes you have to parameterize your pallets with a pattern like above.
Simply search the trait in the rust-docs, and find the implementors!

---

## Additional Resources! 😋

> Check speaker notes (click "s" 😉)

<img width="300px" rounded src="../scale/img/thats_all_folks.png" />

Note:

One important concept that is important to substrate-based chains, but is somewhat missing here is
`chain-spec`. Make sure to read up about it in the substrate docs.
