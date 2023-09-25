# 📒 Book Overview

This book contains a set of course materials covering both the conceptual underpinnings and hands-on experience in developing blockchain and web3 technologies.
Students will be introduced to core concepts in economic, cryptographic, and computer science fields that lay the foundation for approaching web3 development, as well as hands-on experience developing web3 systems in Rust, primarily utilizing the ecosystem of tooling provided by Polkadot and Substrate.

> 🙋 This book is designed specifically for use in an **_in-person course_**.
> This provides _far more value_ from these materials than an online only, self-guided experience could provide.
>
> ✅ The Academy encourages everyone to [apply to the program](https://dot.li/pba-github)
> Our program is facilitated a few times a year at prestigious places around the world, with on the order of ~50-100 students per cohort.

## 👨‍🎓 Learning Outcomes

By the end of the Polkadot Blockchain Academy, students will be able to:

- Apply economic, cryptographic, and computer science concepts to web3 application design
- Robustly design and evaluate security of web3, both at the protocol and user application level
- Write a smart contract using one of a number of languages and deploy it to a blockchain
- Implement a Substrate based blockchain
- Deploy a parachain utilizing Substrate, Cumulus, and Polkadot
- Employ FRAME to accelerate blockchain and parachain development
- Configure XCM for cross-consensus messaging between parachains

## 🖋️ Nomenclature

The academy uses _explicit terms_ to describe materials use within as _content categories_ defined here:

- **Lesson**: a segment of content (1-2 hours) that is one of:
  - **Lecture**: An oral presentation that consists _primarily_ of slide based content.
    _Most_ content in this book is of this type.
    - **Exercise**: a short (5-10 minutes) exercise for to be **completed** during a lecture (code snippets, mini-demos, etc.).
  - **Workshop**: these are step-by-step, longer (0.5-3 hours) **guided** in-class material (live-coding, competitions, games, etc.).
    Workshops are instructor lead, and hand-held to get everyone to the same result.
  - **Activity**: these are **self-directed** activities for individuals and/or small groups.
    Activities are _not_ guided or "hand-held" by the instructor like workshops are.
- **Assignment**: a **_graded_** piece of work, typically one per week is assigned.
  - **Assignments are _not_ public** - these are only accessible by Academy Faculty, Staff, and (in a derivative form) Students.

## 🪜 Course Sequence

The course is segmented into **modules**, with the granular **lessons** intended to be completed in the sequence provided in the left-side navigation bar.

<!-- prettier-ignore-start -->

| Module                                                        | Topic                                                                                                                |
| ------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| [🔐 Cryptography](./cryptography/)                            | _Applied_ cryptography concepts and introduction to many common tools of the trade for web3 builders.                |
| [🪙 Economics and Game Theory](./economics/)                  | _Applied_ economics and game theory fundamental to the architecture and operation of web3 applications.              |
| [⛓️ Blockchains and Smart Contracts](./blockchain-contracts/) | Blockchain and applications built on them covered in depth conceptually and hands-on operation and construction.     |
| [🧬 Substrate](./substrate/)                                  | The blockchain framework canonical to Polkadot and Parachains covered in depth, at a lower level.                    |
| [🧱 FRAME](./frame/)                                          | The primary Substrate runtime framework used for parachain development.                                              |
| [🟣 Polkadot](./polkadot/)                                    | The Polkadot blockchain covered in depth, focus on high-level design and practically how to utilize its blockspace. |
| [💱 XCM](./xcm/)                                              | The cross consensus messaging format covered from first principals to use in protocols.                              |

<!-- prettier-ignore-end -->

The lessons include materials used, with links and instructions to required external materials as needed.[^except]

[^except]: _Notably, the graded assignments for the Academy and some solutions to public activities and exercises remain closed source, and links are intentionally left out of this book. These materials may be shared as needed with students in person during the Academy._

<!-- FIXME once https://github.com/rust-lang/mdBook/issues/2169#issue-1856015876 is fixed upstream, update to get right behavior for footers! -->
