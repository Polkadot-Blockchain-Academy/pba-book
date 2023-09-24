---
title: OpenGov
description: The Polkadot ecosystem on-chain governance solution
duration: 1 hour
---

# OpenGov

Notes:

Hello!

I'm Bradley Olson

Was student at first Academy in Cambridge

Currently on Parachains Core Team at Parity

Making Polkadot truly decentralized requires a robust, agile, and democratic system of governance.

Gavin has put a lot of effort over the last year or so into crafting a system which does those words justice, OpenGov.

I'm here to give you an overview of what OpenGov is and to surface some new information about how it is performing on Kusama.

So lets get to it

---

## Overview

<pba-flex center>

- Why blockchain governance?
- Why on-chain?
- Goals of on-chain governance
- Initial Solution, Governance V1
- Improvement, OpenGov
- How is it going? By the numbers.
- OpenGov and you

</pba-flex>

---

## Reasons for Blockchain Governance

<pba-flex center>

- Software as executive branch
  - Applies existing laws (code) in pre-defined ways
  - Protocol security ensures the letter of those laws is followed
- But evolving protocols need a legislative branch
  - To update the laws (code)
  - To rectify cases where letter != spirit (bugs)
  - To trigger parts of the system that aren't on a set schedule
  - To spend treasury funds
- Legeslative can be on or off chain

</pba-flex>

---

## Why On-chain?

<pba-cols style="font-size:smaller">
<pba-col center>

- Off-chain governance

  - Formal proposal by dev team
  - Discussions, debates, and media campaigns
  - Hard fork

- Issues
  - Centralization
  - Low throughput
  - Long decision period
  - Little accessibility

</pba-col>
<pba-col center>

<!-- set height*width in px, where full screen is 1920*1080 -->
<img rounded style="width: 300px" src="./img/Bitcoin.png" />

<!-- set height*width in px, where full screen is 1920*1080 -->
<img rounded style="width: 200px" src="./img/Ethereum.png" />

</pba-col>
</pba-cols>

---

## Goals of On-chain Governance

<pba-flex center>

- **Transparency**: Decisions by who and by what rules?
- **Decentralization**: Distributed power, weighted only by commitment and conviction
- **Security**: Harmful proposals don't pass or have limited scope
- **Accessibility**: Easy to draft proposal, to receive vote, to vote yourself, and to vote by proxy
- **Concurrency**: Maximize simultaneous referenda as security allows
- **Speed**: Each referendum completed as fast as security allows
- **Agility**: Speed is responsive to support/controversy

</pba-flex>

---

## Governance V1

<pba-cols>
<pba-col center>

<!-- set height*width in px, where full screen is 1920*1080 -->
<img rounded style="width: 300px" src="./img/Polkadot.png" />

</pba-col>
<pba-col center style="font-size:smaller">

- Tri-cameral system: Referenda, council, and technical committee
- Single track
- 1 referendum at a time
- Root origin (Unlimited Power!)
- 28 day referendum
- 1 month minimum enactment period
- Emergency handled technical committee
- Cancellations by council and technical committee
- Most proposals initiated by council
- Fully council controlled roles such as tipping

</pba-col>
</pba-cols>

---

## Gov V1, Room for Improvement

<pba-cols>
<pba-col left>

The good:

- Security
- Transparency

The bad:

- Decentralization
- Concurrency
- Speed
- Agility

</pba-col>
<pba-col center>

<!-- set height*width in px, where full screen is 1920*1080 -->
<img rounded style="width: 500px" src="./img/Snail.jpeg" />

</pba-col>
</pba-cols>

---

## OpenGov Overview

<pba-cols>
<pba-col center>

- Origins and tracks
- Lifecycle of a referendum
- Support and approval threshold curves
- The Polkadot Fellowship
- Vote delegation by track
- OpenGov and governance goals

</pba-col>
<pba-col center>

<!-- set height*width in px, where full screen is 1920*1080 -->
<img rounded style="width: 500px" src="./img/overview.png" />

</pba-col>
</pba-cols>

---

## Origins

<pba-flex center>

- Level of privilege that code executes with
- Similar to user on Unix
- Proposal is two things
  - Operation: What should happen
  - Origin: Who authorizes it
- Many operations require a specific origin

</pba-flex>

---

<!-- set height*width in px, where full screen is 1920*1080 -->
<img rounded style="width: 300px" src="./img/rail_road_tracks.jpeg" />

## Origins and Tracks

- Each origin is served by a referendum track
- A track can serve more than one origin
- These tracks are totally independent from one another
- Track examples: Root, ParachainAdmin, BigSpender, Tipper
- Emergency tracks: ReferendumCanceler, ReferendumKiller

---

## Track Parameters

<pba-cols>
<pba-col>

Parameters give us the ability to find an optimal balance between security and throughput.

The security needs of the Tipper track are very different than those of the Root track.

</pba-col>
<pba-col>
<pba-flex center>

- Lead-in period duration
- Decision period duration
- Confirmation period duration
- Minimum enactment period
- Concurrency, how many referenda can run in this track at a time
- Support and Approval threshold curves

</pba-flex>
</pba-col>
</pba-cols>

---

<pba-cols>
<pba-col>

### OpenGov Tracks

</pba-col>
<pba-col>

<img rounded style="width: 600px" src="./img/TracksTable.webp" />

</pba-col>
</pba-cols>

Notes:

Highlight difference between parameters of WhiteListedCaller and Root tracks

---

## Criteria for Passing a Proposal

- Approval: Approving votes/total votes cast, weighted by conviction
  - Conviction: Locking tokens for longer periods scales their voting impact up to a maximum of 6x with a lockup duration of 896 days
- Support: Approving votes/total possible vote pool, disregarding conviction

---

## Decision and Confirmation Periods

- If Approval and Support thresholds met, confirmation period begins
- Approval and Support must remain above respective thresholds for entire confirmation period
- Confirmation period concludes -> proposal approved early
- Decision period expires -> proposal rejected
- There is only one decision period, during which a proposal can potentially enter and leave many confirmation periods if thresholds aren't consistently met

---

## Lifecycle of A Referendum

<!-- set height*width in px, where full screen is 1920*1080 -->
<img rounded style="width: 1000px" src="./img/lifecycle_of_a_referendum.png" />

Notes:

Steps in order: **Proposing, Lead In, Deciding, Confirming, Enactment**

---

## Support and Approval Threshold Curves

<pba-flex center>

- We want agility
  - Well supported proposals pass quickly
  - Controversial proposals get more deliberation
- Addressed with time varying curves
  - Support threshold
    - Starts at ~50%
    - Ends at minimum secure turnout for track <br />(EX: Big Spender ends at 0 + epsilon %)
  - Approval threshold
    - Starts at 100%
    - Ends at 50 + epsilon %
- Monotonically decreasing at rates determined by track specific security needs

</pab-flex>

---

## Example Support and Approval Curves

<img rounded style="width: 1400px" src="./img/support_and_approval_curves.png" />

Notes:

From PolkaWorld Article in Resources

---

## Vote Delegation

<pba-cols style="font-size:smaller">
<pba-col center>

- Traditional delegation: You entrust one third party with your vote on all matters
- Delegation by track: You may delegate your vote to one or more third parties on a per track basis
- EX: Tipper vote delegated to local ambassador, WhiteListedCaller vote delegated to Parity Technologies, vote retained for all other tracks
- This is likely a first!

</pba-col>
<pba-col center>

<img rounded style="width: 500px" src="./img/vote.jpeg" />

</pba-col>
</pba-cols>

---

<img rounded style="width: 400px" src="../async-backing-deep/img/stopwatch.png" />

## OpenGov Acting Under Pressure

Typical path to safety: Lower throughput and restricted origins

But in emergencies we may need to pass proposals that both require root origin and are time critical!

Solution: Some sort of oracle capable of providing expert information

---

## Oraclization of Expert Information

<pba-flex center>

1. Track everyone's level of expertise
1. Allow experts to register sentiment
1. Aggregate opinions by level of expertise

</pba-flex>

> But how are these steps accomplished?

---

<!-- .slide: data-background-color="#4A2439" -->

## Enter...<br /><br />The Polkadot Fellowship

---

Purely on-chain membership body to recognize and compensate all individuals who hold and use expert knowledge of Polkadot in line with its broad interest and philosophy

Members hold rank denoting proven level of expertise and commitment as recognized by their peers and, for higher ranks, through general referendum.

---

## Who Make up the Fellowship?

<pba-col center>

- Experts in the Polkadot core protocol who maintain a consistent level of active contribution
- Notably this does not include core developers of independent parachain protocols, which should develop their own protocol specific fellowships as needed.
- Trajectory
  - Currently: < 100 core developers, mostly from Parity or the Web3 Foundation
  - Next year or two: Hundreds
  - Ideal far future: Tens of thousands, independent of any centralized entity
- Only one fellowship for Polkadot and Kusama

</pba-col>

---

## Function of the Fellowship

<pba-col center>

- WhiteListedCaller track
  - Root privileges
  - More agile
  - Maintains reasonable safety via Fellowship
- White list proposals must pass two votes
  - Expertise weighted Fellowship vote via second referendum pallet instantiation
  - Same general referendum as other tracks, still requiring majority vote from DOT holders
- Just an oracle!
- Secondarily intended to cultivate a long term base of Polkadot core developers outside of Parity

</pba-col>

Notes:

Stress that as an oracle, the Fellowship can't take any action on its own. Any white listed call will still require substantial DOT-holder backing.

---

## OpenGov and Governance Goals

<pba-col center>

- Open source + single process + track abstraction -> Transparency

<!-- .element: class="fragment" data-fragment-index="1" -->

- Liberal proposal creation + greater throughput + per-track delegation -> Accessibility

<!-- .element: class="fragment" data-fragment-index="2" -->

- Accessibility + No special bodies -> Decentralization

<!-- .element: class="fragment" data-fragment-index="3" -->

- Limited origins + emergency tracks + white list -> Security

<!-- .element: class="fragment" data-fragment-index="4" -->

- Multiple tracks + low risk tracks -> Concurrency

<!-- .element: class="fragment" data-fragment-index="5" -->

- Low risk tracks + early confirmation -> Speed

<!-- .element: class="fragment" data-fragment-index="6" -->

- Support and approval threshold curves + white list -> Agility

<!-- .element: class="fragment" data-fragment-index="7" -->

</pba-col>

---

<!-- .slide: data-background-color="#000" -->

# OpenGov<br /><br />By The Numbers

---

## Governance Activity

<!-- set height*width in px, where full screen is 1920*1080 -->
<img rounded style="width: 900px" src="./img/proposals_per_day.png" />

> 5.5x more daily governance activity

---

## Proposal Origins

<!-- set height*width in px, where full screen is 1920*1080 -->
<img rounded style="width: 900px" src="./img/proposals_from_democracy.png" />

> Proposals now primarily authored via democracy

---

## Treasury Usage

<img rounded style="width: 900px" src="./img/treasury_waste_since_opengov.png" />

> Treasury funds used more efficiently

---

## OpenGov and You

<pba-col center>

- Participate in OpenGov and Polkadot Fellowship on Polkadot and Kusama
- Can customize OpenGov instances per parachain
- Custom fellowships per parachain
- Potentially create non-technical fellowships, such as a fellowship for brand ambassadors

</pba-col>

---

## Resources

<pba-col center>

1. [PolkaWorld Hands-On OpenGov](https://polkaworld.medium.com/a-hands-on-guide-for-kusamas-open-gov-governance-system-98277629b0c5)
1. [OpenGov Article from Moonbeam Team](https://moonbeam.network/blog/opengov/)
1. [Gavin’s Polkadot Decoded 2022 talk](https://www.youtube.com/watch?v=EF93ZM_P_Oc)
1. [Gov (OpenGov and V1) tracking](https://polkadot.polkassembly.io/opengov)
1. [OpenGov tracking](https://kusama.subsquare.io/)

</pba-col>

---

<!-- .slide: data-background-color="#4A2439" -->

# Questions

---

## OpenGov in Action

<pba-flex center>

- Vote - <https://kusama.subsquare.io/referenda>
- Delegate vote - <https://kusama.subsquare.io/referenda>
- Submit proposal - <https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkusama-rpc.dwellir.com#/referenda>

</pba-flex>
