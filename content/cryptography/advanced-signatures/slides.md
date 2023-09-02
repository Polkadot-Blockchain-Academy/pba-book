---
title: Advanced Digital Signatures
description: More exotic digital signature methods
duration: 1 hour
---

# Advanced Digital Signatures

---

### Certificates

A certificate is essentially a witness statement concerning one or more public keys. It is a common usage of digital signatures, but _it is not a cryptographic primitive_!

Notes:

A certificate is one issuing key signing a message containing another certified key, which attests to some properties or relationship about the certified key.

We must already trust the issuing key to give this attestation any significance, traditionally provided under "Certificate Authority" or "Web of Trust" schemes.

---

### Certificates

A certification system specified conventions on who is allowed to issue certificates, the rules over their issuance (e.g. time limits and revocation) as well as their format and semantics.

For example, the certificate transparency protocol for TLS certificates helps protect against compromised Certificate Authorities.

Notes:

Certificate transparency: [explanation](https://certificate.transparency.dev/howctworks/) and [dashboard](https://ct.cloudflare.com/)

- Maybe mention PGP web-of-trust style schemes

---

### Certificates in Web3

We are building systems that don't have a "Certificate Authority".<br />
But we can still use certificates in some niche instances.

Notes:

Potential example to give verbally:

- Session keys are a set of keys that generally run in online infrastructure.
  An account, whose keys are protected, can sign a transaction to certify all the keys in the set.
- Session keys are used to sign operational messages, but also in challenge-response type games to prove availability by signing a message.

---

### Multi-Signatures

We often want signatures that must be signed<br />by multiple parties to become valid.

<pba-flex center>

- Require some threshold of members to<br />agree to a message
- Protect against key loss

</pba-flex>

---

### Types of Multi-Signature

<pba-flex center>

- Verifier enforced
- Cryptographic threshold
- Cryptographic non-threshold<br />(a.k.a. signature aggregation)

---

### Verifier Enforced Multiple Signatures

We assume that there is some verifier, who can check that some threshold of individual keys have provided valid signatures.

This could be a trusted company or third party.
For our purposes, _it's a blockchain_.

---

### Verifier Enforced Multiple Signatures

Multiple signatures enforced by a verifier generally provide a good user experience, as no interaction is required from the participants.

Notes:

This good experience comes at the cost of using state and more user interactions with the system, but is generally low.

Even in a web3 system, the verifier can be _distinct_ from the blockchain. 5 people can entrust a verifier with the identity of "all 5 signed this" associated to a verifier-owned private key.

---

### Cryptographic Multi-Sigs

We want a succinct way to demonstrate that everyone from some set of parties have signed a message. This is achieved purely on the signer side (without support from the verifier).

<pba-flex center>

_Example: "The five key holders have signed this message."_

---

### Key Generation for Multi-Sigs

In regular multi-signatures,<br />signatures from individual public keys are aggregated.

Each participant can choose their own key to use for the multi-signature.

Notes:

In some cases, a security requirement of these systems is that every participant demonstrates ownership of the public key submitted for the multi-signature, otherwise security can be compromised.

---

### Cryptographic Threshold Multi-Sigs

This makes more compact signatures compatible with legacy systems. Unlike a regular multi-sig, the public key is associated with a _threshold_ number of signing parties, so not all parties are needed to take part in the signing process to create a valid signature.

This requires MPC protocols and may need multiple rounds of interaction to generate the final signature. They may be vulnerable to DoS from a malfunctioning (or malicious) key-holder.

<pba-flex center>

_Example: "5 of 7 key holders have signed this message."_

Notes:

These require multi-party computation (MPC) protocols, which add some complexity for the signing users.

---

### Key Generation - Threshold

Threshold multi-signature schemes require that all signers run a _distributed key generation_ (DKG) protocol that constructs key _shares_.

The secret encodes the threshold behavior, and signing demands some threshold of signature _fragments_.

This DKG protocol breaks other useful things, like hard key derivation.

---

### Schnorr Multi-Sigs

Schnorr signatures are primarily used for threshold multi-sig.

- Fit legacy systems nicely, and can reduce fees on blockchains.
- Reduce verifier costs in bandwidth & CPU time, so great for certificates.
- Could support soft key derivations.

---

### Schnorr Multi-Sigs

However, automation becomes tricky.

We need agreement upon the final signer list and two random nonce contributions from each prospective signer, before constructing the signature fragments.

---

### BLS Signatures

BLS signatures are especially useful for aggregated (non-threshold) multi-signatures (but can be used for threshold as well).

Signatures can be aggregated without advance agreement upon the signer list, which simplifies automation and makes them useful in consensus.

Verifying individual signatures is _slow_, but verifying aggregated ones is relatively fast.

(Coming to Substrate soonish.)

---

### BLS Signatures

Allows multiple signatures generated under multiple public keys for multiple messages to be aggregated into a single signature.

<pba-flex center>

- Uses heavier pairing friendly elliptic curves than ECDSA/Schnorr.
- Very popular for consensus.

<pba-flex>

---

### BLS Signatures

However...

- DKGs remain tricky (for threshold).
- Soft key derivations are typically insecure for BLS.
- Verifiers are hundreds of times slower than Schnorr, due to using pairings, for a single signature.
- But for hundreds or thousands of signatures on the same message, aggregated signature verification can be much faster than Schnorr.

---

### Schnorr and BLS Summary

Schnorr & BLS multi-signatures avoid complicating verifier logic,<br />but introduce user experience costs such as:

- DKG protocols
- Reduced key derivation ability
- Verification speed

---

### Ring Signatures

- Ring signatures prove the signer lies within some "anonymity set" of signing keys, but hide which key actually signed.
- Ring signatures come in many sizes, with many ways of presenting their anonymity sets.
- Anonymous blockchain transactions typically employ ring signatures (Monero, ZCash).

Notes:

- ZCash uses a ring signature based upon Groth16 zkSNARKs which makes the entire chain history be the anonymity set.
- Monero uses ring signatures with smaller signer sets.
- Ring signatures trade some _non-repudation_ for _privacy_.

---

<!-- .slide: data-background-color="#4A2439" -->

# Questions
