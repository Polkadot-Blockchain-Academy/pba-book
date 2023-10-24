---
title: XCM Beyond Asset Transfers
description: Deep dive on advanced XCM use cases beyond asset transfers and bridging
duration: 1 hour
---

# XCM Beyond Asset Transfers

---

## Outline

<pba-flex center>

1. Pre-requisites
1. XCMultisig
1. XCM Universal Interface
1. General XCM Tips
1. Conclusion
1. Next Steps
1. References

</pba-flex>

---

## Pre-requisites

The following are expected:

<pba-flex center>

- Knowledge of core XCM concepts
- Knowledge of XCM chain configuration

</pba-flex>

---

## XCMultisig

[InvArch Network](https://invarch.network/) has the concept of XCMultisigs, these are entities that exist on the InvArch Network runtime and provide advanced multisig capabilities to users across many other blockchains.

Let's go over how that works!

Notes:

The reason for the name comes from their XCM functionality, more specifically from the idea that these entities have their logic defined in the InvArch Network runtime, but exist in all other connected chains with the same exact account address, thus allowing them to transact in these chains through XCM.

---v

### Overview

<diagram class="mermaid">
stateDiagram-v2
state Polkadot {
    direction LR

    state InvArch {
        direction LR
        v0: Multisig ID 0
        sxc: Send XCM Call
        vacc: 0x123...

        state if_state <<choice>>
        v0 --> if_state

        if_state --> sxc
        if_state --> vacc

        vacc --> [*]: Transact


        sxc --> h0
        sxc --> i0
        sxc --> b0
    }

    state HydraDX {
        direction LR

        h0: Multisig ID 0
        hmxs: **XCM Converters**
        hacc: 0x123...

        h0 --> hmxs
        hmxs --> hacc
        hacc --> [*]: Transact
    }

    state Interlay {
        direction LR

        i0: Multisig ID 0
        imxs: **XCM Converters**
        iacc: 0x123...

        i0 --> imxs
        imxs --> iacc
        iacc --> [*]: Transact
    }

    state Bifrost {
        direction LR

        b0: Multisig ID 0
        bmxs: **XCM Converters**
        bacc: 0x123...

        b0 --> bmxs
        bmxs --> bacc
        bacc --> [*]: Transact
    }

}
</diagram>

---v

### Message details

To better understand how this all works, let's go over the messages being sent and their origins.

```rust [0|1-13|15-33|35|37-53|55|57-75]
let multisig: MultiLocation = MultiLocation {
  parents: 1,
  interior: Junctions::X3(
    Junction::Parachain(<T as pallet::Config>::ParaId::get()),
    Junction::PalletInstance(<T as pallet::Config>::INV4PalletIndex::get()),
    Junction::GeneralIndex(0u128),
  ),
};

let multisig_interior = Junctions::X2(
  Junction::PalletInstance(<T as pallet::Config>::INV4PalletIndex::get()),
  Junction::GeneralIndex(0u128),
);

let destination = MultiLocation {
  parents: 1,
  interior: Junctions::X1(
    Junction::Parachain(1234)
  ),
};

let fee_asset_location = MultiLocation {
  parents: 1,
  interior: Junctions::X2(
    Junction::Parachain(1234),
    Junction::GeneralIndex(0),
  ),
};

let fee_multiasset = MultiAsset {
  id: AssetId::Concrete(fee_asset_location),
  fun: Fungibility::Fungible(1000000000000),
};

let call = vec![...];

let message = Xcm(vec![
  Instruction::WithdrawAsset(fee_multiasset.clone().into()),
  Instruction::BuyExecution {
    fees: fee_multiasset,
    weight_limit: WeightLimit::Unlimited,
  },
  Instruction::Transact {
    origin_kind: OriginKind::Native,
    require_weight_at_most: 5000000000,
    call: <DoubleEncoded<_> as From<Vec<u8>>>::from(call),
  },
  Instruction::RefundSurplus,
  Instruction::DepositAsset {
    assets: MultiAssetFilter::Wild(WildMultiAsset::All),
    beneficiary: multisig,
  },
]);

pallet_xcm::Pallet::<T>::send_xcm(multisig_interior, destination, message)?;

// Pallet XCM will then add a DescendOrigin instruction to index 0 of the message.
Instruction::DescendOrigin(multisig_interior)

// Which mutates the initial Origin
MultiLocation {
  parents: 1,
  interior: Junctions::X1(
    Junction::Parachain(<T as pallet::Config>::ParaId::get()),
  ),
}
// Becomes
MultiLocation {
  parents: 1,
  interior: Junctions::X3(
    Junction::Parachain(<T as pallet::Config>::ParaId::get()),
    Junction::PalletInstance(<T as pallet::Config>::INV4PalletIndex::get()),
    Junction::GeneralIndex(0u128),
  ),
}
```

---v

### XCM Converters

Now that we understand the origin and message structure, let's take a look at those _XCM Converters_!

<diagram class="mermaid">
stateDiagram-v2
  direction LR

    para: Parachain 2125
    pal: Pallet 51
    m: Mulisig ID 0
    acc: 0x123...

    para --> if
    pal --> if
    m --> hash

      state Checks {
        if: Parachain == 2125 && Pallet == 51
        if --> [*]: No Match
        if --> Hasher: Match
      }

      state Hasher {
        cs: Constant Salt

        cs --> hash
      }

    hash --> acc

</diagram>

Notes:

The reason for the custom hasher is to replicate the account generation in the origin chain.
The combination of these checks and the hasher makes up the converters that return AccountIds and native Origins for our MultiLocation.

---v

### What happens if we map AccountId origins to the exact accounts within?

## Account Impersonation!

<!-- .element: class="fragment" data-fragment-index="0" -->

Hey Chain B, I'm sending you a balance transfer request from one of my users, their address is "Chain B's treasury" ;)

<!-- .element: class="fragment" data-fragment-index="1" -->

# TRUST!

<!-- .element: class="fragment" data-fragment-index="2" -->

Notes:

Emphatically explain this!

---

## XCM Universal Interface

XCM can be used as a general API abstraction on top of multiple blockchains.
With some clever usage, we can build chains that can be integrated by dApps in a generic manner, and also dApps that easily integrate multiple chains without any custom logic.

---v

## Concept

###### XCM Powered Multichain NFT Marketplace

Imagine an NFT marketplace where not only multiple chains are supported, but also any standards these chains choose to implement!

---v

### How?

<diagram class="mermaid">
stateDiagram-v2
  direction TB

    ui: UI
    xcm: XCM API
    indexer: Indexer

    ui --> xcm

    indexer --> ui

    xcm --> axti
    xcm --> mxti
    xcm --> cxti

    state Asset_Hub {
      axti: XCM AssetExchanger
      apu: Pallet Uniques
      apn: Pallet NFTs

      axti --> apu
      axti --> apn
    }

    state Moonbeam {
      mxti: XCM AssetExchanger
      mpe: Pallet EVM

      mxti --> mpe
    }

    state Chain_C {
      cxti: XCM AssetExchanger
      cpu: Pallet Uniques
      cpc: Pallet Contracts

      cxti --> cpu
      cxti --> cpc
    }

</diagram>

---v

### Matching NFTs

```rust [0|4-15|18-21]
MultiAsset {
  // Where to find the NFT (contract or collection in an NFT pallet)
  id: AssetId::Concrete (
    Multilocation {
      parents: 0,
      interior: Junctions::X3(
        // Parachain ID just so we can pre-check if this message was intended for this chain
        Junction::Parachain (para_id),
        // Pallet ID so we know which pallet we should be using to look up the NFT
        Junction::PalletInstance(pallet_id),
        // General Index to select a specific collection by integer id
        // Or GeneralKev to select a specific collection bv it's contract id
        Junction::GeneralIndex(collection_id) or Junction::GeneralKey(contract_address),
      )
    }
  ),
  // The NFT itself
  fun: Fungibility::NonFungible(
    // Specific NFT instance inside the collection selected by it's id
    AssetInstance::Instance(nft_id)
  )
}
```

---v

### Implementing AssetExchanger

```rust [1-20|22-38|40-48]
pub trait AssetExchange {
	/// Handler for exchanging an asset.
	///
	/// - `origin`: The location attempting the exchange; this should generally not matter.
	/// - `give`: The assets which have been removed from the caller.
	/// - `want`: The minimum amount of assets which should be given to the caller in case any
	///   exchange happens. If more assets are provided, then they should generally be of the
	///   same asset class if at all possible.
	/// - `maximal`: If `true`, then as much as possible should be exchanged.
	///
	/// `Ok` is returned along with the new set of assets which have been exchanged for `give`. At
	/// least want must be in the set. Some assets originally in `give` may also be in this set. In
	/// the case of returning an `Err`, then `give` is returned.
	fn exchange_asset(
		origin: Option<&MultiLocation>,
		give: Assets,
		want: &MultiAssets,
		maximal: bool,
	) -> Result<Assets, Assets>;
}

struct MyNftStandardExchanger;

impl AssetExchange for MyNftStandardExchanger {
	fn exchange_asset(
		origin: Option<&MultiLocation>,
		give: Assets,
		want: &MultiAssets,
		maximal: bool,
	) -> Result<Assets, Assets> {
    match (give, want) {
      (FUNGIBLE, NONFUNGIBLE) => MyNftPallet::buy(...),
      (NONFUNGIBLE, FUNGIBLE) => MyNftPallet::sell(...),
      (NONFUNGIBLE, NONFUNGIBLE) => MyNftPallet::swap(...),
      (FUNGIBLE, FUNGIBLE) => Err(give),
    }
	}
}

impl xcm_executor::Config for XcmConfig {
  ...
  type AssetExchanger = (
    MyNftStandardExchanger,
    EvmNftExchanger,
    PalletUniquesExchanger
  );
  type AssetTransactor = AssetTransactors;
}
```

---

## General XCM Tips

In this section we will go over some general tips on how to build with XCM.

---v

### MultiLocations & MultiAssets

Deciding how to map MultiLocations to entities in your runtime is very important, as these MultiLocations will end up being used across other XCM-connected chains.

```rust [1-4|6-8|10-11|13-14|16-17|19-24|26-32]
// Main runtime token
Junctions::X1(Parachain(para_id));
Junctions::X2(Parachain(para_id), GeneralIndex(main_token_id));
Junctions::X2(Parachain(para_id), PalletInstance(balances_pallet_id));

// Other tokens
Junctions::X2(Parachain(para_id), GeneralIndex(token_id));
Junctions::X3(Parachain(para_id), PalletInstance(tokens_pallet_id), GeneralIndex(token_id));

// Runtime protocols (i.e. Treasury or it's account)
Junctions::X2(Parachain(para_id), PalletInstance(treasury_pallet_id));

// Wasm smart contracts
Junctions::X3(Parachain(para_id), PalletInstance(contracts_pallet_id), AccountId32(wasm_contract_account));

// EVM smart contracts
Junctions::X3(Parachain(para_id), PalletInstance(evm_pallet_id), AccountKey20(evm_contract_account));

// Wasm or EVM contracts
Junctions::X3(
  Parachain(para_id),
  PalletInstance(contracts_pallet_id || evm_pallet_id),
  AccountId32(wasm_contract_account) || AccountKey20(evm_contract_account),
);

// NFTs
MultiAsset {
  // Match collection
  id: Concrete(wasm_or_evm_multilocation || X2(Parachain(para_id), PalletInstance(nft_pallet_id))),
  // Match item
  fun: Fungibility::NonFungible(AssetInstance::Index(nft_id))
};
```

---v

### Message Instructions

```rust [1-23|25-45]
// Pay for execution fees and refund surplus
Xcm(vec![
  // Withdraw asset to use within this message, places the amount in the holding register.
  Instruction::WithdrawAsset(fee_multiasset.into()),
  // Pay for execution fees during this message.
  Instruction::BuyExecution {
    // The asset and amount we withdrew in the first instruction.
    fees: fee_multiasset,
    // Max amount of weight we are willing to buy.
    weight_limit: WeightLimit::Unlimited,
  },
  // An instruction or set of instructions that will require payment of execution fees.
  <Instruction that pays execution fee>,
  // Refund unused purchased weight back to the holding register.
  Instruction::RefundSurplus,
  // Deposit assets from the holding register back into the balance of an account.
  Instruction::DepositAsset {
    // Match total amount of all assets in the holding register.
    assets: MultiAssetFilter::Wild(WildMultiAsset::All),
    // The receiver of the refunded fees, usually the origin that paid for the fees in the first place.
    beneficiary: account_id_multilocation,
  },
]);

// XCM assertions

// Errors is described pallet does not exist in the runtime.
ExpectPallet {
  // Pallet index.
  index: 21,
  // Pallet name.
  name: "Referenda".as_bytes().to_vec(),
  // Name of the module.
  module_name: "pallet_referenda".as_bytes().to_vec(),
  // Major version of the crate.
  crate_major: 4,
  // Minimum minor version acceptable.
  min_crate_minor: 0,
}

// Errors if described asset and amount are not present in the holding register.
ExpectAsset(MultiAsset {
	id: AssetId::Concrete(asset_multilocation),
  fun: Fungibility::Fungible(1_000_000_000_000u128),
})
```

---

## Conclusion

During this presentation we went through a couple real world XCM use cases and some general tips for working with the message standard, the goal here is to leave you with some inspiration and some ideas, so that you can start tinkering with XCM to power your own ideas and supercharge blockchain applications!

---

## References

- [XCM source code](https://github.com/paritytech/polkadot-sdk/tree/master/polkadot/xcm) - The source code for the main XCM implementation in the paritytech/polkadot repository.

<!-- FIXME new repo expected soon for XCM outside SDK -->

- [XCM Documentation](https://paritytech.github.io/xcm-docs/) - The official documentation for XCM: Cross-Consensus Messaging.
- [InvArch's Pallet Rings](https://github.com/InvArch/InvArch-Frames/tree/main/pallet-rings) - Reference implementation of an XCM abstraction pallet for XCMultisigs.
