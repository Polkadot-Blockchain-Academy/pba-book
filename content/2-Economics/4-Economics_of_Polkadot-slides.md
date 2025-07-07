---
title: The Economics of Polkadot
description: Tokenomics of Relay Chains and more
duration: 2 hour
---

# The Economics of Polkadot

Note:

- First: Lecture
- Then we will see the tournament results and discuss

---

## The Economics of Polkadot

<ul>
<li class="fragment">Which economic pieces build the the Polkadot Network?</li>
<li class="fragment">What are their mechanisms and incentives?</li>
<li class="fragment">How are those pieces interrelated?</li>
</ul>

<p class="fragment">Remark: There are currently many changes planned</p>

Note:

- It was a bit challenging to find a good structure for this.
- Because most of the "old" system is still fully in place and important, but a lot is about to change very soon.
- I focus on the system as it is now (also conceptually things do not change) and reference to upcoming changes.

---

<img rounded style="width: 1200px; margin-right: 50px;" src="./img/2.4-polkadot_pieces.svg" />

---

# Token Economics

---

## DOT Token

<!-- FIXME https://github.com/Polkadot-Blockchain-Academy/pba-content/issues/684 to render math for offline use -->

<ul>
<li class="fragment">Native token of the Polkadot network.</li>
<li class="fragment">1 DOT = \(1\mathrm{e}{10}\) Plancks</li>
<li class="fragment">
    Planck = smallest unit of account.
    <ul>
        <li class="fragment">Reference to Planck Length = the smallest possible distance in Physics.</li>
    </ul>
</li>
<li class="fragment">
    <strong>Utility Token</strong> with several use-cases:
    <ul>
        <li class="fragment">Governance (decentralization)</li>
        <li class="fragment">Bonding in slot auctions (utility)</li>
        <li class="fragment">Staking (security)</li>
        <li class="fragment">Message passing (e.g., transfers)</li>
    </ul>
</li>
</ul>

---

## Inflation Model

<ul>
<li class="fragment">Expansion in token supply.</li>
<li class="fragment">Token minted from thin air.
  <ul>
  <li class="fragment">Used to pay staking rewards for validators and nominators.</li>
  <li class="fragment">(Indirectly) fund Treasury.</li>
  </ul>
</li>
<li class="fragment">Central economic variables of the model are:</li>
<li class="fragment"><strong>Exogenous</strong>:
  <ul>
  <li class="fragment">Staking rate (Total amount of staked DOT / Total amount of DOT).</li>
  </ul>
</li>
<li class="fragment"><strong>Endogenous</strong>:
  <ul>
  <li class="fragment">Optimal staking rate (a sufficient backing for validators to provide reasonable security).</li>
  </ul>
</li>
<li class="fragment">Total inflation rate (10%).</li>
</ul>

---

## Inflation Model

<ul>
<li class="fragment">Different <strong>states</strong> of DOT:
  <ul>
  <li class="fragment"><strong>Liquid</strong>: Used for messaging and liquidity on markets.</li>
  <li class="fragment"><strong>Bonded (Staking)</strong>: Economic mass that guarantees the security of the network.</li>
  <li class="fragment"><strong>Bonded (Parachains)</strong>: The demand for DOT tokens by parachains.</li>
  </ul>
</li>
<li class="fragment">The goal is to obtain (some) <strong>sensible ratio</strong> between those three token states.</li>
</ul>

---

## Inflation Model

<div style="display: flex; justify-content: center; align-items: center;">
    <div style="flex: 1; text-align: center;">
        <img rounded style="width: 650px; margin-right: 50px;" src="./img/2.4-current_inflation_model_polkadot.png" />
    </div>
    <div style="flex: 1;">
        <ul>
            <li class="fragment"><strong>Green</strong>: Yearly interest rate in staking, <strong>Blue</strong>: Inflation stakers</li>
            <li class="fragment"><strong>Central variable</strong>: Ideal staking rate (currently ~59.3%).</li>
            <li class="fragment">Highest staking rewards at the ideal staking rate.</li>
            <li class="fragment">Incentives to (increase) decrease the staking rate it is (below) above the optimal.</li>
            <li class="fragment">Staking inefficiencies -> Treasury.</li>
            <li class="fragment">Ideal staking rate scales with number of active parachains (0.5% less with each parachain).</li>
        </ul>
    </div>
</div>

Notes:

- current staking rate 56.7%
- Assume we have 100 token in the whole network. The yearly inflation rate is 10%. Now assume that we have an ideal staking rate of 50% and we actually have 50% of the token staked the whole year (i.e., all inflation goes to stakers). That means, of the 100 token, we have 50 in the staking system. Let’s also make a simplifying assumption that staking rewards are only distributed once after one year.

- At the end of the year, 10 token (100\*0.1) are minted from inflation and distributed to those 50 token (equally). Only those get the rewards, because only they are part of the staking system.

- Let’s say I had 1 token in the staking system at the beginning of the year, then I will have 1.2 token after (because I get 1/50th of the 10 token). As you can see, my staking APY (yield) is 20%.

---

# Adjusted Ideal Rate

<ul>
<li class="fragment">Referendum 166 proposed to change the calculation of the ideal rate.</li>
<li class="fragment">Got accepted by community with overwhelming approval and support.</li>
<li class="fragment">Without it, the ideal rate would be 51.5% now!</li>
<li class="fragment">That would lead to large reduction of staking rewards to stakers.</li>
</ul>

Note:

- before: auction_proportion <- min(auctioned_slots, 60) / 200 , ideal_stake <- 75 / 100 - auction_proportion
- now: auction_proportion <- min(auctioned_slots, 60) / 300 , ideal_stake <- 75 / 100 - auction_proportion

---

## Inflation

<ul>
    <li class="fragment">In the fiat-world, inflation has a negative connotation.</li>
    <li class="fragment">This is a general discussion in economics.</li>
    <li class="fragment">My take on this:</li>
    <ul>
        <li class="fragment">Predictable (maximum) inflation is good.</li>
        <li class="fragment">It incentivizes to work with the tokens (i.e., bond for good parachains, use for message passing).</li>
        <li class="fragment">Deflation can cause a halt of economic activity, because people start hoarding tokens.</li>
    </ul>
    <li class="fragment">Polkadot is paying validators to do their job and be resilient from inflation!</li>

</ul>

---

## Staking: Concept

<ul>
    <li class="fragment"><strong>Nominated Proof-Of-Stake (NPoS)</strong>.</li>
    <li class="fragment">Economic incentives of <strong>validators</strong> and <strong>nominators</strong> are aligned with those of the network.</li>
    <ul>
        <li class="fragment">Good behavior is rewarded with staking rewards.</li>
        <li class="fragment">Malicious / Neglecting behavior is punished (slashed).</li>
    </ul>
    <li class="fragment">Currently, minimum total stake is ~2.4M DOTs.</li>
    <li class="fragment">The total stake in the system directly translates to the <strong>economic security</strong> that it provides.</li>
    <li class="fragment">Total stake is pooled from validators (self-stake) and their nominators (nominated stake)</li>
    <ul>
        <li class="fragment">High degree of inclusion</li>
        <li class="fragment">High security</li>
        <li class="fragment">The goal is to get as much <strong>skin-in-the-game</strong> as possible.</li>
    </ul>
</ul>

---

## Validators

<ul>
    <li class="fragment">What makes Validators resilient:</li>
    <ul>
        <li class="fragment">Self-stake</li>
        <li class="fragment">Reputation (identity)</li>
        <li class="fragment">High future rewards (self-stake + commission)</li>
    </ul>
</ul>

---

## Nominators

<ul>
    <li class="fragment">Bond tokens for up to 16 validators that they deem trustworthy.</li>
    <li class="fragment">They have an incentive to find the best ones that match their preferences.</li>
    <li class="fragment">They are tasked to collectively curate the set of active validators.</li>
</ul>

---

## Rewards

<pba-cols>
<pba-col>
<pba-flex center>

<img rounded style="width: 700px;" src="./img/2.4-rewards_flow.png" />

</pba-flex>
</pba-col>
<pba-col>

<ul>
    <li class="fragment"><em>What are staking rewards for?</em></li>
    <li class="fragment"><strong>Validators</strong>: Hardware, networking, and maintenance costs, <strong>resilience</strong>.</li>
    <li class="fragment"><strong>Nominators</strong>: Curation of the active set of validators, sort out the good from the bad ones (Invisible Hand).</li>
</ul>

</pba-col>
</pba-cols>

Notes:

- Talk about economic security of validators (self-stake + NPV of future rewards (moderated by commission)).

---

## Validator Selection

<ul>
    <li class="fragment">The job of nominators is to find and select suitable validators.</li>
    <li class="fragment">Nominators face several trade-offs when selecting validators:</li>
    <ul>
        <li class="fragment">Security, Performance, Decentralization</li>
        <li class="fragment">Ideally those variables in their historic time-series.</li>
    </ul>
    <li class="fragment">Economic Background:</li>
    <ul>
        <li class="fragment">Self-stake as main indicator of skin-in-the-game.</li>
        <li class="fragment">Higher commission, ceteris paribus, leaves a validator with more incentives to behave.</li>
    </ul>
    <li class="fragment">Various sources of trust</li>
    <li class="fragment">Efficient validator recommendation is one of my research topics.</li>
</ul>

---

# Parachains / Cores

---

## What are Parachains?

<ul>
    <li class="fragment">Parachains (or cores) are the layer-1 part of the protocol.</li>
    <li class="fragment">Blockchains of their own that run in parallel.</li>
    <ul>
        <li class="fragment">Highly domain specific and have high degree of flexibility in their architecture.</li>
        <li class="fragment">Share same messaging standard to be interoperable and exchange messages through the Relay Chain.</li>
    </ul>
    <li class="fragment">Polkadot: 50 Parachains, Kusama: 45 Parachains.</li>
    <li class="fragment">Their state transition function (STF) is registered on the Relay Chain.</li>
    <ul>
        <li class="fragment">Validators can validate state transitions without knowing all the data on the Parachain.</li>
        <li class="fragment">Collators keep the parachain alive (but are not needed for security).</li>
    </ul>
    <li class="fragment">Offer their utility to the network.</li>
</ul>

---

## Parachain Slots

<ul>
    <li class="fragment">The access to the network is abstracted into the notion of “slots”.</li>
    <ul>
        <li class="fragment">Leases for ~2 years on Polkadot (~1 year on Kusama).</li>
        <li class="fragment">Only limited amount of slots available (networking).</li>
        <li class="fragment">The slots are allocated through a candle auction.</li>
    </ul>
    <li class="fragment">Bonded tokens held (trustlessly) in custody on the Relay Chain.</li>
    <li class="fragment">The tokens will be refunded after the slot expires.</li>
</ul>

<p class="fragment">This will change in favor of agile coretime markets likely in August 2024.</p>

---

## Economic Intuition

<ul>
    <li class="fragment">Tokens cannot be used for anything (staking, transacting, liquidity, governance).</li>
    <ul>
        <li class="fragment">That means, tokens locked cause opportunity costs.</li>
        <li class="fragment">An approximation is the trust-free rate of return from staking.</li>
    </ul>
    <li class="fragment">Parachains need to compete with those costs and generate benefits that exceed those opportunity costs.</li>
    <ul>
        <li class="fragment">Sufficient crowdloan rewards.</li>
        <li class="fragment">Sufficient economic activity on-chain that justifies renewal.</li>
    </ul>
    <li class="fragment">Slot mechanism creates constant demand for DOT token.</li>
    <li class="fragment">It is costly to be and remain a parachain.</li>
    <ul>
        <li class="fragment">Natural selection mechanism to select useful parachains.</li>
        <li class="fragment">Continuous pressure to gather funds for extending slots.</li>
    </ul>
</ul>

---

## What do Parachains get?

<ul>
    <li class="fragment"><strong>Parachains pay for security</strong>.</li>
    <ul>
        <li class="fragment">Every parachain is as secure as the Relay Chain.</li>
        <li class="fragment">Polkadot is a security alliance with network effects.</li>
        <li class="fragment">Not only scaling number of transactions, but it also scaling of security.</li>
    </ul>
    <li class="fragment">Security is a pie of limited size, because financial resources are limited.</li>
    <li class="fragment">Every chain that secures itself need cut a piece of the cake, which leaves less to others (zero-sum).</li>
    <li class="fragment">Shared security protocols allow to keep the cake whole and entail it to all participants.</li>
    <li class="fragment">Shared security is a scaling device, because the amount of stake you need to pay stakers to secure 100 shards is less than you need to pay stakers to secure 100 individual chains.</li>
</ul>

---

## Outlook: Polkadot

<ul>
    <li class="fragment">Based on Gav’s Keynote at Polkadot Decoded 2023 and RFC-1.</li>
    <li class="fragment">Blockspace: A current narrative in the Polkadot system.</li>
    <li class="fragment">Polkadot is moving away from regarding Parachains as a distinct entity but rather regard the whole network as global distributed computer.</li>
    <li class="fragment">It's spaces and apps rather than chains.</li>
    <li class="fragment">This computer has computation cores that can be allocated flexible to applications that need it.</li>
    <li class="fragment">Cores can be much easier be acquired, shared, and resold.</li>
</ul>

---

## Core Attributes of Blockspace

<ul>
    <li class="fragment"><strong>Security</strong>: The scarcest resource in blockchain, crucial in preventing consensus faults or 51% attacks that could compromise transactions.</li>
    <li class="fragment"><strong>Availability</strong>: Ensuring blockspace is available without long waiting times or uncertain costs for a smooth, seamless interaction within the decentralized ecosystem.</li>
    <li class="fragment"><strong>Flexibility</strong>: The ability of blockspace to be fine-tuned by the consumer for specific use-cases.</li>
</ul>

---

## Blockspace Ecosystem

<ul>
    <li class="fragment">A networked collection of individual blockspace producers (blockchains) offering secured, fit-for-purpose, efficiently-allocated, and cost-effective blockspace.</li>
    <li class="fragment">A valuable aspect of a blockspace ecosystem is its connective tissue of shared security and composability.</li>
    <li class="fragment">Dapp developers or blockspace providers can focus on their unique features, reusing existing capabilities within the ecosystem.</li>
    <li class="fragment">For example, a supply chain traceability application could use different types of blockspace for identity verification, asset tokenization, and source traceability.</li>
</ul>

---

## Agile Coretime

<ul>
    <li class="fragment">The final design is not yet determined.</li>
    <ul>
        <li class="fragment">Cores are sold for 4 weeks as NFT by a broker.</li>
        <li class="fragment">Unrented cores go to the instantaneous market.</li>
        <li class="fragment">Price de-/increases relative to demand.</li>
        <ul>
            <li class="fragment">Dutch auction.</li>
        </ul>
        <li class="fragment">Current tenants have a priority buy right for their core(s).</li>
        <ul>
            <li class="fragment">One goal is to offer price predictability for existing teams.</li>
            <li class="fragment">Challenging Task.</li>
        </ul>
    </ul>
</ul>

---

## Agile Coretime

<ul>
    <li class="fragment">The first three cores are already being sold on Kusama</li>
    <ul>
        <li class="fragment">First renewals are already happening.</li>
        <li class="fragment">There is some discussion about adjusting the market structure.</li>
        <li class="fragment">In RFC17, I proposed some design for the market.</li>
        <li class="fragment">Launch on Polkadot in August 2024!</li>
    </ul>
</ul>

---

## RFC 10

<ul>
    <li class="fragment">Proposed to burn coretime revenues.</li>
    <li class="fragment">Got accepted by the fellowship.</li>
    <li class="fragment"><strong>Clear incentives:</strong> Coretime is clearly a cost.</li>
    <ul>
        <li class="fragment">Some actors might asymmetrically benefit from Treasury.</li>
    </ul>
    <li class="fragment"><strong>Balancing Inflation:</strong> Deflationary pressure in the system.</li>
    <ul>
        <li class="fragment">Initially rather low.</li>
    </ul>
    <li class="fragment"><strong>Collective Value Accrual:</strong></li>
    <ul>
        <li class="fragment">Burning one DOT benefits all other DOTs equally.</li>
    </ul>
</ul>

---

## Why Agile Coretime?

<ul>
    <li class="fragment">This allows for low barriers of entry for people to simply deploy their code to a core and test stuff</li>
    <li class="fragment">It makes blockspace more efficient, because not all teams can/want to have a full block every 6/12 seconds.</li>
    <li class="fragment">Secondary markets increase efficiency and improve resource allocation.</li>

</ul>

---

# Treasury

<ul>
    <li class="fragment">The treasury is an on-chain fund that holds DOT token and is governed by all token holders of the network.</li>
    <li class="fragment">Those funds come from:</li>
    <ul>
        <li class="fragment">Transactions</li>
        <li class="fragment">Slashes</li>
        <li class="fragment">Staking inefficiencies (deviations from optimal staking rate)</li>
    </ul>
    <li class="fragment">Through governance, everybody can submit proposals to initiate treasury spending.</li>
    <li class="fragment">It currently holds around 28M DOT.</li>
    <li class="fragment">Spending is incentivized by a burn mechanism (1% every 24 days).</li>
</ul>

---

## Treasury as DAO

<ul>
    <li class="fragment">A DAO (decentralized autonomous organization) that has access to funds and can make funding decisions directed by the collective (that have vested interest in the network).</li>
    <li class="fragment">This has huge potential that might not yet have been fully recognized by the people.</li>
    <li class="fragment">This provides the chain the power to fund its own existence and improves the utility in the future. It will pay…</li>
    <ul>
        <li class="fragment">… <strong>core developers</strong> to improve the protocol (fellowship).</li>
        <li class="fragment">… <strong>researchers</strong> to explore new directions, solve problems and conduct studies that are beneficial for the network.</li>
        <li class="fragment">… for campaigns <strong>educating people</strong> about the protocol.</li>
        <li class="fragment">… for <strong>systems-parachains</strong> (development & collators).</li>
    </ul>
    <li class="fragment">A truly decentralized and self-sustaining organization.</li>
</ul>

---

# How does it all fit together?

---

<div style="display: flex; justify-content: center; align-items: center;">
    <img src="./img/Polkadot-Economics-Cycle.svg" style="width: 1200px;" />
</div>

---

## Takeaways

<ul>
    <li class="fragment">Polkadot is a system that offers shared security and cross-chain messaging.</li>
    <li class="fragment">Security scales, i.e., it takes less stake to secure 100 parachains than 100 individual chains.</li>
    <li class="fragment">The DOT token captures the utility that the parachains provide and converts it to security.</li>
    <li class="fragment">The slot mechanics (renewal, auctions) creates a market where parachains need to outcompete opportunity costs to be sustainable (i.e., they need to be useful).</li>
    <li class="fragment">Polkadot is a DAO that will be able to fund its own preservation and evolution.</li>
    <li class="fragment">There are many changes to come to Polkadot creating a much more agile system.</li>
</ul>

---

# Potential Changes to the Economics

---

## Independent Treasury Inflow

<ul>
    <li class="fragment">The current system incentivizes to move the staking rate to the ideal rate.</li>
    <li class="fragment">Then, Treasury inflow would be 0 DOT.</li>
    <li class="fragment">That is not sustainable.</li>
    <li class="fragment">Proposed change: Detach inflation to stakers from total inflation and divert the rest to Treasury directly.</li>
</ul>

---

## Fixed Ideal Staking Rate

<ul>
    <li class="fragment">Ideal staking rate scales with number of parachains.</li>
    <li class="fragment">The notion of parachain in its original form is being deprecated.</li>
    <li class="fragment">It might be much more reasonable to aim for a static ideal staking rate. </li>
</ul>

---

## Lowered inflation?

<ul>
    <li class="fragment">Voices become louder that demand lowering inflation.</li>
    <li class="fragment">This will be a highly debated topic.</li>
    <li class="fragment">Inflation is used to pay validators and nominators to do their job.</li>
    <li class="fragment">Interesting how many people "wake up" to the debate trying to block lowering of inflation.</li>
</ul>

Notes:

- Validator rewards are a major contributor to the economic security of Polkadot.
- Lowering inflation leads to lowered rewards.
- We could increase minimum commission, but that would further disadvantage nominators.

---

## Further Resources

- [Agile Coretime RFC](https://github.com/polkadot-fellows/RFCs/pull/1)
- [Updates on Coretime Sales](https://forum.polkadot.network/t/agile-coretime-launch-status-report/8347)
- [Discussion on Changing Inflation Model](https://forum.polkadot.network/t/adjusting-the-current-inflation-model-to-sustain-treasury-inflow/3301)
- [Talk about Agile Polkadot](https://www.youtube.com/watch?v=GIB1WeVuJD0)
- [Nominating and Validator Selection On Polkadot](https://polkadot.network/blog/nominating-and-validator-selection-on-polkadot/)
- [(Journal Version) Paper on Validator Selection](https://www.sciencedirect.com/science/article/abs/pii/S0305048323000336)
- [(Open Source Version) Paper on Validator Selection)](https://papers.ssrn.com/sol3/papers.cfm?abstract_id=4253515)
- [Referendum 166](https://polkadot.polkassembly.io/referenda/166)
- [Discussing on adjusting ideal rate](https://forum.polkadot.network/t/adjusting-polkadots-ideal-staking-rate-calculation/3897)
- [RFC17](https://github.com/polkadot-fellows/RFCs/pull/17)
