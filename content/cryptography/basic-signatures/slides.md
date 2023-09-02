---
title: Digital Signature Basics
description: Hands-on application of digital signature
duration: 1 hour
---

# Digital Signatures Basics

---

## Signature API

Signature libraries should generally all expose some basic functions:

- `fn generate_key(r) -> sk;` <br /> Generate a `sk` (secret key) from some input `r`.
- `fn public_key(sk) -> pk;` <br /> Return the `pk` (public key) from a `sk`.
- `fn sign(sk, msg) -> signature;` <br /> Takes `sk` and a message; returns a digital signature.
- `fn verify(pk, msg, signature) -> bool;` <br /> For the inputs `pk`, a message, and a signature; returns whether the signature is valid.

Notes:

The input `r` could be anything, for example the movement pattern of a mouse.

For some cryptographies (ECDSA), the verify might not take in the public key as an input. It takes in the message and signature, and returns the public key if it is valid.

---

<!-- .slide: data-background-color="#4A2439" -->

# Subkey Demo

## Key Generation and Signing

Notes:

See the Jupyter notebook and/or HackMD cheat sheet for this lesson.

1. Generate a secret key
1. Sign a message
1. Verify the signature
1. Attempt to alter the message

---

## Hash Functions

There are two lessons dedicated to hash functions.<br />But they are used as part of all signing processes.

For now, we only concern ourselves with using Blake2.

---

## Hashed Messages

As mentioned in the introduction,<br />it's often more practical to sign the hash of a message.

Therefore, the sign/verify API may be _used_ like:

<pba-flex center>

- `fn sign(sk, H(msg)) -> signature;`
- `fn verify(pk, H(msg), signature) -> bool;`

</pba-flex>

Where `H` is a hash function (for our purposes, Blake2).<br />
This means the verifier will need to run the correct hash function on the message.

---

## Cryptographic Guarantees

Signatures provide many useful properties:

- Confidentiality: Weak, the same as a hash
- Authenticity: Yes
- Integrity: Yes
- Non-repudiation: Yes

Notes:

If a hash is signed, you can prove a signature is valid _without_ telling anyone the actual message that was signed, just the hash.

---

## Signing Payloads

Signing payloads are an important part of system design.<br />
Users should have credible expectations about how their messages are used.

For example, when a user authorizes a transfer,<br />they almost always mean just one time.

Notes:

There need to be explicit rules about how a message is interpreted. If the same signature can be used in multiple contexts, there is the possibility that it will be maliciously resubmitted.

In an application, this typically looks like namespacing in the signature payload.

---

## Signing and Verifying

<img style="height: 600px" src="./img/sig-verify-flow.svg" />

Notes:

Note that signing and encryption are _not_ inverses.

---

## Replay Attacks

Replay attacks occur when someone intercepts and resends a valid message.<br />
The receiver will carry out the instructions since the message contains a valid signature.

<pba-flex center>

- Since we assume that channels are insecure, all messages should be considered intercepted.
- The "receiver", for blockchain purposes, is actually an automated system.

</pba-flex>

Notes:

Lack of _context_ is the problem.
Solve by embedding the context and intent \_within the message being signed.
Tell the story of Ethereum Classic replays.

---

## Replay Attack Prevention

Signing payloads should be designed so that they can<br />only be used _one time_ and in _one context_.<br />
Examples:

<pba-flex center>

- Monotonically increasing account nonces
- Timestamps (or previous blocks)
- Context identifiers like genesis hash and spec versions

---

# Signature Schemes

---

## ECDSA

- Uses Secp256k1 elliptic curve.
- ECDSA (used initially in Bitcoin/Ethereum) was developed to work around the patent on Schnorr signatures.
- ECDSA complicates more advanced cryptographic techniques, like threshold signatures.
- Nondeterministic

---

## Ed25519

- Schnorr signature designed to reduce mistakes in implementation and usage in classical applications, like TLS certificates.
- Signing is 20-30x faster than ECDSA signatures.
- Deterministic

---

## Sr25519

Sr25519 addresses several small risk factors that emerged<br />from Ed25519 usage by blockchains.

---

## Use in Substrate

- Sr25519 is the default key type in most Substrate-based applications.
- Its public key is 32 bytes and generally used to identify key holders (likewise for ed25519).
- Secp256k1 public keys are _33_ bytes, so their _hash_ is used to represent their holders.

---

<!-- .slide: data-background-color="#4A2439" -->

# Questions
