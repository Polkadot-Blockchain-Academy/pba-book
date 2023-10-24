---
title: XCM in Polkadot # Also update the h1 header on the first slide to the same name
description: XCM in the Polkadot Context for web3 builders
duration: 1 hour
---

# XCM in Polkadot

---

## _At the end of this lecture, you will be able to:_

<pba-flex center>

- Understand the configuration of the Rococo chain
- Send real-world messages between parachain A <-> Rococo
- Identify potential errors on XCM messages

---

## 🤔 Considerations

- There should be no trust assumption between chains unless explicitly requested.
- We cannot assume chains will not act maliciously
- Spamming XCM messages creates a DoS problem

---

## 🛠️ Rococo Configuration

- Barriers
- Teleport filtering
- Trusted reserves
- Asset transactors
- Fee payment
- Proper XCM Instruction Weighting
- Location to Account/FRAME Origin conversions

Notes:

From now on, we will use the Rococo runtime as a reference.
Rococo is a testnet for
Polkadot and Kusama that we will use in to test our XCM messages.
Most of the Rococo configuration
is identical to that in Polkadot.

---

## 🚧 XCM `Barrier` in Rococo

```rust
/// The barriers one of which must be passed for an XCM message to be executed.
pub type Barrier = (
  // Weight that is paid for may be consumed.
  TakeWeightCredit,
  // If the message is one that immediately attempts to pay for execution, then allow it.
  AllowTopLevelPaidExecutionFrom<Everything>,
  // Messages coming from system parachains need not pay for execution.
  AllowUnpaidExecutionFrom<IsChildSystemParachain<ParaId>>,
  // Expected responses are OK.
  AllowKnownQueryResponses<XcmPallet>,
  // Subscriptions for version tracking are OK.
  AllowSubscriptionsFrom<Everything>,
);
```

---v

## 🚧 XCM `Barrier` in Rococo

- `TakeWeightCredit` and `AllowTopLevelPaidExecutionFrom` are used to prevent spamming for local/remote XCM execution.
- `AllowUnpaidExecutionFrom` lets a system parachain have free execution in the relay.
- `AllowKnownQueryResponses` and `AllowSubscriptionsFrom`, as we know already, are mostly used for versioning.

Notes:

- Child system parachains are parachains that contain core polkadot features, and they will get a paraId of less than 1000.
  They are allocated by Polkadot governance and get free execution.
- `AllowKnownQueryResponses` will check pallet-xcm storage to know whether the response is expected. -`AllowSubscriptionsFrom` determines that any origin is able to subscribe for version changes.

---

## 🤝 Trusted teleporters in Rococo

```rust [0|2|6-8|18-22]
parameter_types! {
  pub const RocLocation: MultiLocation = Here.into();
  pub const Rococo: MultiAssetFilter =
    Wild(AllOf { fun: WildFungible, id: Concrete(RocLocation::get()) });

  pub const AssetHub: MultiLocation = Parachain(1000).into();
  pub const Contracts: MultiLocation = Parachain(1002).into();
  pub const Encointer: MultiLocation = Parachain(1003).into();

  pub const RococoForAssetHub: (MultiAssetFilter, MultiLocation) =
    (Rococo::get(), AssetHub::get());
  pub const RococoForContracts: (MultiAssetFilter, MultiLocation) =
    (Rococo::get(), Contracts::get());
  pub const RococoForEncointer: (MultiAssetFilter, MultiLocation) =
    (Rococo::get(), Encointer::get());
}

pub type TrustedTeleporters = (
  xcm_builder::Case<RococoForAssetHub>,
  xcm_builder::Case<RococoForContracts>,
  xcm_builder::Case<RococoForEncointer>,
);
```

---v

## 🤝 Trusted teleporters in Rococo

- Teleporting involves trust between chains.
- 1000 (Asset Hub) and 1001 (Contracts) and 1002 (Encointer) are allowed to teleport tokens represented by the **Here**
- **Here** represents the relay token

```rust
impl xcm_executor::Config for XcmConfig {
  /* snip */
  type IsTeleporter = TrustedTeleporters;
  /* snip */
}
```

Notes:

- Asset Hub, Rococo and Encointer are able to teleport the relay chain token
- Any other chain sending a `ReceiveTeleportedAsset` or any other token being teleported will be rejected with `UntrustedTeleportLocation`.

---

## 💱Trusted reserves in Rococo

- Rococo does not recognize any chain as reserve
- Rococo prevents reception of any **ReserveAssetDeposited** message

```rust
impl xcm_executor::Config for XcmConfig {
  /* snip */
  type IsReserve = ();
  /* snip */
}
```

Notes:

- Trusting other parachains (e.g., common good parachains) to be reserves of the relay native token would cause rare situations with the total issuance.
  For instance, users could drain reserves of the sovereign account with teleported funds.

---

## 📁 `LocationToAccountId` in Rococo

- Conversion between a multilocation to an AccountId is a key component to withdraw/deposit assets and issue `Transact` operations.
- Parachain origins will be converted to their corresponding sovereign account.
- Local 32 byte origins will be converted to a 32 byte defined AccountId.

```rust
pub type LocationConverter = (
  // We can convert a child parachain using the standard `AccountId` conversion.
  ChildParachainConvertsVia<ParaId, AccountId>,
  // We can directly alias an `AccountId32` into a local account.
  AccountId32Aliases<RococoNetwork, AccountId>,
);
```

Notes:

- Any other origin that is not a parachain origin or a local 32 byte account origin will not be convertible to an accountId.
- Question class what happens if a message coming from a parachain starts with `DescendOrigin`?
  XcmV2 will reject it at the barrier level (Since **_AllowTopLevelPaidExecutionFrom_** expects the first instruction to be one of **_ReceiveTeleportedAsset_** , **_WithdrawAsset_** , **_ReserveAssetDeposited_** or **_ClaimAsset_** - XcmV3 will pass the barrier as **_AllowTopLevelPaidExecutionFrom_** is inside **_WithComputedOrigin_**.

---

## 🪙 Asset Transactors in Rococo

<div style="font-size: smaller">

```rust
pub type LocalAssetTransactor = XcmCurrencyAdapter<
  // Use this currency:
  Balances,
  // Use this currency when it is a fungible asset
  // matching the given location or name:
  IsConcrete<RocLocation>,
  // We can convert the MultiLocations
  // with our converter above:
  LocationConverter,
  // Our chain's account ID type
  // (we can't get away without mentioning it explicitly):
  AccountId,
  // It's a native asset so we keep track of the teleports
  // to maintain total issuance.
  CheckAccount,
>;

impl xcm_executor::Config for XcmConfig {
  /* snip */
  type AssetTransactor = LocalAssetTransactor;
  /* snip */
}
```

</div>

---v

## 🪙 `asset-transactors` in Rococo

- Single asset-transactor in Rococo
- Asset-transactor is matching the **Here** multilocation id to the Currency defined in **Balances**, which refers to **_pallet-balances_**
- Essentially, this is configuring XCM such that the native token (DOT) is associated with the multilocation **Here**.

Notes:

- Rococo is tracking teleports in the **CheckAccount**, which is defined in **palletXcm**.
  This aims at maintaining the total issuance even if assets have been teleported to another chain.

---

## 📍`origin-converter` in Rococo

```rust
type LocalOriginConverter = (
  // Converts to a signed origin with "LocationConverter"
  SovereignSignedViaLocation<LocationConverter, RuntimeOrigin>,
  // Converts a child parachain multilocation to a parachain origin
  ChildParachainAsNative<parachains_origin::Origin, RuntimeOrigin>,
  // Converts a local 32 byte multilocation to a signed
  // origin
  SignedAccountId32AsNative<WestendNetwork, RuntimeOrigin>,
  // Converts system parachain origins into root origin
  ChildSystemParachainAsSuperuser<ParaId, RuntimeOrigin>,
);
```

```rust
impl xcm_executor::Config for XcmConfig {
  /* snip */
  type OriginConverter = LocalOriginConverter;
  /* snip */
}
```

---v

## 📍`origin-converter` in Rococo

- Defines ways in which we can convert a multilocation to a dispatch origin, typically used by the `Transact` instruction:
- Child parachain origins are converted to signed origins through **_LocationConverter_** (`OriginKind == Sovereign`).
- Child parachains can also be converted to native parachain origins (`OriginKind == Native`).
- Local 32 byte origins are converted to signed 32 byte origins (`OriginKind == Native`).

Notes:

- There exists the concept of a "parachain dispatch origin" which is used for very specific functions (like, e.g., opening a channel with another chain).
  This gets checked with the _ensure_parachain!_ macro.
- System parachains are able to dispatch as root origins, as they can bee seen as an extension to the rococo runtime itself.

---

### 🏋️ `Weigher` in Rococo

- Uses `WeightInfoBounds` with benchmarked values with `pallet-xcm-benchmarks`
- Full list of weights can be seen [here](https://github.com/paritytech/polkadot-sdk/tree/cc9f812/polkadot/runtime/rococo/src/weights/xcm)

```rust
impl xcm_executor::Config for XcmConfig {
  /* snip */
type Weigher = WeightInfoBounds<
		crate::weights::xcm::RococoXcmWeight<RuntimeCall>,
		RuntimeCall,
		MaxInstructions,
	>;
 /* snip */
}
```

---

## 🔧 `WeightTrader` in Rococo

- Weight is converted to fee with the **_WeightToFee_** type.
- The asset in which we charge for fee is **_RocLocation_**.
  This means we can only pay for xcm execution in the **native currency**
- Fees will go to the block author thanks to **_ToAuthor_**

```rust
impl xcm_executor::Config for XcmConfig {
  /* snip */
  type Trader = UsingComponents<
    WeightToFee,
	RocLocation,
	AccountId,
	Balances,
	ToAuthor<Runtime>>;
  /* snip */
}
```

Notes:

- Trying to buyExecution with any other token that does not match the specified AssetId (in this case, `RocLocation`, which represents the native token) **will fail**.

- **_WeightToFee_** contains an associated function that will be used to convert the required amount of weight into an amount of tokens used for execution payment.

---

## 🎨 `XcmPallet` in Rococo

```rust
impl pallet_xcm::Config for Runtime {
  /* snip */
  type XcmRouter = XcmRouter;
  type SendXcmOrigin =
    xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
  // Anyone can execute XCM messages locally.
  type ExecuteXcmOrigin =
    xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
  type XcmExecuteFilter = Everything;
  type XcmExecutor = xcm_executor::XcmExecutor<XcmConfig>;
  // Anyone is able to use teleportation
  // regardless of who they are and what they want to teleport.
  type XcmTeleportFilter = Everything;
  // Anyone is able to use reserve transfers
  // regardless of who they are and what they want to transfer.
  type XcmReserveTransferFilter = Everything;
  /* snip */
}
```

---v

## 🎨 XcmPallet in Rococo

- No filter on messages for Execution, Teleporting or Reserve transferring.
- Only origins defined by **_LocalOriginToLocation_** are allowed to send/execute arbitrary messages.
- **_LocalOriginToLocation_** defined to allow council and regular account 32 byte signed origin calls

```rust
pub type LocalOriginToLocation = (
  // We allow an origin from the Collective pallet to be used in XCM
  // as a corresponding Plurality of the `Unit` body.
  CouncilToPlurality,
  // And a usual Signed origin to be used in XCM as a corresponding AccountId32
  SignedToAccountId32<RuntimeOrigin, AccountId, RococoNetwork>,
);
```

Notes:

- **_LocalOrigin_** allows to go from a frame dispatch origin to a multilocation.
  This is necessary because **we enter the xcm-executor with xcm origins, not with frame dispatch origins**.
  Note that this is an extrinsic in a frame pallet, and thus, **we call it with frame origins**.

- Council decisions are converted to `Plurality` junction multilocations.

- Signed origins are converted to `AccountId32` junction multilocations.

---
