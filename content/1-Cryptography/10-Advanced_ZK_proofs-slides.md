---
title: Advanced ZK Proofs
description: Introduction to zero-knowledge proofs and zk-SNARKS
duration: 2 hour
---

# Cryptography Day IV

<pba-flex center>

- Quiz
- Advanced ZK Proofs<!-- .element: class="fragment" data-fragment-index="0" -->
- ZKP Activity<!-- .element: class="fragment" data-fragment-index="1" -->
- Cryptography in Context<!-- .element: class="fragment" data-fragment-index="2" -->

</pba-flex>

---

# Advanced ZK Proofs

---

# Outline

<pba-flex center>

1. [Under the hood of zk-SNARKs](#zk-practice) <!-- .element: class="fragment" data-fragment-index="1" -->
1. [Introduction to Plonk](#plonk) <!-- .element: class="fragment" data-fragment-index="2" -->
1. [Polynomial commitment](#polynomial-commitment) <!-- .element: class="fragment" data-fragment-index="3" -->
1. [Generating ZK-proofs using Circom and snarkjs](#circom-snarkjs) <!-- .element: class="fragment" data-fragment-index="4" -->

</pba-flex>

---

## Circuit to SNARK Strategy

- To represent the circuit as a univariate polynomial called the "All Gate Polynomial".<!-- .element: class="fragment" data-fragment-index="1" -->
- The all-gate polynomial is equal to zero at each "gate" of the circuit if the solution satisfies the gate relation.<!-- .element: class="fragment" data-fragment-index="2" -->
- Then the verifier should be able to test if the polynomial actually has a root for every gate.<!-- .element: class="fragment" data-fragment-index="3" -->
- ... without knowing the polynomial: This is done using "polynomial commitment".<!-- .element: class="fragment" data-fragment-index="4" -->

---

## Universal PLONK Gate

<img style="height: 200px; padding-left:100px" src="./img/factorization-circuit.png" />

- Supppose we have a left input $a$ and a right input $b$ and we are doing some addition and multiplication with them and the output is $c$.<!-- .element: class="fragment" data-fragment-index="1" -->
- Then we could encode all of these operations as:<!-- .element: class="fragment" data-fragment-index="2" -->
  $Q_l\times a + Q_r \times b + Q_o \times c + Q_m \times a\times b + Q_c = 0$<!-- .element: class="fragment" data-fragment-index="3" -->
- for some constant $Q_l$ $Q_r$ $Q_o$ $Q_m$ and $Q_c$<!-- .element: class="fragment" data-fragment-index="4" -->
- In fact all the operation we discussed can be written using one of these gates.<!-- .element: class="fragment" data-fragment-index="5" -->

---

## Gate table for factorization

$r \times s = N$

$Q_l\times a + Q_r\times b + Q_o\times c + Q_m\times a\times b + Q_c = 0$

 <img style="height: 200px; padding-left:100px" src="./img/gate-table-factorization.png" />

---

## Gate table for left input to be small and not 1

$r_{01} = r_{0} + 2r_{1}$

$r = r_{01} + 4r_{2}$

$r_{i} \times (r_{i} - 1) = 0 \Rightarrow r_{i}^2 - r_{i} = 0$

$(r-1)\times\frac{1}{r - 1} = 1 \Rightarrow r\frac{1}{r - 1} - \frac{1}{r - 1} = 1$

$Q_l\times a + Q_r\times b + Q_o\times c + Q_m\times a\times b + Q_c = 0$

<img style="height: 500px; padding-left:100px" src="./img/gate-table-left-input-less-than-8-and-not-1.png" />

---

## Gate table for the right input to be an integer and not 1

$Q_l\times a + Q_r\times b + Q_o\times c + Q_m\times a\times b + Q_c = 0$
<img style="height: 500px; padding-left:100px" src="./img/gate-table-right-input-less-than-8-and-not-1.png" />

---

## Encode all gate values as a polynomial G

- You can always encode a column of a table into a polynomial.<!-- .element: class="fragment" data-fragment-index="1" -->
- $Q_l(x)$ such that $Q_l(1) = 0, Q_l(2) = 1, Q_l(3) = 1, Q_l(4) = -1 ,...$<!-- .element: class="fragment" data-fragment-index="2" -->
  <img style="height: 300px; padding-left:100px" src="./img/gate-table-left-input-less-than-8-and-not-1.png" /><!-- .element: class="fragment" data-fragment-index="2" -->
- When you have one polynomial for each column then you can turn the whole table into a polynomial:<!-- .element: class="fragment" data-fragment-index="3" -->
  $Q_l(x)\times a(x) + Q_r(x)\times b(x) + Q_o(x)\times  c(x) + Q_m(x)\times a(x)\times b(x) + Q_c(x)$<!-- .element: class="fragment" data-fragment-index="4" -->
  $= 0$<!-- .element: class="fragment" data-fragment-index="4" -->

---

## Compute the all-gate polynomial from the gate table

SAGE demo

---

## Prove the Validity of G

- G encode every gate is evaluated correctly: Zero test.

---

# Zero test

- if f(x) = 0 for x = 1,..,13 then
- $f(x) = q(x) \times  (x-1)\times ...\times (x-13)$
- $f(x)/q(x) = (x-1)...(x-13)$
- How to verifier this.

---

# Zero test on the resulting polynomial.

SAGE demo

---

# Proving the correctness of the wiring

- So far we have proven that the we have a solution which satisfies each round of PLONK gate.<!-- .element: class="fragment" data-fragment-index="1" -->
- However we have not proven that we are using the outputs from previous rounds in correct places in each round.<!-- .element: class="fragment" data-fragment-index="2" -->
- So the prover could cheat and use the values.<!-- .element: class="fragment" data-fragment-index="3" -->
- The verifier need enforce equality of reused values in different rounds.<!-- .element: class="fragment" data-fragment-index="4" -->
- This is actually the hard creative bit in PLONK and that is what P stands for in PLONK.<!-- .element: class="fragment" data-fragment-index="5" -->

---

# The Wiring in a Glance

## <img style="height: 700px; padding-left:100px" src="./img/wiring-fans-with-constraints.png" />

# The Trace polynomial

<img style="height: 700px; padding-left:100px" src="./img/trace-polynomial-known-points.png" />

---

# The Trace poly with Wiring

<img style="height: 700px; padding-left:100px" src="./img/trace-polynomial-known-points-with-wiring.png" />

---

# The wiring permutation: $T(i)=T(\psi(i))$

<img style="height: 700px; padding-left:100px" src="./img/psi-with-wirings.png" />

---

# Naive Permutation check wtih zero test

- We could find polynomial $\psi$. <!-- .element: class="fragment" data-fragment-index="1" -->
- Then compute $T(\psi(x))$. <!-- .element: class="fragment" data-fragment-index="2" -->
- Then compute $T(\psi(x)) - T(x)$. <!-- .element: class="fragment" data-fragment-index="3" -->
- Run Zero-test to erify that $T(\psi(x)) - T(x)/((x - 1)...(x - 39))$ is a polynomial. <!-- .element: class="fragment" data-fragment-index="4" -->
- We will end up with a degree $38\times38 = 1444$ polynomial. <!-- .element: class="fragment" data-fragment-index="5" -->
- It is impractical. <!-- .element: class="fragment" data-fragment-index="6" -->

---

# SAGE demo: Computing the trace and the wiring

SAGE demo

---

# Developing a wiring enforcement gadget/polynomial

- We take a step back and develop some tools to tackle this. <!-- .element: class="fragment" data-fragment-index="1" -->
- They sound random and irrelevant at first but it all makes sense at the end. <!-- .element: class="fragment" data-fragment-index="2" -->

---

# Product check

- We have a polynomial $f(x)$ and we want to prove that:<!-- .element: class="fragment" data-fragment-index="1" -->
- $\prod_{i \in \{1..39\}}f(i) = 1$.<!-- .element: class="fragment" data-fragment-index="2" -->
- We could perform a a zero test $\prod_{i \in \{1..39\}}f(i)$ but the degree is huge. <!-- .element: class="fragment" data-fragment-index="3" -->
- Instead we introduce a new polynomial: <!-- .element: class="fragment" data-fragment-index="4" -->
- $t(x) = \prod_{i \in \{1..x+1}}f(i)$ <!-- .element: class="fragment" data-fragment-index="5" -->.
- We have a nice recursion: $t(x + 1) = t(x)f(x+1)$ for $x \in \{1..39}$ <!-- .element: class="fragment" data-fragment-index="6" -->

---

# Product check

- The observeration is that if you have the recursion: <!-- .element: class="fragment" data-fragment-index="1" -->
- $t(x + 1) = t(x)f(x+1)$ for $x \in \{1..39}$ <!-- .element: class="fragment" data-fragment-index="2" -->
- And you know $ t(39) = 1 $ then you know that: <!-- .element: class="fragment" data-fragment-index="3" -->
- $\prod\_{i \in \{1..39}}f(i) = 1$ . <!-- .element: class="fragment" data-fragment-index="4" -->
- We intepolate $t$ and it will have degree 38 (vs $38 \times 38$) <!-- .element: class="fragment" data-fragment-index="5" -->
- We run a zero test on $t(x + 1) - t(x)f(x+1) = 0$ for $\{1,...,39\}$. <!-- .element: class="fragment" data-fragment-index="6" -->

---

# Ratio check

- We can run the product check to prove $\prod_{i \in \{1..39\}}f(i)/g(i) = 1$.<!-- .element: class="fragment" data-fragment-index="1" -->
- $t(x + 1) = t(x)f(x+1)/g(x + 1)$ <!-- .element: class="fragment" data-fragment-index="2" -->
- We can only run a zero test polynomials. <!-- .element: class="fragment" data-fragment-index="3" -->
- Run zero test on $t(x + 1)g(x + 1) - t(x)f(x+1)$. <!-- .element: class="fragment" data-fragment-index="4" -->

---

# Permutation check

- Now we want to use the ratio check to enforce our wiring. <!-- .element: class="fragment" data-fragment-index="1" -->
- We have $T(a) = T(\psi(a))$ then <!-- .element: class="fragment" data-fragment-index="2" -->
- $\{ (a, T(a))| \textrm{ for all } a \in \{1,..,39\}\} == {(\psi(a), T(a))| \textrm{ for all } a \in \{1,...,39\}\}$ <!-- .element: class="fragment" data-fragment-index="3" -->
- Then for any random $u_1, u_2$ <!-- .element: class="fragment" data-fragment-index="4" -->
- $\prod_{a\in\{1,..,39\}}\frac{u_1 - u_2 \times a - T(a)}{u_1 - u_2 \times \psi(a) - T(\psi(a))} = 1$. <!-- .element: class="fragment" data-fragment-index="5" -->

---

# Proof of wiring being correct

- The verifier runs a zero test on $T(x) - N$.
- The verifier runs a zero test on $T(3x) - a(x)$, $T(3x+1) - b(x)$, $T(3x+2) - c(x)$
- A Permutation check on $T(x)$ and $T(\psi(x))$

---

# SAGE demo: Proof of correctness of the wiring on the trace polynomial.

SAGE Demo

Zero test on:
$t(x + 1)(u_1 - u_2 \times (x+1) - T(x + 1)) - t(x)(u_1 - u_2 \times (\psi(x)+1) - T(\psi(x)+1) = 0$

---

# Zero test without knowing the polynomial: Polynomial commitment

- When the prover tells the $f(x)$ at some point $u$ ($f(u)$) without revealing $f(x)$.<!-- .element: class="fragment" data-fragment-index="1" -->
- It is a tool to convince the verifier which it has done so honestly.<!-- .element: class="fragment" data-fragment-index="2" -->
- The prover first commit to the polynomial $f(x)$ so later on, they can't back off and cheat (and use another polynomial).<!-- .element: class="fragment" data-fragment-index="3" -->
- Then the verifier is going to ask the prover to evaluate the polynomials in random point $u$.<!-- .element: class="fragment" data-fragment-index="4" -->
- The verifier is able to be confident that $f(u) = v$.<!-- .element: class="fragment" data-fragment-index="5" -->

---

# Zero test using polynomial commitment

- The prover claims it has $f(x)$ passing the zero test.<!-- .element: class="fragment" data-fragment-index="1" -->
- The prover is also able to compute $q(x)$ such that <!-- .element: class="fragment" data-fragment-index="2" -->
- $f(x) = q(x) \times  \prod(x-1)..(x-13)$<!-- .element: class="fragment" data-fragment-index="3" -->
- The prover commit to $f$ and $q$.<!-- .element: class="fragment" data-fragment-index="4" -->
- The verifier ask the prover to provide them with $f(u)$ and $q(u)$ for some random point $u$<!-- .element: class="fragment" data-fragment-index="5" -->
- It is very unlikely that the prover is able to lie about $f(u)$ and $q(u)$ given he has commited to $f$ and $q$. <!-- .element: class="fragment" data-fragment-index="6" -->
- The verifier computes $\prod(u-1)...(u-13)$<!-- .element: class="fragment" data-fragment-index="7" -->
- The verifier verifies that $f(u) = q(u)\times \prod(u-1)...(u-13)$ and if so believes that the prover has a solution.<!-- .element: class="fragment" data-fragment-index="8" -->

---

# KZG Polynomial-commitment

- Is one of the most space efficient polynomial commitment.<!-- .element: class="fragment" data-fragment-index="1" -->
- Uses elliptic curve cryptography.<!-- .element: class="fragment" data-fragment-index="2" -->
- It requires trusted setup: a pre-computation with toxic waste which needs to be discarded to keep the scheme secure.<!-- .element: class="fragment" data-fragment-index="3" -->

---

## Making ZK non-interactive

- The only interactive step is when verifier is quizzing prover with a random value $r$.<!-- .element: class="fragment" data-fragment-index="1" -->
- We replace that with asking the prover to apply a secure hash function to his commitment to generate $r$.<!-- .element: class="fragment" data-fragment-index="2" -->
- That way if the prover changes his commitment his point also changes without his control. <!-- .element: class="fragment" data-fragment-index="3" -->

---

## Use Circom to generate trace polynomials.

Circom demo

---

## Use snarkjs to generate proofs

Generate proof demo with snarkjs

---

## Use snarkjs to verify the proofs

Verify the proof snarkjs

---

<!-- .slide: data-background-color="#4A2439" -->

# Questions
