# Broken External Links Report

Generated from CI link checker failures.

## 404 Errors (Dead Links)

These links return 404 and need to be fixed or removed.

### Polkadot Blog Links (Domain Changed: polkadot.network → polkadot.com)

| File | Line | Broken URL | Suggested Replacement |
|------|------|------------|----------------------|
| `content/2-Economics/3-Price_Finding_Mechanisms-slides.md` | 872 | `https://polkadot.network/blog/making-history-an-overview-of-the-first-five-parachain-slot-auctions-on-kusama/` | `https://polkadot.com/blog/making-history-an-overview-of-the-first-five-parachain-slot-auctions-on-kusama/` |
| `content/2-Economics/3-Price_Finding_Mechanisms-slides.md` | 873 | `https://polkadot.network/blog/kusama-batch-2-auctions-report/` | `https://polkadot.com/blog/kusama-batch-2-auctions-report/` |
| `content/2-Economics/3-Price_Finding_Mechanisms-slides.md` | 874 | `https://polkadot.network/blog/making-history-again-polkadot-auctions-1-5/` | `https://polkadot.com/blog/making-history-again-polkadot-auctions-1-5/` |
| `content/2-Economics/4-Economics_of_Polkadot-slides.md` | 549 | `https://polkadot.network/blog/nominating-and-validator-selection-on-polkadot/` | `https://polkadot.com/blog/nominating-and-validator-selection-on-polkadot/` |
| `content/9-Polkadot/6-execution-sharding-approvals-and-disputes-slides.md` | 182 | `https://polkadot.network/blog/polkadot-v1-0-sharding-and-economic-security/` | `https://polkadot.com/blog/polkadot-v1-0-sharding-and-economic-security/` |

### Documentation Links

| File | Line | Broken URL | Suggested Replacement |
|------|------|------------|----------------------|
| `content/6-Protocol_On-Chain/Zombienet-slides.md` | 787 | `https://paritytech.github.io/zombienet-sdk/zombienet_orchestrator/network/struct.Network.html#method.add_parachain` | `https://paritytech.github.io/zombienet-sdk/zombienet_sdk/` (general docs) |
| `content/1-Cryptography/materials/zk-factorization-example/circuit_example_playground/snarkjs_README.md` | 508 | `https://blog.iden3.io/first-zk-proof.html` | `https://iden3-docs.readthedocs.io/en/latest/iden3_repos/circom/TUTORIAL.html` |

### GitHub Profile Links

| File | Line | Broken URL | Suggested Replacement |
|------|------|------------|----------------------|
| `content/5-PVM-Polkadot-Architecture-and-Smart-Contracts/3-Platform_Agnostic_Bytecode-slides.md` | 532 | `https://github.com/gabriele-0201` | Remove or replace (account deleted) |
| `content/9-Polkadot/4-state-sharding-collators-and-cumulus-slides.md` | 769 | `https://github.com/gabriele-0201` | `https://it.linkedin.com/in/gabriele-miotti-26ab58218` (LinkedIn profile) |

### Other Dead Links

| File | Line | Broken URL | Suggested Replacement |
|------|------|------------|----------------------|
| `content/1-Cryptography/11-Cryptography_In_Context-slides.md` | 321 | `https://www.bitfinex.com/posts/215` | `https://en.wikipedia.org/wiki/2016_Bitfinex_hack` or `https://www.coindesk.com/markets/2016/08/03/the-bitfinex-bitcoin-hack-what-we-know-and-dont-know` |
| Unknown | - | `https://polkadot-blockchain-academy.github.io/pba-book/blockchain-contracts/_materials/grandpa-board-game.html` | Check if resource exists elsewhere or remove |

---

## 403 Errors (Sites Blocking Automated Requests)

These links work in browsers but block automated link checkers. Added to CI ignore list.

| File | Line | URL | Error |
|------|------|-----|-------|
| `content/3-Blockchain/6-Consensus_Finality-slides.md` | - | `https://medium.com/softblocks/explaining-how-tendermint-consensus-works-433066cbc465` | 403 Forbidden |
| `content/1-Cryptography/*` | - | `https://medium.com/coinmonks/announcing-the-perpetual-powers-of-tau-ceremony-to-benefit-all-zk-snark-projects-c3da86af8377` | 403 Forbidden |
| `content/1-Cryptography/*` | - | `https://ct.cloudflare.com/` | 403 Forbidden |
| `content/2-Economics/2-Game_Theory-slides.md` | 433 | `https://www.opec.org/opec_web/en/` | SSL Certificate Error |
| `content/*` | - | `https://www.sciencedirect.com/science/article/abs/pii/S0305048323000336` | 403 Forbidden |

---

## Summary

- **5 Polkadot blog links**: Domain changed from `polkadot.network` to `polkadot.com`
- **2 Documentation links**: Need updated URLs
- **2 GitHub profile links**: Account `gabriele-0201` deleted
- **2 Other dead links**: Need replacement or removal
- **5 Links blocked by sites**: Added to CI ignore list (work in browsers)
