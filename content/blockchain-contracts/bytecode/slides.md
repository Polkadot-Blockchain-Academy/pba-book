---
title: Platform Agnostic Bytecode
description: What are PABs and why they exist?
duration: 1 hour
---

# Platform Agnostic Bytecode

---

## Review of Compilers

<img src="./img/compiling.png" />

🤯 Fun Side Reading: <!-- .element: class="fragment" data-fragment-index="1" -->
[Reflections on Trusting Trust](https://www.cs.cmu.edu/~rdriley/487/papers/Thompson_1984_ReflectionsonTrustingTrust.pdf) <!-- .element: class="fragment" data-fragment-index="1" -->

Notes:

Just a very quick reminder of how compilers work.
Humans write programs in some human readable language like Lauren talked about.
Then the compiler translates the semantics of that program into an equivalent program in a much lower more machine-readable language called a bytecode.

CLICK

Whenever I show this diagram or talk about compilers, I always like to mention one of my favorite essays ever.
Ken Thompson's 1984 Turing Award lecture.

---

## Definition

A PAB is a bytecode that follows two main principles:

- Turing Completeness, as a standard bytecode would respect

<!-- .element: class="fragment" data-fragment-index="1" -->

- Support for tooling that makes it executable on every machine

<!-- .element: class="fragment" data-fragment-index="2" -->

Notes:

Ideally a bytecode like this is designed to be executed on a virtual machine that follows general known patterns.

---

<pba-cols>
<pba-col left>

<pba-flex center>

###### High Level Languages

<img style="width: 30%" src="./img/rust_logo.png" />

<img style="width: 30%" src="./img/c_logo.png" />

<img style="width: 30%" src="./img/c++_logo.png" />

</pba-flex>
</pba-col>
<!-- .element: class="fragment" data-fragment-index="1" -->

<pba-col center>
<pba-flex center>

###### PABs

<img style="width: 30%" src="./img/jvm_logo.png" />
<img style="width: 30%" src="./img/wasm_logo.png" />
<img style="width: 20%" src="./img/eth_logo.png" />
<img style="width: 30%" src="./img/risc-v_logo.png" />

</pba-flex>
</pba-col>
<!-- .element: class="fragment" data-fragment-index="2" -->

<pba-col right>
<pba-flex center>

###### Architecture's bytecode

<img style="width: 30%" src="./img/intel_logo.png" />
<img style="width: 30%" src="./img/arm_logo.jpg" />
<img style="width: 30%" src="./img/risc-v_logo.png" />

</pba-flex>
</pba-col>
<!-- .element: class="fragment" data-fragment-index="3" -->

</pba-cols>

Notes:

From left to right you can see different levels of abstraction over the program that will ultimately be run on some machine.
Generally, from a high level language you need two compilation step if you want to pass through a PAB.

Other examples of PABs used right now:

- Inside the Linux Kernel -> eBPF
- Inside browsers -> Wasm
- Inside Blockchains -> Wasm
  - Full nodes
  - Light nodes (Wasm inside Wasm)
- LLVM Toolchain -> LLVM IR

---v

## Compiling in a PAB

<img src="./img/compiling_twice.png" />

Notes:

So when we are using a PAB, we need to compile twice.
This is, of course, the cost to using a PAB.
In this lesson we'll also explore the advantages.

---

#### What a PAB allows is:

<pba-flex center>

- Portability
  <!-- .element: class="fragment" data-fragment-index="1" -->
      - Avoid Hardware Centralization
  <!-- .element: class="fragment" data-fragment-index="3" -->
- Determinism
  <!-- .element: class="fragment" data-fragment-index="2" -->
      - Make consensus possible
  <!-- .element: class="fragment" data-fragment-index="4" -->

</pba-flex>

Notes:

The main goal of a PAB is to make the code **portable**, you should be able to compile it once and then share it around without caring about the architecture on which will be executed.
Of course in a decentralized network we want that different nodes, with different architectures came up to the same result if the input are the same, that's called **determinism**, if a PAB would not have determinism then reaching consensus is impossible.

---v

##### That's why PABs are so important

---

## Desireable Features

- Hardware Independence

<!-- .element: class="fragment" data-fragment-index="1" -->

- Efficiency

<!-- .element: class="fragment" data-fragment-index="2" -->

- Tool Simplicity

<!-- .element: class="fragment" data-fragment-index="3" -->

- Support as Compilation Target

<!-- .element: class="fragment" data-fragment-index="4" -->

- Sandboxing

<!-- .element: class="fragment" data-fragment-index="5" -->

Notes:

- Hardware Independence: It should not be tightly related to a specific architecture, otherwise the execution on different machine could be convoluted
- Efficiency: the execution of a PAB should be efficient, the problem for a PAB is that in the execution time is also considered the "translation" to the machine's bytecode or the interpretation
- Support as Compilation Target: The PAB should be possible to be compiled by as many as possible High Level languages
- Tool Simplicity: If the tools that makes the PAB executable are extremely complex then nobody will use it

---v

### Sandboxing?

An environment for running untrusted code without affecting the host.

<!-- .element: class="fragment" data-fragment-index="1" -->

<img style="height: 300px" src="./img/sandbox.jpg" />

A SmartContract is _Arbitrary Code_ that may be executed on other people's infrastructure, we don't want SmartContracts capable of destroying the nodes on which they are executed

<!-- .element: class="fragment" data-fragment-index="2" -->

Notes:

CLICK read definition

The term sandbox is an analogy to kids playing in a sandbox.
The parent puts the kid in the sandbox and tells them they can play in the sandbox and they are safe as long as they stay in.
Don't go in the woods and get bitten by a snake or in the road and get hit by a car.
Just stay in the sandbox.

Of course the analogy isn't perfect.
The children in the sandbox stay there because the parent asked them to.
They could leave anytime they wanted to.
For actual untrusted code, a better analogy would be a walled garden or a Jail

---v

### Sandboxing?

<img src="./img/jail.jpg" /> <!-- .element: class="fragment" data-fragment-index="1" -->

A sandboxed environment must be created by the executor of the PAB.

<!-- .element: class="fragment" data-fragment-index="2" -->

Notes:

Of course the security can be seen by various point of view and some examples are:

- Compilation takes too much time -> compiling bomb
- Access to the environment -> "buffer overflow" techniques

Those things can't be addressed by the PAB itself but they can give good guidelines and code design to make an 100% secure implementation of the executor possible.

---

## PAB's lifecycle example

<div class="r-stack">
<img style="width: 70%" src="./img/pab_path_1.svg" />
<img style="width: 70%" src="./img/pab_path_2.svg"/>
<!-- .element: class="fragment" data-fragment-index="1" -->
<img style="width: 70%" src="./img/pab_path_3.svg"/>
<!-- .element: class="fragment" data-fragment-index="2" -->
<img style="width: 70%" src="./img/pab_path_4.svg"/>
<!-- .element: class="fragment" data-fragment-index="3" -->
<img style="width: 70%" src="./img/pab_path_5.svg"/>
<!-- .element: class="fragment" data-fragment-index="4" -->
<img style="width: 70%" src="./img/pab_path_6.svg"/>
<!-- .element: class="fragment" data-fragment-index="5" -->
</div>

---

<!-- .slide: data-background-color="#4A2439" -->

<img rounded style="width: 300px" src="./img/question-mark.svg" />

---

<pba-cols>
<pba-col center>

# WebAssembly

<!-- .element: class="fragment" data-fragment-index="1" -->

</pba-col>
<pba-col center>

<img style="width: 70%" src="./img/wasm_logo.png" />

</pba-col>
</pba-cols>

---

## Wasm's key points

<pba-flex center>

- Hardware-independent
  <!-- .element: class="fragment" data-fragment-index="1" -->
  - Binary instruction format for a stack-based virtual machine
  <!-- .element: class="fragment" data-fragment-index="1" -->
- Supported as compilation target by many languages
  <!-- .element: class="fragment" data-fragment-index="2" -->
  - Rust, C, C++ and many others
  <!-- .element: class="fragment" data-fragment-index="2" -->
- Fast (with near-native performance)

<!-- .element: class="fragment" data-fragment-index="3" -->

- Safe (executed in a sandboxed environment)

<!-- .element: class="fragment" data-fragment-index="4" -->

- Open (programs can interoperate with their environment)

<!-- .element: class="fragment" data-fragment-index="5" -->

</pba-flex>

Notes:

Wasm seems to respect every rating points we defined before

---

## Stack-Based Virtual Machine Example

<pba-cols>
<pba-col center>

Adding two number in wasm text representation (.wat)

<!-- .element: class="fragment fade-out" data-fragment-index="1" -->

```wasm [1-12|5|6|8]
(module
  (import "console" "log" (func $log (param i32)))
  (func $main
    ;; load `10` and `3` onto the stack
    i32.const 10
    i32.const 3

    i32.add ;; add up both numbers
    call $log ;; log the result
  )
  (start $main)
)
```

<!-- .element: class="fragment" data-fragment-index="0" -->

</pba-col>
<pba-col center>

<div class="r-stack">
<img src="./img/stack_1.svg" style="width: 100%">
<!-- .element: class="fragment" data-fragment-index="1" -->
<img src="./img/stack_2.svg" style="width: 100%">
<!-- .element: class="fragment" data-fragment-index="2" -->
<img src="./img/stack_3.svg" style="width: 100%">
<!-- .element: class="fragment" data-fragment-index="3" -->
<img src="./img/stack_4.svg" style="width: 100%">
<!-- .element: class="fragment" data-fragment-index="4" -->
<img src="./img/stack_5.svg" style="width: 100%">
<!-- .element: class="fragment" data-fragment-index="5" -->
<img src="./img/stack_6.svg" style="width: 100%">
<!-- .element: class="fragment" data-fragment-index="6" -->
</div>

</pba-col>
</pba-cols>

Notes:

Wasm has also a text representation,
Wat has some features that allow for better readability:

- Stack push operations can be grouped to its consuming instruction.
- Labels can be applied to elements.
- Blocks can enclosed with parenthesis instead of explicit start/end instructions.

Instructions push results to the stack and use values on the stack as arguments, the compilation process generally translate this stack-based bytecode to register based, where registers are used to pass values to instructions as a primary mechanism.
The compilation will try to elide the wasm stack and work with only the architecture registers.

There is another type of stack used in wasm and that's called: shadow stack, resource to learn more: <https://hackmd.io/RNp7oBzKQmmaGvssJDHxrw>

---

## Wasm seems to be a perfect PAB, but

- How does communication with the environment work?

<!-- .element: class="fragment" data-fragment-index="1" -->

- How the memory is managed?

<!-- .element: class="fragment" data-fragment-index="2" -->

- How is it executed?

<!-- .element: class="fragment" data-fragment-index="4" -->

Notes:

Assuming all the things we said before wasm seems to be perfect but how those things really works?

---

## Communication with the Environment

Let's call **Embedder** the program that will take the wasm blob as input and execute it

<!-- .element: class="fragment" data-fragment-index="0" -->

- the wasm blob may expect parameters from the embedder
  - embedder -> wasm

<!-- .element: class="fragment" data-fragment-index="1" -->

- the embedder may act on a return value from the wasm
  - wasm -> embedder

<!-- .element: class="fragment" data-fragment-index="2" -->

---v

### Problem

**Wasm has no ambient access to the computing environment in which code is executed**

</br>

### Solution

<!-- .element: class="fragment" data-fragment-index="1" -->

<img src="./img/env_communication.svg" style="width: 70%">
<!-- .element: class="fragment" data-fragment-index="1" -->

Notes:

- Every interaction with the environment can be done only by a set of functions, called **Host Functions**, provided by the embedder and imported in wasm
- The embedder is able to call the functions defined in wasm blob, called **Runtime API**, and pass arguments through a shared memory

---

## Memory

In addition to the stack Wasm has also access to memory provided by the embedder, the **Linear Memory**.

<!-- .element: class="fragment" data-fragment-index="0" -->

</br>

- This area will be used also used as a frontier for data sharing
- To make everything secure the Embedder is doing incredibly convoluted things

<!-- .element: class="fragment" data-fragment-index="1" -->

Notes:

From Wasm the Linear Memory is byte addressable
Linear Memory can be manipulated using functions called 'store' and 'load'

The Rust compiler uses for dynamic/heap memory and to pass non primitives values to functions by emulating an additional stack within the linear memory, this emulated stack (the shadow stack) is what we would understand as stack in other architectures

---v

### Example

<div class="r-stack">
<img src="./img/linear_memory_1.svg" style ="width: 70%">
<!-- .element: class="fragment fade-out" data-fragment-index="1" -->
<img src="./img/linear_memory_2.svg" style ="width: 70%">
<!-- .element: class="fragment" data-fragment-index="1" -->
</div>

Notes:

Here's an example, wasm sees linear memory like a byte array and if it tries to access the second byte, it would use an index 1.
When it's time to execute it the embedder will see this access and translate the linear memory access at index 1 to a standard memory access to base_linear_memory + 1.

Buffer overflow? Wasm uses 32 bit, this makes impossible to have an offset bigger then 4GiB, this means that the embedder can leave those 4GiB free in its virtual memory to makes impossible to the wasm blob to access any environment information.
Even if the offset is only positive there are embedded that are defining as protected the 2GiB before the BLM so that if for some reason the wasm code trick the embedder to treat the offset as a signed number that would cause an Operating System error.

---

## How Wasm is executed

<pba-flex left>

There are multiple ways to execute wasm:

- Ahead Of Time Compilation
- Just in Time Compilation
- Single Pass Compilation
- Interpretation
- ...

<!-- .element: class="fragment" data-fragment-index="1" -->

</pba-flex >

Notes:

AOT: Compile all the code at the beginning, this allows to makes a lot of improvement to the final code efficiency
JIT: The code is compiled only when needed, examples are functions that are compiled only when called, this leave space only to partials improvements
SPC: This is a specific technique of compilation that is made in linear time, the compilation is done only passing once on the code
Interpretation: The wasm blob is treated as any other interpreted language and executed in a Virtual Machine

---v

### Wasmtime

- It is a stand alone wasm environment
- Wasmtime is built on the optimizing Cranelift code generator to quickly generate high-quality machine code either at runtime (JIT) or ahead-of-time (AOT)
- It executes the compiled wasm blob in sandboxed environment while keeping everything extremely secure

<!--TODO: graphics-->

Notes:

- wasmtime book: <https://docs.wasmtime.dev/>
- Used in substrate as embedder for the blockchain logic

Cranelift is a fast, secure, relatively simple and innovative compiler backend.
It takes an intermediate representation of a program generated by some frontend and compiles it to executable machine code

---v

#### Wasm lifecycle in Wasmtime

<div class="r-stack">
<img style="width: 70%" src="./img/wasmtime_exec_1.svg" />
<img style="width: 70%" src="./img/wasmtime_exec_2.svg"/>
<!-- .element: class="fragment" data-fragment-index="1" -->
<img style="width: 70%" src="./img/wasmtime_exec_3.svg"/>
<!-- .element: class="fragment" data-fragment-index="2" -->
<img style="width: 70%" src="./img/wasmtime_exec_4.svg"/>
<!-- .element: class="fragment" data-fragment-index="3" -->
</div>

---v

### Wasmi

- It is a wasm environment with support for embedded environment such as WebAssembly itself
- Focus on simple, correct and deterministic WebAssembly execution
- The technique of execution is interpretation but:
  - The wasm code is transpiled to WasmI IR, another stack-based bytecode
  - The WasmI IR is then interpreted by a Virtual Machine

<!--TODO: graphics-->

Notes:

proposal to switch from a stack based ir to registry based ir <https://github.com/paritytech/wasmi/issues/361>

paper explaining the efficiency of translating wasm to registry based code <https://www.intel.com/content/www/us/en/developer/articles/technical/webassembly-interpreter-design-wasm-micro-runtime.html>

Due to it's characteristics it is mainly used to execute SmartContracts on chain

---v

#### Wasm lifecycle in Wasmi

<div class="r-stack">
<img style="width: 70%" src="./img/wasmi_exec_1.svg" />
<img style="width: 70%" src="./img/wasmi_exec_2.svg"/>
<!-- .element: class="fragment" data-fragment-index="1" -->
<img style="width: 70%" src="./img/wasmi_exec_3.svg"/>
<!-- .element: class="fragment" data-fragment-index="2" -->
<img style="width: 70%" src="./img/wasmi_exec_4.svg"/>
<!-- .element: class="fragment" data-fragment-index="3" -->
</div>

<!-- Really nice slide but there's not enough knowledge about substrate

There are also light clients, where both Runtime and Client are implemented in wasm, so we have:

- A browser as embedder of the node's client
  - the node's client as embedder for the node's runtime
    - the node's runtime as embedder for the SmartContract

<img style="height: 30vh" src="./img/mind-blown-explosion.gif" />

We have a double recursion of a PAB that embed itself

-->

---

# Alternatives

---v

## EVM

- The **Ethereum Virtual Machine** executes a stack machine
  - Interesting: here the bytecode was create to be executed in a blockchain, so instructions are not hardware-dependent but there are instruction tightly related to Cryptography and others blockchain instructions

---v

## CosmWasm

- Wasm is always used but with different tools
- They use CosmWasm as Embedder and internally is used Wasmer, a Single Pass Compiler

---v

## Solana eBPF

- eBPF is used as PAB, but intrinsically eBPF has a lot of restrictions
- Solana forked the eBPF backend of LLVM to makes every program to be compiled in eBPF
- The Embedder is rBFP, a virtual machine for eBPF programs

Notes:

<https://forum.polkadot.network/t/ebpf-contracts-hackathon/1084>

---v

## RISC-V ?!

- RISC-V is a new instruction-set architecture
- main goals are:
  - real ISA suitable for direct native hardware implementation
  - avoids “over-architecting”

</br>

Being so simple and "Hardware-Independent" there are work in progress experiments to test if it is suitable to become the new polkadot smart contract language

Notes:

Discussion about using RISC-V as smart contract language: <https://forum.polkadot.network/t/exploring-alternatives-to-wasm-for-smart-contracts/2434>

RISC-V Instruction Set Manual, Unprivileged ISA: <https://github.com/riscv/riscv-isa-manual/releases/download/Ratified-IMAFDQC/riscv-spec-20191213.pdf>

---

## Activity: Compiling Rust to Wasm

- Let's make a simple Rust crate that compiles to Wasm!
- Clone the repo

---v

### Activity: Compiling Rust to Wasm

- A target triple consists of three strings separated by a hyphen, with a possible fourth string at the end preceded by a hyphen.
- The first is the **architecture**, the second is the **"vendor"**, the third is the **OS type**, and the optional fourth is environment type.

* `wasm32-unknown-emscripten`: Legacy, provides some kind of `std`-like environment
* `wasm32-unknown-unknown` ✓ WebAssembly: Can compile anywhere, can run anywhere, no `std`
* `wasm32-wasi` ✓ WebAssembly with WASI

---v

### Rust -> Wasm Details

```rust
#[no_mangle] // don't re-name symbols while linking
pub extern "C" fn add_one() { // use C-style ABI
  ...
}
```

and if a library:

```
[lib]
crate-type = ["cdylib"]
```

---v

### Activity: Compiling Rust to Wasm

```
rustup target add wasm32-unknown-unknown

cargo build --target wasm32-unknown-unknown --release

wasmtime ./target/wasm32-unknown-unknown/release/wasm-crate.wasm --invoke <func_name> <arg1> <arg2> ...
```

---v

## Additional Resources! 😋

> Check speaker notes (click "s" 😉)

Notes:

- More on PAB:

  - <https://github.com/gabriele-0201/IPABDN/blob/main/thesis/IPABDN.pdf>

- More on Rust target spec:

  - <https://rust-lang.github.io/rfcs/0131-target-specification.html>

- Lin Clark's awesome talks on WASI (not super relevant to our work though):

  - <https://www.youtube.com/watch?v=fh9WXPu0hw8>
  - <https://www.youtube.com/watch?v=HktWin_LPf4>

- `wasm-unknown` vs `wasm-wasi`:

  - <https://users.rust-lang.org/t/wasm32-unknown-unknown-vs-wasm32-wasi/78325/5>

- `extern "C"`:

  - <https://doc.rust-lang.org/std/keyword.extern.html>
  - <https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#using-extern-functions-to-call-external-code>

- Chapter 11 of this book is a great read: <https://nostarch.com/rust-rustaceans>
