---
title: XCM Pallet
description: The link between XCM and FRAME.
duration: 1 hour
---

# XCM Pallet

---v

## What you'll learn

The link between XCM and FRAME.

How to integrate XCM in your chains.

---

# How XCM can be used

<pba-flex center>

- Interacting directly with the executor by writing XCMs

or

- Packaging up all XCM-logic on extrinsics

---

# The XCM Pallet

We have now learnt about the XCVM and FRAME.

The XCM pallet is the bridge between the XCVM subsystem and the FRAME subsystem.

**It allows us both to interact with the executor directly and provides useful extrinsics**.

---v

## Some functionalities of the pallet

<pba-flex center>

- Executing XCMs locally.
- Sending XCMs to a different location.
- Cross-Consensus Transfers.
- Version negotiation
- Handling responses
- Asset trapping

---

# Executing XCMs locally

<diagram class="mermaid limit size-40">
<!-- prettier-ignore-start -->
flowchart TD
subgraph paraA[Parachain A              .]
  executor --"success?"--> palletxcm
  palletxcm("pallet-xcm") --"execute"--> executor("xcm-executor")
end
execute("execute(xcm)") --> palletxcm
<!-- prettier-ignore-end -->
</diagram>

Notes:

It checks the origin to ensure that the configured `SendXcmOrigin` filter is not blocking the execution.
It executes the message **locally** and returns the outcome as an event.

---

# Sending XCMs

<diagram class="mermaid" style="display: flex; width: 150%; justify-content: center; transform: translateX(-17%);">
<!-- prettier-ignore-start -->
flowchart LR
subgraph paraA[Parachain A]
palletxcma("pallet-xcm") --"deliver"--> routera("xcm-router")
routera --> mqueuea("message queue")
end

subgraph paraB[Parachain B]
mqueueb("message queue") --> executorb("xcm-executor")
end

send("send(xcm)") --> palletxcma
mqueuea --> mqueueb

<!-- prettier-ignore-end -->
</diagram>

Notes:

This extrinsic is a function to send a message to a destination.
It checks the origin, the destination and the message.
Then it lets the `XcmRouter` handle the forwarding of the message.

---

# Version negotiation

The XCM pallet stores the latest supported version for all known chains.

The instructions `SubscribeVersion` and `UnsubscribeVersion` are used to subscribe to chains to know their versions.

If the version of a destination is not known, the lowest known XCM version will be used, in order to be compatible.

---

# Handling responses

The XCM pallet allows creating query ids and awaiting for responses to XCMs.

With an instruction like `ReportError`, you can know if your operation was successful or not.

---

# Claiming trapped assets

At the end of execution, if assets are left in the holding register, they are trapped.

These assets live in a storage item under the XCM pallet\*.

The pallet provides an extrinsic to claim them.

Notes:

- Only if the pallet is configured to be the asset trapper.

---

# Example (continued)

---v

<img src="img/Example Flow - XCM Pallet.png">

---

# Summary

<pba-flex center>

- Learnt the link between XCM and FRAME.
- Learnt what the XCM pallet does.

---

# Next steps

What about the XCM configuration we mentioned before?
