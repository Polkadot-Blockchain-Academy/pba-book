---
title: Parachain XCM Configuration # Also update the h1 header on the first slide to the same name
description: XCM configuration overview and considerations, for parachains.
duration: 1 hour
---

# Parachain XCM Configuration

---v

## _At the end of this lecture, you will be able to:_

<pba-flex center>

- Understand the different XCM configurable parts of a chain
- Construct different XCM configurations for chains with different needs

---

## 🛠️ Configurables in `XcmConfig`

Notes:

The XCM Configuration has many configurable items

EXERCISE: ask the class to raise hands and postulate on what they think should be configurable.

---v

## 🛠️ Configurables in `XcmConfig`

```rust [1-2|6-7|8-9|10-11|12-13|14-15|16-31]
// How we convert locations into account ids
type SovereignAccountOf = SovereignAccountOf;

pub struct XcmConfig;
impl Config for XcmConfig {
  // The absolute Location of the current system
  type UniversalLocation = UniversalLocation;
  // Pre-execution filters
  type Barrier = Barrier;
  // How we withdraw/deposit assets
  type AssetTransactor = LocalAssetTransactor;
  // How we convert a Location to a FRAME dispatch origin
  type OriginConverter = LocalOriginConverter;
  // How we route the XCM outside this chain
  type XcmSender = XcmRouter;
  // Who we trust as reserve chains
  type IsReserve = ?;
  // Who do we trust as teleporters
  type IsTeleporter = ?;
  // How we weigh a message
  type Weigher = ?;
  // How we charge for fees
  type Trader = ?;
  // How we handle responses
  type ResponseHandler = ?;
  // How we handle asset traps
  type AssetTrap = ?;
  // How we handle asset claims
  type AssetClaims = ?;
  // How we handle version subscriptions
  type SubscriptionService = ?;
}
```

Notes:

- `SovereignAccountOf`: Means of converting a `Location` into an account ID
  Used later for: `OriginConverter` , `AssetTransactor`

- `xcm-pallet` is a pallet that not only allows sending and executing XCM messages, but rather it also implements several of the configuration traits and thus can be used perform several XCM configuration actions.

---v

## 🛠️ `xcm-builder`

`xcm-builder` is a crate containing common configuration shims to facilitate XCM configuration.

Most pre-built configuration items can be found in `xcm-builder`.

It allows to use the XCM executor in FRAME.

---

### 🤔 Grab your chain's requirements before starting

Questions that you should have answers for:

- _Is my chain going to transfer just the native token?_
  _Is my chain going to receive several other kinds of assets?_

- _Is my chain going to allow free execution?_
  _Maybe only limited to some parachains/relay chain?_

- _Is my chain a 20 byte account chain?_
  _a 32 byte account chain?_

- _How will my chain accept fee payment?_
  _In one asset?_
  _In several?_

Notes:

- Some of the answers to these questions might imply you need to use your own custom primitives.

---v

### Our starting example setup requirements

1. Parachain that does not charge for relay incoming messages.
2. Parachain that trusts the relay as the reserve chain for the relay chain tokens.
3. Parachain that mints in `pallet-balances` when it receives relay chain tokens.
4. Users can execute XCMs locally.

---

### 📁 `SovereignAccountOf` via `xcm-builder`

- Defines how we convert a `Location` into a local account ID.
- Useful when we want to withdraw/deposit tokens from a `Location` defined origin
- Useful when we want to dispatch as signed origins from a `Location` defined origin.

<diagram class="mermaid">
graph TD;
  Location("AccountId32 { id: [18, 52, ..., 205, 239], network: Some(Rococo) }")-- SovereignAccountOf -->Account("0x123..def (Alice)")
</diagram>

Notes:

- This will define how we convert a `Location` into a local account ID.
- This is useful when we want to withdraw/deposit tokens from a `Location` defined origin or when we want to dispatch as signed origins from a `Location` defined origin.

---v

### 📁 `SovereignAccountOf` via `xcm-builder`

- `HashedDescription`: Hashes the description of a `Location` and converts that into an `AccountId`.

```rust
pub struct HashedDescription<AccountId, Describe>(PhantomData<(AccountId, Describe)>);
impl<
  AccountId: From<[u8; 32]> + Clone,
  Describe: DescribeLocation
> ConvertLocation<AccountId> for HashedDescription<AccountId, Describe>
{
	fn convert_location(value: &Location) -> Option<AccountId> {
		Some(blake2_256(&Describe::describe_location(value)?).into())
	}
}
```

---v

### 📁 `SovereignAccountOf` via `xcm-builder`

- `HashedDescription`. An example of a converter definition:

<pba-flex center>

```rust
pub type LocationToAccount =
  HashedDescription<AccountId, (
    LegacyDescribeForeignChainAccount, // Legacy conversion - MUST BE FIRST!
    DescribeTerminus,
    DescribePalletTerminal
  )>;
```

---v

### 📁 `SovereignAccountOf` via `xcm-builder`

- `DescribeLocation`: Means of converting a location into a stable and unique descriptive identifier.

```rust
pub trait DescribeLocation {
	/// Create a description of the given `location` if possible. No two locations should have the
	/// same descriptor.
	fn describe_location(location: &Location) -> Option<Vec<u8>>;
}
```

Notes:

[Impl for Tuple](https://github.com/paritytech/polkadot-sdk/blob/342d720/polkadot/xcm/xcm-builder/src/location_conversion.rs#L34)

---v

### 📁 `SovereignAccountOf` via `xcm-builder`

- `DescribeAccountId32Terminal`

```rust
fn describe_location(l: &Location) -> Option<Vec<u8>> {
	match (l.parents, &l.interior) {
		(0, X1(AccountId32 { id, .. })) => Some((b"AccountId32", id).encode()),
		_ => return None,
	}
}
```

---v

### 📁 `SovereignAccountOf` via `xcm-builder`

- `DescribeTerminus`

```rust
fn describe_location(l: &Location) -> Option<Vec<u8>> {
	match (l.parents, &l.interior) {
		(0, Here) => Some(Vec::new()),
		_ => return None,
	}
}
```

---v

### 📁 `SovereignAccountOf` via `xcm-builder`

- `DescribePalletTerminal`

```rust
fn describe_location(l: &Location) -> Option<Vec<u8>> {
	match (l.parents, &l.interior) {
		(0, X1(PalletInstance(i))) =>
			Some((b"Pallet", Compact::<u32>::from(*i as u32)).encode()),
		_ => return None,
	}
}
```

---v

### 📁 `SovereignAccountOf` via `xcm-builder`

- `DescribeAccountKey20Terminal`

```rust
fn describe_location(l: &Location) -> Option<Vec<u8>> {
	match (l.parents, &l.interior) {
		(0, X1(AccountKey20 { key, .. })) => Some((b"AccountKey20", key).encode()),
		_ => return None,
	}
}
```

---v

### 📁 `SovereignAccountOf` via `xcm-builder`

- `AccountId32Aliases`: Converts a local `AccountId32` `Location` into an account ID of 32 bytes.

- `Account32Hash`: Hashes the `Location` and takes the lowest 32 bytes as account.

- `ParentIsPreset`: Converts the parent `Location` into an account of the form `b'Parent' + trailing 0s`

---v

### 📁 `SovereignAccountOf` via `xcm-builder`

- `ChildParachainConvertsVia`: Converts the **child** parachain `Location` into an account of the form `b'para' + para_id_as_u32 + trailing 0s`

- `SiblingParachainConvertsVia`: Convert the **sibling** parachain `Location` into an account of the form `b'sibl' + para_id_as_u32 + trailing 0s`

---

### `UniversalLocation`

The absolute location of the consensus system being configured.

<pba-flex center>

```rust
parameter_types! {
  pub const UniversalLocation: InteriorLocation = GlobalConsensus(NetworkId::Polkadot).into();
}
```

---

### 🚧 `Barrier` via `xcm-builder`

- Barriers specify whether or not an XCM is allowed to be executed on the local consensus system.
- They are checked before the actual XCM instruction execution.
- **Barriers should not involve any heavy computation.**

Notes:

**At the point at which barriers are checked nothing has yet been paid for its execution**.

---v

### 🚧 `Barrier` via `xcm-builder`

Physical vs Computed origin

- Physical origin: the consensus system that built this particular XCM and sent it to the recipient
- Computed origin: the entity that ultimately instructed the consensus system to build the XCM

Notes:

If an EOA transfers some funds via XCM, then the computed origin would be its account, but the physical origin would be the platform that was used (e.g. parachain).

---v

### 🚧 `Barrier` via `xcm-builder`

Barriers that operate upon **computed origins** must be put inside of `WithComputedOrigin`.

Allows for origin altering instructions at the start.

<pba-flex center>

```rust
pub struct WithComputedOrigin<InnerBarrier, LocalUniversal, MaxPrefixes>;
```

---v

### 🚧 `Barrier` via `xcm-builder`

- `TakeWeightCredit`: Subtracts the maximum weight the message can consume from the available weight credit.
  Usually configured for local `xcm-execution`

---v

### 🚧 `Barrier` via `xcm-builder`

- `AllowTopLevelPaidExecutionFrom<T>`: For origins contained in `T`, it makes sure the first instruction puts asset into the holding register, followed by a `BuyExecution` instruction capable of buying sufficient weight.
  **Critical to avoid free DOS**.

Notes:

- A chain without `AllowTopLevelPaidExecutionFrom` could potentially receive several heavy-computation instructions without paying for it.
  Checking that the first instructions are indeed paying for execution helps to quick-discard them.

- While `BuyExecution` is crucial for messages coming from other consensus systems, local XCM execution fees are paid as any other substrate extrinsic.

---v

### 🚧 `Barrier` via `xcm-builder`

- `AllowExplicitUnpaidExecutionFrom<T>`: Allows free execution if `origin` is contained in `T` and the first instruction is `UnpaidExecution`.

Notes:

- **This fulfills our requirements**
- To meet our example use case, we only need the relay to have free execution.

---v

### 🚧 `Barrier` via `xcm-builder`

- `AllowKnownQueryResponses`: Allows the execution of the message if it contains only an expected `QueryResponse`
- `AllowSubscriptionsFrom<T>`: If the `origin` that sent the message is contained in `T`, it allows the execution of the message if it contains only a `SubscribeVersion` or `UnsubscribeVersion` instruction

---

### 🪙 `AssetTransactor` via `xcm-builder`

- Define how we are going to withdraw and deposit assets
- Heavily dependant on the assets we want our chain to transfer

<diagram class="mermaid">
graph LR
  Withdraw("WithdrawAsset((Here, 100u128).into())")-->DOT(100 tokens from e.g. pallet-balances)
</diagram>

Notes:

- The relay chain is a clear example of a chain that handles a **single token**.
- AssetHub on the contrary acts as an asset-reserve chain, and it needs to handle **several assets**

---v

### 🪙 `AssetTransactor` via `xcm-builder`

- `FungiblesAdapter`: Used for depositing/withdrawing from a set of defined fungible tokens.
  An example of these would be `pallet-assets` tokens.
- `NonFungiblesAdapter`: Used for depositing/withdrowing NFTs. For example `pallet-nfts`.

Notes:

- **Matcher**: Matches the `Asset` against some filters and returns the amount to be deposited/withdrawn
- **AccountIdConverter**: Means of converting a `Location` into an account

- For our example, it suffices to uses `CurrencyAdapter`, as all we are going to do is mint in a single currency (Balances) whenever we receive the relay token.

---v

### 🪙 `AssetTransactor` via `xcm-builder`

<pba-flex center>

```rust
fn withdraw_asset(
	what: &Asset,
	who: &Location,
	_maybe_context: Option<&XcmContext>,
) -> result::Result<xcm_executor::Assets, XcmError> {
	let (asset_id, amount) = Matcher::matches_fungibles(what)?;
	let who = AccountIdConverter::convert_location(who)
		.ok_or(MatchError::AccountIdConversionFailed)?;
	Assets::burn_from(asset_id, &who, amount, Exact, Polite)
		.map_err(|e| XcmError::FailedToTransactAsset(e.into()))?;
	Ok(what.clone().into())
}
```

---v

### 🪙 `AssetTransactor` via `xcm-builder`

<pba-flex center>

```rust
fn deposit_asset(
  what: &Asset,
  who: &Location,
  _context: &XcmContext
) -> XcmResult {
	let (asset_id, amount) = Matcher::matches_fungibles(what)?;
	let who = AccountIdConverter::convert_location(who)
		.ok_or(MatchError::AccountIdConversionFailed)?;
	Assets::mint_into(asset_id, &who, amount)
		.map_err(|e| XcmError::FailedToTransactAsset(e.into()))?;
	Ok(())
}
```

---

### 📍 `OriginConverter` via `xcm-builder`

- Defines how to convert an XCM origin, defined by a `Location`, into a frame dispatch origin.
- Used in the `Transact` instruction.

Notes:

- `Transact` needs to dispatch from a frame dispatch origin.
  However the `xcm-executor` works with XCM origins which are defined by `Location`s.
- `OriginConverter` is the component that converts one into the other

---v

### 📍 List of origin converters

- `SovereignSignedViaLocation`: Converts the `Location` origin (typically, a parachain origin) into a signed origin.

- `SignedAccountId32AsNative`: Converts a local 32 byte account `Location` into a signed origin using the same 32 byte account.

- `ParentAsSuperuser`: Converts the parent origin into the root origin.

- `SignedAccountKey20AsNative`: Converts a local 20 byte account `Location` into a signed origin using the same 20 byte account.

Notes:

- `ParentAsSuperuser` can be used in common-good chains as they do not have a local root origin and instead allow the relay chain root origin to act as the root origin.

---

## 🛠️ XcmRouter in `XcmConfig`

- `ParentAsUmp` routes XCM to relay chain through UMP.
- `XcmpQueue` routes XCM to other parachains through XCMP.

<pba-flex center>

```rust
pub type XcmRouter = (
	// Two routers - use UMP to communicate with the relay chain:
	cumulus_primitives_utility::ParentAsUmp<ParachainSystem, PolkadotXcm>,
	// ..and XCMP to communicate with the sibling chains.
	XcmpQueue,
);
```

Notes:

- `ParachainSystem` is a pallet in cumulus that handles incoming DMP messages and queues, among other miscellaneous parachain-related matters.
- If the destination location matches the form of `Location { parents: 1, interior: Here }`, the message will be routed through UMP.
  The UMP channel is available by default.
- If the destination matches the form of `Location { parents: 1, interior: X1(Parachain(para_id)) }`, the message will be routed through XCMP.
  As of today, an HRMP channel should be established before the message can be routed.
- The tuple implementation of this item means the executor will try using the items in order.

---v

## Router

```rust
pub trait SendXcm {
  type Ticket;

  fn validate(
    destination: &mut Option<Location>,
    message: &mut Option<Xcm<()>>,
  ) -> SendResult<Self::Ticket>;

  fn deliver(ticket: Self::Ticket) -> Result<XcmHash, SendError>;
}
```

Notes:

It's important to validate that the message can indeed be sent before sending it.
This ensures you pay for sending fees and you actually do send it.

---

## Summary

In this lecture, we learnt:

- How chains interpret locations and turn them to accounts and FRAME origins
- How to set a barrier to protect our chain from undesired messages
- How to handle assets in XCM
- Other configuration items relevant for XCM
