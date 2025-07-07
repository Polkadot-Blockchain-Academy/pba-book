# Activity: Infinite deck poker using VRFs

As we learnt in class, VRFs are a way of providing private randomness that can later be publicly revealed.

A card game also has randomness, for instance, drawing a random card, which can be kept a secret until the card is played.

We can use a VRF output mod 52 to determine a card.

## Challenge

Your task is to create a poker game or a simplification of one that uses VRFs to determine a players' card.

### Possible simplifications

Card-less poker - There are no hands.
The player with the highest VRF revealed at the end should win the game but with the usual poker bidding rules.

Or we can simplify bidding etc.

## Guidelines

- Players need to have keypairs - their own secret and public keys.
  When a player or players draw a card, we need to choose an input for the VRFs of players who draw cards.
- One good way to get an input is for all players to do a commit-reveal and combine the results, however you could choose whatever technique you'd like.
- Players know their own VRF output (i.e. the cards in their hand), but other players don't until the game calls for them to reveal their card, by publishing a VRF output.

You can choose what VRF library you want to use, though we recommend the [`schnorrkel` Rust crate](https://github.com/w3f/schnorrkel).

## Ways this protocol fails to simulate actual card drawing

There is a complication: normally in card games you can't draw the same card twice or have two people draw the same card.
But imagine we shuffle many decks together then the same card can be drawn twice.
By "Infinite deck", we mean that card counting is useless: we always have a 1/52 probability of drawing a card, no matter who has already drawn what.

Another complication is that if games require cards to be drawn and played at different times, we can tell when a card that is revealed was drawn.
Not a problem for e.g. Poker, when we reveal once of at all.

## Instructions

1. Find some 2-3 other teammates you haven't worked with before.
1. Solve the challenge.

Don't hesitate to ask for help if needed.
The target duration for this activity is about 1.5 hours.
