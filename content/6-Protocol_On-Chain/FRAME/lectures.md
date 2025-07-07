# New Frame Lectures List

## Pre-requisite:

- sp_runtime stuff

## FRAME Lectures

> Ordering constraint: things that I think should come before one another.

- Error and call refund >> weight. Because of this, weight is explained broadly as a part of
  transactions (quite early), and then there is a separate lecture toward the end about the details
  and benchmarking.

### Essentials

- Intro to FRAME (Kian)

  - simple-pallet
  - breeze over system, covers
    - Transient Storage Items
    - Some of the types
  - simple-runtime
  - executive

- Dispatchable, Calls and Transactions

  - Call Encoding (indices)
  - Weight basics

- Frame Storage (Shawn)

- Pallet Hooks

- Error

  - nested errors
  - sp_runtime DispatchError
  - PostInfo and WeightRefund

- Events

  - Phases,

- Origin
  - filtering
  - custom origins (system)
  - collective origin
  - example

---

### Slightly Less Important.

- Inherents

- Metadata

- Benchmarking (Shawn)

  - Weight classes

- Frame System: Revisiting and Leftover

  - PalletInfoAccess
  - Account Details: consumers, providers, sufficiency, etc
  - ChainContext, whatever the fuck it is.

- Frame Executive: Revisiting and Leftover

- Testing a Pallet (Kian)
  - How to write tests for a pallet.
  - ExtBuilder patter
  - `sp_io::TestExternalities`

### Advance Topics

- Pallets that receive/store calls + Dispatch Them

  - `type Call` and how to deal with it.

- Parity Patterns + Best-Practices

  - sanity-checks by Kian
  - ...

- Transactional layers/dispatch (Shawn)
- Signed Extensions, ValidateUnsigned TransactionValidity (priority) (Kian)
- Digests
- Migration + Try-Runtime (Kian)
- Custom RPCs + Runtime APIs (maybe, we will probably deprecate it)
- Pallet Instancing

### Pallet Gallery!

- Common Pallets: High level, no detail, as they might be subject to change.
  - Balances
  - Transaction Payment
  - Timestamp
  - Utility
  - MultiSig + Proxy
  - Babe + Aura + Grandpa

### Part 4: Interesting Case Studies (for Q&A Sessions and such)

- Staking, Offchain Workers etc.
