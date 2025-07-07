#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

pub use frame_support::{
  construct_runtime, parameter_types,
  traits::{ConstU128, ConstU32, ConstU64, ConstU8, KeyOwnerProofSystem, Randomness, StorageInfo},
  weights::{
    constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
    IdentityFee, Weight,
  },
  StorageValue,
};
use sp_api::impl_runtime_apis;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
use sp_runtime::{
  create_runtime_str, generic,
  traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, IdentifyAccount, Verify},
  transaction_validity::{TransactionSource, TransactionValidity},
  ApplyExtrinsicResult, MultiSignature,
};
pub use sp_runtime::{Perbill, Permill};
use sp_std::prelude::*;
use sp_version::RuntimeVersion;

pub type Signature = MultiSignature;
// TODO: exercise: let's decrypt this line! AccountId32
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

// pub type Signature = sp_core::sr25519::Signature;
// pub type AccountId = sp_core::sr25519::Public;

pub type Hash = sp_core::H256;
pub type BlockNumber = u32;
pub type Balance = u128;
pub type Index = u32;

// One of apis that the runtime needs to implement requires it to know its version!
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
  spec_name: create_runtime_str!("simple-runtime"),
  impl_name: create_runtime_str!("simple-runtime"),
  authoring_version: 1,
  spec_version: 100,
  impl_version: 1,
  apis: RUNTIME_API_VERSIONS,
  transaction_version: 1,
  state_version: 1,
};

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::one();
parameter_types! {
  pub const Version: RuntimeVersion = VERSION;
  pub const BlockHashCount: BlockNumber = 2400;

  pub BlockWeights: frame_system::limits::BlockWeights = frame_system::limits::BlockWeights
  ::with_sensible_defaults(2 * WEIGHT_PER_SECOND, NORMAL_DISPATCH_RATIO);
  pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
  ::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);

  pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Runtime {
  // these might ring bell for you now.
  type AccountId = AccountId;
  type Index = Index;
  type Hash = Hash;
  type Hashing = BlakeTwo256;
  type Version = Version;

  // we will look into what this in the expanded version.
  type Call = Call;

  // this one's more important, keep this also in mind!
  type Origin = Origin;

  // the rest probably won't and no worries for now!
  type BaseCallFilter = frame_support::traits::Everything;
  type BlockWeights = BlockWeights;
  type BlockLength = BlockLength;
  type Lookup = AccountIdLookup<AccountId, ()>;
  type BlockNumber = BlockNumber;
  type Header = generic::Header<BlockNumber, BlakeTwo256>;
  type Event = Event;
  type BlockHashCount = BlockHashCount;
  type DbWeight = RocksDbWeight;
  type PalletInfo = PalletInfo;
  type OnNewAccount = ();
  type OnKilledAccount = ();
  type AccountData = ();
  type SystemWeightInfo = ();
  type SS58Prefix = SS58Prefix;
  type OnSetCode = ();
  type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl simple_pallet::pallet::Config for Runtime {
  // recall that we wanted a type here that's From<u32>.
  type ValueType = u64;
  const MAX_VALUE: u32 = 1000;
  fn on_value_update(_: Self::ValueType) {
    todo!();
  }
}

impl transfer_pallet::Config for Runtime {
  type MinimumBalance = frame_support::traits::ConstU128<100>;
}

pub mod opaque {
  use super::BlockNumber;
  pub use sp_runtime::OpaqueExtrinsic;
  use sp_runtime::{generic, traits::BlakeTwo256};
  pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
  pub type Block = generic::Block<Header, OpaqueExtrinsic>;
  pub type BlockId = generic::BlockId<Block>;
}

// This is the marco that amalgamates all of our types together, and creates some important types
// such as:
// - struct Runtime, which you already see in this file
// - implements the Config trait of all pallets.
// - `type System`, `type SimplePallet`.
// - AllPalletsWithSystem
//   - and recall that all pallets implement things like `Hooks`, `OnInitialize`, and all of these
//     traits are tuple-able.
// - enum Call
// - enum Event, GenesisConfig, Error etc. but we don't have them here.
//
// This macro's kinda old, and will hopefully be transformed into a better version soon (enum
// Runtime: faceplam).
construct_runtime!(
  pub enum Runtime where
  Block = Block,
  NodeBlock = opaque::Block,
  UncheckedExtrinsic = UncheckedExtrinsic
  {
    System: frame_system = 2,
    SimplePallet: simple_pallet = 1,
    TransferPallet: transfer_pallet = 0,
  }
);

// we visit these in the last part and transition into executive
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
pub type SignedExtra = (frame_system::CheckGenesis<Runtime>, frame_system::CheckNonce<Runtime>);
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
pub type SignedPayload = generic::SignedPayload<Call, SignedExtra>;
pub type Executive = frame_executive::Executive<
  Runtime,
  Block,
  frame_system::ChainContext<Runtime>,
  Runtime,
  AllPalletsWithSystem,
>;

// this is the juicy part! all implementations seem to come from Executive!
impl_runtime_apis! {
  impl sp_api::Core<Block> for Runtime {
    fn version() -> RuntimeVersion {
      VERSION
    }

    fn execute_block(block: Block) {
      Executive::execute_block(block);
    }

    fn initialize_block(header: &<Block as BlockT>::Header) {
      Executive::initialize_block(header)
    }
  }

  impl sp_block_builder::BlockBuilder<Block> for Runtime {
    fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
      Executive::apply_extrinsic(extrinsic)
    }

    fn finalize_block() -> <Block as BlockT>::Header {
      Executive::finalize_block()
    }

    fn inherent_extrinsics(_: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
      unimplemented!()
    }

    fn check_inherents(
      _: Block,
      _: sp_inherents::InherentData,
    ) -> sp_inherents::CheckInherentsResult {
      unimplemented!()
    }
  }

  impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
    fn validate_transaction(
      source: TransactionSource,
      tx: <Block as BlockT>::Extrinsic,
      block_hash: <Block as BlockT>::Hash,
    ) -> TransactionValidity {
      Executive::validate_transaction(source, tx, block_hash)
    }
  }
}
