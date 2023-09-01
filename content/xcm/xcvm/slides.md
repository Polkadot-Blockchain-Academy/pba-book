---
title: XCVM
description: Learning about the XCVM state machine
duration: 1 hour
---

# XCVM

---

# 🫀 The XCVM

At the core of XCM lies the **Cross-Consensus Virtual Machine (XCVM)**.

A “message” in XCM is an XCVM program, which is a sequence of instructions.

The XCVM is a state machine, state is kept track in **registers**.

Notes:

It’s an ultra-high level non-Turing-complete computer.
Messages are one or more XCM instructions.
The program executes until it either runs to the end or hits an error, at which point it finishes up and halts.
An XCM executor following the XCVM specification is provided by Parity, and it can be extended or customized, or even ignored altogether and users can create their own construct that follows the XCVM spec.

---

# 📜 XCVM Instructions

XCVM Instructions might change a register, they might change the state of the consensus system or both.

---v

## Kinds of instructions

<pba-flex center>

- Command
- Trusted Indication
- Information
- System Notification

---v

## Example: TransferAsset

An instruction used to transfer assets to some other address.

<pba-flex center>

```rust
TransferAsset {
    assets: Assets,
    beneficiary: Location,
}
```

Notes:

This instruction is a command.
It needs to know which assets to transfer and to which account to transfer them to.

---

# XCVM Registers

<diagram class="mermaid">
graph LR
    subgraph Registers[ ]
        Holding(Holding)
        Origin(Origin)
        More(...)
    end
</diagram>

Notes:

Registers _are_ the state of XCVM.
Note that they are temporary/transient.
We'll talk about are the `holding` and `origin` registers, but there are more.

---v

## 📍 The Origin Register

Contains an `Option<Location>` of the cross-consensus origin where the message originated from.

Notes:

This `Location` can change over the course of program execution.

It might be `None` because some instructions clear the origin register.

---v

### 💸 The Holding Register

Expresses a number of assets in control of the xcm execution that have no on-chain representation.

They don't belong to any account.

It can be seen as the register holding "unspent assets".

---

# Basic XCVM Operation

<diagram class="mermaid">
graph LR
    subgraph Program
        WithdrawAsset-->BuyExecution
        BuyExecution-->DepositAsset
        DepositAsset
    end
    Program-.->Fetch
    Fetch(Fetch Instruction)-->Execute(Execute instruction)
    Execute-->Fetch
    Execute-.->Registers
    subgraph Registers
        Holding(Holding)
        Origin(Origin)
        More(...)
    end
</diagram>

Notes:

The XCVM fetches instruction from the program and executes them one by one.

---v

## XCVM vs. Standard State Machine

<pba-flex center>

1. Error _handler_ register
2. Appendix register

Notes:

1. Code that is run in the case where the XCM program fails or errors.
   Regardless of the result, when the program completes, the error handler register is cleared.
   This ensures that error handling logic from a previous program does not affect any appended code (i.e. the code in the error handler register does not loop infinitely, the code in the Appendix register cannot access the result of the code execution in the error handler).
2. Code that is run regardless of the execution result of the XCM program.

---v

## More complete XCVM operation

<diagram class="mermaid">
graph LR
    subgraph Program
        WithdrawAsset-->BuyExecution
        BuyExecution-->DepositAsset
        DepositAsset
    end
    Program-.->Fetch
    Fetch(Fetch Instruction)-->Execute(Execute instruction)
    Execute-->Fetch
    Execute-.->Registers
    subgraph Registers
        Holding(Holding)
        Origin(Origin)
        ErrorRegister(Error)
        ErrorHandler(Error Handler)
        AppendixRegister(Appendix)
        More(...)
    end
    Execute-- Error -->Error(Error Handler)
    Error-.->ErrorHandler
    Error-.->ErrorRegister
    Error-->Appendix
    Appendix-.->AppendixRegister
    Execute-->Appendix
</diagram>

---

# 💁 XCM by example

---v

## The `WithdrawAsset` instruction

<pba-flex center>

```rust
enum Instruction {
    /* snip */
    WithdrawAsset(Assets),
    /* snip */
}
```

Notes:

There are a number of instructions
which place assets on the Holding Register.
One very simple one is the
`WithdrawAsset` instruction.

It withdraws some assets from the account of the location specified in the Origin Register.
But what does it do with them?
If they don’t get deposited anywhere then it's a pretty useless operation.
These assets are held in the holding register until something is done with them, for example, using the following instruction.

---v

## The `BuyExecution` instruction

<pba-flex center>

```rust
enum Instruction {
    /* snip */
    BuyExecution {
        fees: Asset,
        weight_limit: WeightLimit,
    },
    /* snip */
}
```

Notes:

This instruction uses the specified assets in the Holding register to buy weight for the execution of the following instructions.
It's used in systems that pay fees.

`weight_limit` is a sanity check, to make sure that the execution errors if you would buy more than that weight.
The estimate for the weight has to come from using the recipient's weigher, not the sender's.
The recipient is the one who actually executes the message.

---v

## The `DepositAsset` instruction

<pba-flex center>

```rust
enum Instruction {
    /* snip */
    DepositAsset {
        assets: AssetFilter,
        beneficiary: Location,
    },
    /* snip */
}
```

Notes:

Takes assets from the holding register and deposits them in a beneficiary.
Typically an instruction that places assets into the holding register would have been executed previously.

---v

## Putting it all together

<pba-flex center>

```rust
Xcm(vec![
    WithdrawAsset((Here, amount).into()),
    BuyExecution {
        fees: (Here, amount).into(),
        weight_limit: Limited(sanity_check_weight_limit)
    },
    DepositAsset { assets: All.into(), beneficiary: AccountId32 { ... }.into() },
])
```

Notes:

All examples in these slides use the latest xcm version.

---v

## Good pattern

<pba-flex center>

```rust
Xcm(vec![
    WithdrawAsset((Here, amount).into()),
    BuyExecution {
        fees: (Here, amount).into(),
        weight_limit: Limited(sanity_check_weight_limit)
    },
    DepositAsset { assets: All.into(), beneficiary: AccountId32 { ... }.into() },
    RefundSurplus,
    DepositAsset { assets: All.into(), beneficiary: sender }
])
```

---

# Reserve asset transfer

<pba-flex center>

```rust
Xcm(vec![
    WithdrawAsset(asset),
    InitiateReserveWithdraw {
        assets: All.into(),
        reserve: reserve_location,
        xcm: /* ...what to do with the funds in the reserve... */,
    },
])
```

Notes:

This message is executed locally.
Then, a message is sent to the `reserve` location.
That message contains the custom `xcm` provided along with other instructions.

---v

## Message received in reserve

<pba-flex center>

```rust
Xcm(vec![
    WithdrawAsset(reanchored_asset),
    ClearOrigin, // <- Why is this needed?
    /* ...custom instructions... */
])
```

Notes:

This is the message the reserve receives.

The `ClearOrigin` instruction deletes the content of the origin register.
This is needed because we don't trust the origin to do anything other than move its own assets.

---v

## Custom XCM

<pba-flex center>

```rust
let xcm_for_reserve = Xcm(vec![
    DepositReserveAsset {
        assets: All.into(),
        dest: location,
        xcm: Xcm(vec![
            DepositAsset {
                assets: All.into(),
                beneficiary: AccountId32 { ... }.into(),
            },
        ]),
    },
]);
```

Notes:

For a simple reserve asset transfer, this message will work.

---v

## Message received in destination

<pba-flex center>

```rust
Xcm(vec![
    ReserveAssetDeposited(reanchored_asset),
    ClearOrigin, // <- Why is this needed?
    /* ...custom instructions... */
])
```

Notes:

A very clear exploit in not having `ClearOrigin` here is syphoning all funds from
the reserve's sovereign account in the destination.
The destination can't trust the reserve to totally speak for the source, only for the assets.

---

# Summary

<pba-flex center>

- XCVM
- Kinds of instructions
- Registers
  - Origin
  - Holding
  - Error handler
  - Appendix
- Instructions
  - WithdrawAsset, BuyExecution, DepositAsset
  - RefundSurplus
  - InitiateReserveWithdraw, ReserveAssetDeposited

---v

## Next steps

<pba-flex center>

1. Blog series introducing XCM: Parts [1](https://medium.com/polkadot-network/xcm-the-cross-consensus-message-format-3b77b1373392), [2](https://medium.com/polkadot-network/xcm-part-ii-versioning-and-compatibility-b313fc257b83), and [3](https://medium.com/polkadot-network/xcm-part-iii-execution-and-error-management-ceb8155dd166).
1. XCM Format [repository](https://github.com/paritytech/xcm-format)
1. XCM [Docs](https://paritytech.github.io/xcm-docs/)
