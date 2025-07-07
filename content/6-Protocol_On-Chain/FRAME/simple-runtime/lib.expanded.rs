#![feature(prelude_import)]
#![recursion_limit = "256"]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use sp_api::impl_runtime_apis;
use sp_runtime::{
    create_runtime_str, generic,
    traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, IdentifyAccount, NumberFor, Verify},
    transaction_validity::{TransactionSource, TransactionValidity},
    ApplyExtrinsicResult, MultiSignature,
};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;
pub use frame_support::{
    construct_runtime, parameter_types,
    traits::{
        ConstU128, ConstU32, ConstU64, ConstU8, KeyOwnerProofSystem, Randomness, StorageInfo,
    },
    weights::{
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
        IdentityFee, Weight,
    },
    StorageValue,
};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{Perbill, Permill};
pub type BlockNumber = u32;
pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type Hash = sp_core::H256;
pub type Balance = u128;
pub type Index = u32;
/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
    use super::*;
    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
    /// Opaque block header type.
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;
}
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: { ::sp_runtime::RuntimeString::Borrowed("simple-runtime") },
    impl_name: { ::sp_runtime::RuntimeString::Borrowed("simple-runtime") },
    authoring_version: 1,
    spec_version: 100,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
    state_version: 1,
};
const _: () = {};
pub const MILLISECS_PER_BLOCK: u64 = 6000;
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;
/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
pub struct BlockHashCount;
impl BlockHashCount {
    /// Returns the value of this parameter type.
    pub const fn get() -> BlockNumber {
        2400
    }
}
impl<I: From<BlockNumber>> ::frame_support::traits::Get<I> for BlockHashCount {
    fn get() -> I {
        I::from(Self::get())
    }
}
impl ::frame_support::traits::TypedGet for BlockHashCount {
    type Type = BlockNumber;
    fn get() -> BlockNumber {
        Self::get()
    }
}
pub struct Version;
impl Version {
    /// Returns the value of this parameter type.
    pub const fn get() -> RuntimeVersion {
        VERSION
    }
}
impl<I: From<RuntimeVersion>> ::frame_support::traits::Get<I> for Version {
    fn get() -> I {
        I::from(Self::get())
    }
}
impl ::frame_support::traits::TypedGet for Version {
    type Type = RuntimeVersion;
    fn get() -> RuntimeVersion {
        Self::get()
    }
}
pub struct BlockWeights;
impl BlockWeights {
    /// Returns the value of this parameter type.
    pub fn get() -> frame_system::limits::BlockWeights {
        frame_system::limits::BlockWeights::with_sensible_defaults(
            2 * WEIGHT_PER_SECOND,
            NORMAL_DISPATCH_RATIO,
        )
    }
}
impl<I: From<frame_system::limits::BlockWeights>> ::frame_support::traits::Get<I> for BlockWeights {
    fn get() -> I {
        I::from(Self::get())
    }
}
impl ::frame_support::traits::TypedGet for BlockWeights {
    type Type = frame_system::limits::BlockWeights;
    fn get() -> frame_system::limits::BlockWeights {
        Self::get()
    }
}
pub struct BlockLength;
impl BlockLength {
    /// Returns the value of this parameter type.
    pub fn get() -> frame_system::limits::BlockLength {
        frame_system::limits::BlockLength::max_with_normal_ratio(
            5 * 1024 * 1024,
            NORMAL_DISPATCH_RATIO,
        )
    }
}
impl<I: From<frame_system::limits::BlockLength>> ::frame_support::traits::Get<I> for BlockLength {
    fn get() -> I {
        I::from(Self::get())
    }
}
impl ::frame_support::traits::TypedGet for BlockLength {
    type Type = frame_system::limits::BlockLength;
    fn get() -> frame_system::limits::BlockLength {
        Self::get()
    }
}
pub struct SS58Prefix;
impl SS58Prefix {
    /// Returns the value of this parameter type.
    pub const fn get() -> u8 {
        42
    }
}
impl<I: From<u8>> ::frame_support::traits::Get<I> for SS58Prefix {
    fn get() -> I {
        I::from(Self::get())
    }
}
impl ::frame_support::traits::TypedGet for SS58Prefix {
    type Type = u8;
    fn get() -> u8 {
        Self::get()
    }
}
impl frame_system::Config for Runtime {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = BlockWeights;
    type BlockLength = BlockLength;
    type AccountId = AccountId;
    type Call = Call;
    type Lookup = AccountIdLookup<AccountId, ()>;
    type Index = Index;
    type BlockNumber = BlockNumber;
    type Hash = Hash;
    type Hashing = BlakeTwo256;
    type Header = generic::Header<BlockNumber, BlakeTwo256>;
    type Event = Event;
    type Origin = Origin;
    type BlockHashCount = BlockHashCount;
    type DbWeight = RocksDbWeight;
    type Version = Version;
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
    type ValueType = u64;
    const MAX_VALUE: u32 = 1000;
    fn on_value_update(new_value: Self::ValueType) {
        ::core::panicking::panic("not yet implemented");
    }
}
#[doc(hidden)]
mod sp_api_hidden_includes_construct_runtime {
    pub extern crate frame_support as hidden_include;
}
const _: () = {
    #[allow(unused)]
    type __hidden_use_of_unchecked_extrinsic = UncheckedExtrinsic;
};
pub struct Runtime;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Runtime {
    #[inline]
    fn clone(&self) -> Runtime {
        {
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for Runtime {}
impl ::core::marker::StructuralPartialEq for Runtime {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for Runtime {
    #[inline]
    fn eq(&self, other: &Runtime) -> bool {
        true
    }
}
impl ::core::marker::StructuralEq for Runtime {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for Runtime {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {}
    }
}
impl core::fmt::Debug for Runtime {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_tuple("Runtime").finish()
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for Runtime {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(::scale_info::Path::new("Runtime", "simple_runtime"))
                .type_params(::alloc::vec::Vec::new())
                .docs(&[])
                .composite(::scale_info::build::Fields::unit())
        }
    };
};
impl self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: traits :: GetNodeBlockType for Runtime { type NodeBlock = opaque :: Block ; }
impl self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: traits :: GetRuntimeBlockType for Runtime { type RuntimeBlock = Block ; }
#[allow(non_camel_case_types)]
pub enum Event {
    #[codec(index = 0u8)]
    System(frame_system::Event<Runtime>),
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::clone::Clone for Event {
    #[inline]
    fn clone(&self) -> Event {
        match (&*self,) {
            (&Event::System(ref __self_0),) => {
                Event::System(::core::clone::Clone::clone(&*__self_0))
            }
        }
    }
}
#[allow(non_camel_case_types)]
impl ::core::marker::StructuralPartialEq for Event {}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::cmp::PartialEq for Event {
    #[inline]
    fn eq(&self, other: &Event) -> bool {
        match (&*self, &*other) {
            (&Event::System(ref __self_0), &Event::System(ref __arg_1_0)) => {
                *__self_0 == *__arg_1_0
            }
        }
    }
    #[inline]
    fn ne(&self, other: &Event) -> bool {
        match (&*self, &*other) {
            (&Event::System(ref __self_0), &Event::System(ref __arg_1_0)) => {
                *__self_0 != *__arg_1_0
            }
        }
    }
}
#[allow(non_camel_case_types)]
impl ::core::marker::StructuralEq for Event {}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::cmp::Eq for Event {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<frame_system::Event<Runtime>>;
        }
    }
}
#[allow(deprecated)]
const _: () = {
    #[allow(non_camel_case_types)]
    #[automatically_derived]
    impl ::codec::Encode for Event {
        fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            match *self {
                Event::System(ref aa) => {
                    __codec_dest_edqy.push_byte(0u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                _ => (),
            }
        }
    }
    #[automatically_derived]
    impl ::codec::EncodeLike for Event {}
};
#[allow(deprecated)]
const _: () = {
    #[allow(non_camel_case_types)]
    #[automatically_derived]
    impl ::codec::Decode for Event {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| e.chain("Could not decode `Event`, failed to read variant byte"))?
            {
                __codec_x_edqy if __codec_x_edqy == 0u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Event::System({
                        let __codec_res_edqy =
                            <frame_system::Event<Runtime> as ::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Event::System.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                    "Could not decode `Event`, variant doesn't exist",
                )),
            }
        }
    }
};
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for Event {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            ::scale_info::Type::builder()
                .path(::scale_info::Path::new("Event", "simple_runtime"))
                .type_params(::alloc::vec::Vec::new())
                .docs(&[])
                .variant(::scale_info::build::Variants::new().variant("System", |v| {
                    v.index(0u8 as ::core::primitive::u8)
                        .fields(::scale_info::build::Fields::unnamed().field(|f| {
                            f.ty::<frame_system::Event<Runtime>>()
                                .type_name("frame_system::Event<Runtime>")
                                .docs(&[])
                        }))
                        .docs(&[])
                }))
        }
    };
};
impl core::fmt::Debug for Event {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::System(ref a0) => fmt.debug_tuple("Event::System").field(a0).finish(),
            _ => Ok(()),
        }
    }
}
impl From<frame_system::Event<Runtime>> for Event {
    fn from(x: frame_system::Event<Runtime>) -> Self {
        Event::System(x)
    }
}
impl TryInto<frame_system::Event<Runtime>> for Event {
    type Error = ();
    fn try_into(
        self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        frame_system::Event<Runtime>,
        Self::Error,
    > {
        match self {
            Self::System(evt) => Ok(evt),
            _ => Err(()),
        }
    }
}
/// The runtime origin type representing the origin of a call.
///
/// Origin is always created with the base filter configured in [`frame_system::Config::BaseCallFilter`].
pub struct Origin {
    caller: OriginCaller,
    filter: self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc<
        Box<dyn Fn(&<Runtime as frame_system::Config>::Call) -> bool>,
    >,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Origin {
    #[inline]
    fn clone(&self) -> Origin {
        Origin {
            caller: ::core::clone::Clone::clone(&self.caller),
            filter: ::core::clone::Clone::clone(&self.filter),
        }
    }
}
#[cfg(feature = "std")]
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::fmt::Debug for Origin {
    fn fmt(
        &self,
        fmt : & mut self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: fmt :: Formatter,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        (),
        self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::fmt::Error,
    > {
        fmt.debug_struct("Origin")
            .field("caller", &self.caller)
            .field("filter", &"[function ptr]")
            .finish()
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::OriginTrait
    for Origin
{
    type Call = <Runtime as frame_system::Config>::Call;
    type PalletsOrigin = OriginCaller;
    type AccountId = <Runtime as frame_system::Config>::AccountId;
    fn add_filter(&mut self, filter: impl Fn(&Self::Call) -> bool + 'static) {
        let f = self.filter.clone();
        self.filter =
            self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc::new(
                Box::new(move |call| f(call) && filter(call)),
            );
    }
    fn reset_filter(&mut self) {
        let filter = < < Runtime as frame_system :: Config > :: BaseCallFilter as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: Contains < < Runtime as frame_system :: Config > :: Call > > :: contains ;
        self.filter =
            self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc::new(
                Box::new(filter),
            );
    }
    fn set_caller_from(&mut self, other: impl Into<Self>) {
        self.caller = other.into().caller;
    }
    fn filter_call(&self, call: &Self::Call) -> bool {
        match self.caller {
            OriginCaller::system(frame_system::Origin::<Runtime>::Root) => true,
            _ => (self.filter)(call),
        }
    }
    fn caller(&self) -> &Self::PalletsOrigin {
        &self.caller
    }
    fn try_with_caller<R>(
        mut self,
        f: impl FnOnce(Self::PalletsOrigin) -> Result<R, Self::PalletsOrigin>,
    ) -> Result<R, Self> {
        match f(self.caller) {
            Ok(r) => Ok(r),
            Err(caller) => {
                self.caller = caller;
                Err(self)
            }
        }
    }
    fn none() -> Self {
        frame_system::RawOrigin::None.into()
    }
    fn root() -> Self {
        frame_system::RawOrigin::Root.into()
    }
    fn signed(by: Self::AccountId) -> Self {
        frame_system::RawOrigin::Signed(by).into()
    }
    fn as_signed(self) -> Option<Self::AccountId> {
        match self.caller {
            OriginCaller::system(frame_system::RawOrigin::Signed(by)) => Some(by),
            _ => None,
        }
    }
}
#[allow(non_camel_case_types)]
pub enum OriginCaller {
    #[codec(index = 0u8)]
    system(frame_system::Origin<Runtime>),
    #[allow(dead_code)]
    Void(self::sp_api_hidden_includes_construct_runtime::hidden_include::Void),
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::clone::Clone for OriginCaller {
    #[inline]
    fn clone(&self) -> OriginCaller {
        match (&*self,) {
            (&OriginCaller::system(ref __self_0),) => {
                OriginCaller::system(::core::clone::Clone::clone(&*__self_0))
            }
            (&OriginCaller::Void(ref __self_0),) => {
                OriginCaller::Void(::core::clone::Clone::clone(&*__self_0))
            }
        }
    }
}
#[allow(non_camel_case_types)]
impl ::core::marker::StructuralPartialEq for OriginCaller {}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::cmp::PartialEq for OriginCaller {
    #[inline]
    fn eq(&self, other: &OriginCaller) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&OriginCaller::system(ref __self_0), &OriginCaller::system(ref __arg_1_0)) => {
                        *__self_0 == *__arg_1_0
                    }
                    (&OriginCaller::Void(ref __self_0), &OriginCaller::Void(ref __arg_1_0)) => {
                        *__self_0 == *__arg_1_0
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &OriginCaller) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&OriginCaller::system(ref __self_0), &OriginCaller::system(ref __arg_1_0)) => {
                        *__self_0 != *__arg_1_0
                    }
                    (&OriginCaller::Void(ref __self_0), &OriginCaller::Void(ref __arg_1_0)) => {
                        *__self_0 != *__arg_1_0
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}
#[allow(non_camel_case_types)]
impl ::core::marker::StructuralEq for OriginCaller {}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::cmp::Eq for OriginCaller {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<frame_system::Origin<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<
                self::sp_api_hidden_includes_construct_runtime::hidden_include::Void,
            >;
        }
    }
}
impl core::fmt::Debug for OriginCaller {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::system(ref a0) => fmt.debug_tuple("OriginCaller::system").field(a0).finish(),
            Self::Void(ref a0) => fmt.debug_tuple("OriginCaller::Void").field(a0).finish(),
            _ => Ok(()),
        }
    }
}
#[allow(deprecated)]
const _: () = {
    #[allow(non_camel_case_types)]
    #[automatically_derived]
    impl ::codec::Encode for OriginCaller {
        fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            match *self {
                OriginCaller::system(ref aa) => {
                    __codec_dest_edqy.push_byte(0u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                OriginCaller::Void(ref aa) => {
                    __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                _ => (),
            }
        }
    }
    #[automatically_derived]
    impl ::codec::EncodeLike for OriginCaller {}
};
#[allow(deprecated)]
const _: () = {
    #[allow(non_camel_case_types)]
    #[automatically_derived]
    impl ::codec::Decode for OriginCaller {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            match __codec_input_edqy.read_byte().map_err(|e| {
                e.chain("Could not decode `OriginCaller`, failed to read variant byte")
            })? {
                __codec_x_edqy if __codec_x_edqy == 0u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(OriginCaller::system({
                        let __codec_res_edqy =
                            <frame_system::Origin<Runtime> as ::codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `OriginCaller::system.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 1usize as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(OriginCaller::Void({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: Void as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `OriginCaller::Void.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                    "Could not decode `OriginCaller`, variant doesn't exist",
                )),
            }
        }
    }
};
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for OriginCaller {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            :: scale_info :: Type :: builder () . path (:: scale_info :: Path :: new ("OriginCaller" , "simple_runtime")) . type_params (:: alloc :: vec :: Vec :: new ()) . docs (& []) . variant (:: scale_info :: build :: Variants :: new () . variant ("system" , | v | v . index (0u8 as :: core :: primitive :: u8) . fields (:: scale_info :: build :: Fields :: unnamed () . field (| f | f . ty :: < frame_system :: Origin < Runtime > > () . type_name ("frame_system::Origin<Runtime>") . docs (& []))) . docs (& [])) . variant ("Void" , | v | v . index (1usize as :: core :: primitive :: u8) . fields (:: scale_info :: build :: Fields :: unnamed () . field (| f | f . ty :: < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: Void > () . type_name ("self::sp_api_hidden_includes_construct_runtime::hidden_include::Void") . docs (& []))) . docs (& [])))
        }
    };
};
const _: () = {
    impl ::codec::MaxEncodedLen for OriginCaller {
        fn max_encoded_len() -> ::core::primitive::usize {
            0_usize . max (0_usize . saturating_add (< frame_system :: Origin < Runtime > > :: max_encoded_len ())) . max (0_usize . saturating_add (< self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: Void > :: max_encoded_len ())) . saturating_add (1)
        }
    }
};
#[allow(dead_code)]
impl Origin {
    /// Create with system none origin and [`frame_system::Config::BaseCallFilter`].
    pub fn none() -> Self {
        < Origin as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OriginTrait > :: none ()
    }
    /// Create with system root origin and [`frame_system::Config::BaseCallFilter`].
    pub fn root() -> Self {
        < Origin as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OriginTrait > :: root ()
    }
    /// Create with system signed origin and [`frame_system::Config::BaseCallFilter`].
    pub fn signed(by: <Runtime as frame_system::Config>::AccountId) -> Self {
        < Origin as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OriginTrait > :: signed (by)
    }
}
impl From<frame_system::Origin<Runtime>> for OriginCaller {
    fn from(x: frame_system::Origin<Runtime>) -> Self {
        OriginCaller::system(x)
    }
}
impl TryFrom<OriginCaller> for frame_system::Origin<Runtime> {
    type Error = OriginCaller;
    fn try_from(
        x: OriginCaller,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        frame_system::Origin<Runtime>,
        OriginCaller,
    > {
        if let OriginCaller::system(l) = x {
            Ok(l)
        } else {
            Err(x)
        }
    }
}
impl From<frame_system::Origin<Runtime>> for Origin {
    /// Convert to runtime origin, using as filter: [`frame_system::Config::BaseCallFilter`].
    fn from(x: frame_system::Origin<Runtime>) -> Self {
        let o: OriginCaller = x.into();
        o.into()
    }
}
impl From<OriginCaller> for Origin {
    fn from(x: OriginCaller) -> Self {
        let mut o = Origin {
            caller: x,
            filter:
                self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::rc::Rc::new(
                    Box::new(|_| true),
                ),
        };
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OriginTrait :: reset_filter (& mut o) ;
        o
    }
}
impl From<Origin>
    for self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::result::Result<
        frame_system::Origin<Runtime>,
        Origin,
    >
{
    /// NOTE: converting to pallet origin loses the origin filter information.
    fn from(val: Origin) -> Self {
        if let OriginCaller::system(l) = val.caller {
            Ok(l)
        } else {
            Err(val)
        }
    }
}
impl From<Option<<Runtime as frame_system::Config>::AccountId>> for Origin {
    /// Convert to runtime origin with caller being system signed or none and use filter [`frame_system::Config::BaseCallFilter`].
    fn from(x: Option<<Runtime as frame_system::Config>::AccountId>) -> Self {
        <frame_system::Origin<Runtime>>::from(x).into()
    }
}
pub type System = frame_system::Pallet<Runtime>;
pub type SimplePallet = simple_pallet::Pallet<Runtime>;
/// All pallets included in the runtime as a nested tuple of types.
#[deprecated(
    note = "The type definition has changed from representing all pallets \
			excluding system, in reversed order to become the representation of all pallets \
			including system pallet in regular order. For this reason it is encouraged to use \
			explicitly one of `AllPalletsWithSystem`, `AllPalletsWithoutSystem`, \
			`AllPalletsWithSystemReversed`, `AllPalletsWithoutSystemReversed`. \
			Note that the type `frame_executive::Executive` expects one of `AllPalletsWithSystem` \
			, `AllPalletsWithSystemReversed`, `AllPalletsReversedWithSystemFirst`. More details in \
			https://github.com/paritytech/substrate/pull/10043"
)]
pub type AllPallets = AllPalletsWithSystem;
/// All pallets included in the runtime as a nested tuple of types.
pub type AllPalletsWithSystem = ((System, (SimplePallet,)));
/// All pallets included in the runtime as a nested tuple of types.
/// Excludes the System pallet.
pub type AllPalletsWithoutSystem = ((SimplePallet,));
/// All pallets included in the runtime as a nested tuple of types in reversed order.
/// Excludes the System pallet.
pub type AllPalletsWithoutSystemReversed = ((SimplePallet,));
/// All pallets included in the runtime as a nested tuple of types in reversed order.
pub type AllPalletsWithSystemReversed = ((SimplePallet, (System,)));
/// All pallets included in the runtime as a nested tuple of types in reversed order.
/// With the system pallet first.
pub type AllPalletsReversedWithSystemFirst = (System, AllPalletsWithoutSystemReversed);
/// Provides an implementation of `PalletInfo` to provide information
/// about the pallet setup in the runtime.
pub struct PalletInfo;
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::PalletInfo
    for PalletInfo
{
    fn index<P: 'static>() -> Option<usize> {
        let type_id =
            self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                P,
            >();
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < System > () { return Some (0usize) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < SimplePallet > () { return Some (1usize) }
        None
    }
    fn name<P: 'static>() -> Option<&'static str> {
        let type_id =
            self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                P,
            >();
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < System > () { return Some ("System") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < SimplePallet > () { return Some ("SimplePallet") }
        None
    }
    fn module_name<P: 'static>() -> Option<&'static str> {
        let type_id =
            self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                P,
            >();
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < System > () { return Some ("frame_system") }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < SimplePallet > () { return Some ("simple_pallet") }
        None
    }
    fn crate_version<P: 'static>(
    ) -> Option<self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::CrateVersion>
    {
        let type_id =
            self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_std::any::TypeId::of::<
                P,
            >();
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < System > () { return Some (< frame_system :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: PalletInfoAccess > :: crate_version ()) }
        if type_id == self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: any :: TypeId :: of :: < SimplePallet > () { return Some (< simple_pallet :: Pallet < Runtime > as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: PalletInfoAccess > :: crate_version ()) }
        None
    }
}
pub enum Call {
    #[codec(index = 0u8)]
    System(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            System,
            Runtime,
        >,
    ),
    #[codec(index = 1u8)]
    SimplePallet(
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            SimplePallet,
            Runtime,
        >,
    ),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Call {
    #[inline]
    fn clone(&self) -> Call {
        match (&*self,) {
            (&Call::System(ref __self_0),) => Call::System(::core::clone::Clone::clone(&*__self_0)),
            (&Call::SimplePallet(ref __self_0),) => {
                Call::SimplePallet(::core::clone::Clone::clone(&*__self_0))
            }
        }
    }
}
impl ::core::marker::StructuralPartialEq for Call {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for Call {
    #[inline]
    fn eq(&self, other: &Call) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Call::System(ref __self_0), &Call::System(ref __arg_1_0)) => {
                        *__self_0 == *__arg_1_0
                    }
                    (&Call::SimplePallet(ref __self_0), &Call::SimplePallet(ref __arg_1_0)) => {
                        *__self_0 == *__arg_1_0
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &Call) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Call::System(ref __self_0), &Call::System(ref __arg_1_0)) => {
                        *__self_0 != *__arg_1_0
                    }
                    (&Call::SimplePallet(ref __self_0), &Call::SimplePallet(ref __arg_1_0)) => {
                        *__self_0 != *__arg_1_0
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}
impl ::core::marker::StructuralEq for Call {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for Call {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < System , Runtime > > ;
            let _ : :: core :: cmp :: AssertParamIsEq < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < SimplePallet , Runtime > > ;
        }
    }
}
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Encode for Call {
        fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            match *self {
                Call::System(ref aa) => {
                    __codec_dest_edqy.push_byte(0u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::SimplePallet(ref aa) => {
                    __codec_dest_edqy.push_byte(1u8 as ::core::primitive::u8);
                    ::codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                _ => (),
            }
        }
    }
    #[automatically_derived]
    impl ::codec::EncodeLike for Call {}
};
#[allow(deprecated)]
const _: () = {
    #[automatically_derived]
    impl ::codec::Decode for Call {
        fn decode<__CodecInputEdqy: ::codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> ::core::result::Result<Self, ::codec::Error> {
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| e.chain("Could not decode `Call`, failed to read variant byte"))?
            {
                __codec_x_edqy if __codec_x_edqy == 0u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::System({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < System , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::System.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 1u8 as ::core::primitive::u8 => {
                    ::core::result::Result::Ok(Call::SimplePallet({
                        let __codec_res_edqy = < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < SimplePallet , Runtime > as :: codec :: Decode > :: decode (__codec_input_edqy) ;
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `Call::SimplePallet.0`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                    "Could not decode `Call`, variant doesn't exist",
                )),
            }
        }
    }
};
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    impl ::scale_info::TypeInfo for Call {
        type Identity = Self;
        fn type_info() -> ::scale_info::Type {
            :: scale_info :: Type :: builder () . path (:: scale_info :: Path :: new ("Call" , "simple_runtime")) . type_params (:: alloc :: vec :: Vec :: new ()) . docs (& []) . variant (:: scale_info :: build :: Variants :: new () . variant ("System" , | v | v . index (0u8 as :: core :: primitive :: u8) . fields (:: scale_info :: build :: Fields :: unnamed () . field (| f | f . ty :: < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < System , Runtime > > () . type_name ("self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch\n::CallableCallFor<System, Runtime>") . docs (& []))) . docs (& [])) . variant ("SimplePallet" , | v | v . index (1u8 as :: core :: primitive :: u8) . fields (:: scale_info :: build :: Fields :: unnamed () . field (| f | f . ty :: < self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < SimplePallet , Runtime > > () . type_name ("self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch\n::CallableCallFor<SimplePallet, Runtime>") . docs (& []))) . docs (& [])))
        }
    };
};
impl core::fmt::Debug for Call {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::System(ref a0) => fmt.debug_tuple("Call::System").field(a0).finish(),
            Self::SimplePallet(ref a0) => fmt.debug_tuple("Call::SimplePallet").field(a0).finish(),
            _ => Ok(()),
        }
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::GetDispatchInfo
    for Call
{
    fn get_dispatch_info(
        &self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::DispatchInfo
    {
        match self {
            Call::System(call) => call.get_dispatch_info(),
            Call::SimplePallet(call) => call.get_dispatch_info(),
        }
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::GetCallMetadata
    for Call
{
    fn get_call_metadata(
        &self,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallMetadata
    {
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::GetCallName;
        match self {
            Call::System(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "System";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
            Call::SimplePallet(call) => {
                let function_name = call.get_call_name();
                let pallet_name = "SimplePallet";
                self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallMetadata { function_name , pallet_name , }
            }
        }
    }
    fn get_module_names() -> &'static [&'static str] {
        &["System", "SimplePallet"]
    }
    fn get_call_names(module: &str) -> &'static [&'static str] {
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::{
            Callable, GetCallName,
        };
        match module {
            "System" => <<System as Callable<Runtime>>::Call as GetCallName>::get_call_names(),
            "SimplePallet" => {
                <<SimplePallet as Callable<Runtime>>::Call as GetCallName>::get_call_names()
            }
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::Dispatchable
    for Call
{
    type Origin = Origin;
    type Config = Call;
    type Info =
        self::sp_api_hidden_includes_construct_runtime::hidden_include::weights::DispatchInfo;
    type PostInfo =
        self::sp_api_hidden_includes_construct_runtime::hidden_include::weights::PostDispatchInfo;    fn dispatch (self , origin : Origin) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: DispatchResultWithPostInfo{
        if ! < Self :: Origin as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OriginTrait > :: filter_call (& origin , & self) { return self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_std :: result :: Result :: Err (frame_system :: Error :: < Runtime > :: CallFiltered . into ()) ; }
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (self , origin)
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::UnfilteredDispatchable
    for Call
{
    type Origin = Origin;    fn dispatch_bypass_filter (self , origin : Origin) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: DispatchResultWithPostInfo{
        match self { Call :: System (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , Call :: SimplePallet (call) => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (call , origin) , }
    }
}
impl self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: DispatchableWithStorageLayer for Call { type Origin = Origin ; fn dispatch_with_storage_layer (self , origin : Origin) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: DispatchResultWithPostInfo { self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: storage :: with_storage_layer (| | { self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: Dispatchable :: dispatch (self , origin) }) } fn dispatch_bypass_filter_with_storage_layer (self , origin : Origin) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: DispatchResultWithPostInfo { self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: storage :: with_storage_layer (| | { self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: UnfilteredDispatchable :: dispatch_bypass_filter (self , origin) }) } }
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            System,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            System,
            Runtime,
        >,
    > {
        match self {
            Call::System(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            System,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < System , Runtime >,
    ) -> Self {
        Call::System(call)
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::IsSubType<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            SimplePallet,
            Runtime,
        >,
    > for Call
{
    #[allow(unreachable_patterns)]
    fn is_sub_type(
        &self,
    ) -> Option<
        &self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            SimplePallet,
            Runtime,
        >,
    > {
        match self {
            Call::SimplePallet(call) => Some(call),
            _ => None,
        }
    }
}
impl
    From<
        self::sp_api_hidden_includes_construct_runtime::hidden_include::dispatch::CallableCallFor<
            SimplePallet,
            Runtime,
        >,
    > for Call
{
    fn from(
        call : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: dispatch :: CallableCallFor < SimplePallet , Runtime >,
    ) -> Self {
        Call::SimplePallet(call)
    }
}
impl Runtime {
    pub fn metadata () -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: RuntimeMetadataPrefixed{
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: RuntimeMetadataLastVersion :: new (< [_] > :: into_vec (# [rustc_box] :: alloc :: boxed :: Box :: new ([self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: PalletMetadata { name : "System" , index : 0u8 , storage : Some (frame_system :: Pallet :: < Runtime > :: storage_metadata ()) , calls : Some (frame_system :: Pallet :: < Runtime > :: call_functions ()) , event : Some (self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: PalletEventMetadata { ty : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: scale_info :: meta_type :: < frame_system :: Event < Runtime > > () , }) , constants : frame_system :: Pallet :: < Runtime > :: pallet_constants_metadata () , error : frame_system :: Pallet :: < Runtime > :: error_metadata () , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: PalletMetadata { name : "SimplePallet" , index : 1u8 , storage : Some (simple_pallet :: Pallet :: < Runtime > :: storage_metadata ()) , calls : Some (simple_pallet :: Pallet :: < Runtime > :: call_functions ()) , event : None , constants : simple_pallet :: Pallet :: < Runtime > :: pallet_constants_metadata () , error : simple_pallet :: Pallet :: < Runtime > :: error_metadata () , }])) , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: ExtrinsicMetadata { ty : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: scale_info :: meta_type :: < UncheckedExtrinsic > () , version : < UncheckedExtrinsic as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: traits :: ExtrinsicMetadata > :: VERSION , signed_extensions : < < UncheckedExtrinsic as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: traits :: ExtrinsicMetadata > :: SignedExtensions as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: traits :: SignedExtension > :: metadata () . into_iter () . map (| meta | self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: metadata :: SignedExtensionMetadata { identifier : meta . identifier , ty : meta . ty , additional_signed : meta . additional_signed , }) . collect () , } , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: scale_info :: meta_type :: < Runtime > ()) . into ()
    }
}
#[cfg(any(feature = "std", test))]
pub type SystemConfig = frame_system::GenesisConfig;
#[cfg(any(feature = "std", test))]
use self::sp_api_hidden_includes_construct_runtime::hidden_include::serde as __genesis_config_serde_import__;
#[cfg(any(feature = "std", test))]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(crate = "__genesis_config_serde_import__")]
pub struct GenesisConfig {
    pub system: SystemConfig,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use __genesis_config_serde_import__ as _serde;
    #[automatically_derived]
    impl __genesis_config_serde_import__::Serialize for GenesisConfig {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> __genesis_config_serde_import__::__private::Result<__S::Ok, __S::Error>
        where
            __S: __genesis_config_serde_import__::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "GenesisConfig",
                false as usize + 1,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "system",
                &self.system,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use __genesis_config_serde_import__ as _serde;
    #[automatically_derived]
    impl<'de> __genesis_config_serde_import__::Deserialize<'de> for GenesisConfig {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> __genesis_config_serde_import__::__private::Result<Self, __D::Error>
        where
            __D: __genesis_config_serde_import__::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                            _serde::de::Unexpected::Unsigned(__value),
                            &"field index 0 <= i < 1",
                        )),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "system" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Err(_serde::de::Error::unknown_field(
                            __value, FIELDS,
                        )),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"system" => _serde::__private::Ok(__Field::__field0),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(_serde::de::Error::unknown_field(
                                __value, FIELDS,
                            ))
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<GenesisConfig>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = GenesisConfig;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct GenesisConfig")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 =
                        match match _serde::de::SeqAccess::next_element::<SystemConfig>(&mut __seq)
                        {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct GenesisConfig with 1 element",
                                ));
                            }
                        };
                    _serde::__private::Ok(GenesisConfig { system: __field0 })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<SystemConfig> =
                        _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "system",
                                        ),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<SystemConfig>(
                                        &mut __map,
                                    ) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("system") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(GenesisConfig { system: __field0 })
                }
            }
            const FIELDS: &'static [&'static str] = &["system"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "GenesisConfig",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<GenesisConfig>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::default::Default for GenesisConfig {
    #[inline]
    fn default() -> GenesisConfig {
        GenesisConfig {
            system: ::core::default::Default::default(),
        }
    }
}
#[cfg(any(feature = "std", test))]
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::BuildStorage
    for GenesisConfig
{
    fn assimilate_storage(
        &self,
        storage : & mut self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: Storage,
    ) -> std::result::Result<(), String> {
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: sp_runtime :: BuildModuleGenesisStorage :: < Runtime , frame_system :: __InherentHiddenInstance > :: build_module_genesis_storage (& self . system , storage) ? ;
        self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: BasicExternalities :: execute_with_storage (storage , | | { < AllPalletsWithSystem as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: traits :: OnGenesis > :: on_genesis () ; }) ;
        Ok(())
    }
}
trait InherentDataExt {
    fn create_extrinsics (& self) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: Vec < < Block as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: BlockT > :: Extrinsic > ;
    fn check_extrinsics (& self , block : & Block) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: CheckInherentsResult ;
}
impl InherentDataExt
    for self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::InherentData
{
    fn create_extrinsics (& self) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: Vec < < Block as self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: BlockT > :: Extrinsic >{
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::ProvideInherent;
        let mut inherents = Vec::new();
        inherents
    }    fn check_extrinsics (& self , block : & Block) -> self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: CheckInherentsResult{
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::{
            ProvideInherent, IsFatalError,
        };
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::{
            IsSubType, ExtrinsicCall,
        };
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::Block as _;
        let mut result = self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: CheckInherentsResult :: new () ;
        for xt in block.extrinsics() {
            if self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: Extrinsic :: is_signed (xt) . unwrap_or (false) { break }
            let mut is_inherent = false;
            if !is_inherent {
                break;
            }
        }
        result
    }
}
impl
    self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::EnsureInherentsAreFirst<
        Block,
    > for Runtime
{
    fn ensure_inherents_are_first(block: &Block) -> Result<(), u32> {
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::inherent::ProvideInherent;
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::traits::{
            IsSubType, ExtrinsicCall,
        };
        use self::sp_api_hidden_includes_construct_runtime::hidden_include::sp_runtime::traits::Block as _;
        let mut first_signed_observed = false;
        for (i, xt) in block.extrinsics().iter().enumerate() {
            let is_signed = self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: inherent :: Extrinsic :: is_signed (xt) . unwrap_or (false) ;
            let is_inherent = if is_signed {
                false
            } else {
                let mut is_inherent = false;
                is_inherent
            };
            if !is_inherent {
                first_signed_observed = true;
            }
            if first_signed_observed && is_inherent {
                return Err(i as u32);
            }
        }
        Ok(())
    }
}
impl self::sp_api_hidden_includes_construct_runtime::hidden_include::unsigned::ValidateUnsigned
    for Runtime
{
    type Call = Call;    fn pre_dispatch (call : & Self :: Call) -> Result < () , self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: unsigned :: TransactionValidityError >{
        #[allow(unreachable_patterns)]
        match call {
            _ => Ok(()),
        }
    }
    fn validate_unsigned(
        # [allow (unused_variables)] source : self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: unsigned :: TransactionSource,
        call: &Self::Call,
    ) -> self::sp_api_hidden_includes_construct_runtime::hidden_include::unsigned::TransactionValidity
    {
        # [allow (unreachable_patterns)] match call { _ => self :: sp_api_hidden_includes_construct_runtime :: hidden_include :: unsigned :: UnknownTransaction :: NoUnsignedValidator . into () , }
    }
}
const _: () =
    if !(<frame_system::Error<Runtime> as ::frame_support::traits::PalletError>::MAX_ENCODED_SIZE
        <= ::frame_support::MAX_MODULE_ERROR_ENCODED_SIZE)
    {
        :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["The maximum encoded size of the error type in the `System` pallet exceeds `MAX_MODULE_ERROR_ENCODED_SIZE`"] , & []))
    };
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
pub type SignedExtra = (
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckNonce<Runtime>,
);
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
pub type SignedPayload = generic::SignedPayload<Call, SignedExtra>;
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllPalletsWithSystem,
>;
#[doc(hidden)]
mod sp_api_hidden_includes_IMPL_RUNTIME_APIS {
    pub extern crate sp_api as sp_api;
}
pub struct RuntimeApi {}
/// Implements all runtime apis for the client side.
#[cfg(any(feature = "std", test))]
pub struct RuntimeApiImpl<
    Block: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT,
    C: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<Block> + 'static,
> {
    call: &'static C,
    commit_on_success: std::cell::RefCell<bool>,
    changes: std::cell::RefCell<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::OverlayedChanges,
    >,
    storage_transaction_cache: std::cell::RefCell<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StorageTransactionCache<
            Block,
            C::StateBackend,
        >,
    >,
    recorder: std::option::Option<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ProofRecorder<Block>,
    >,
}
#[cfg(any(feature = "std", test))]
unsafe impl<
        Block: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT,
        C: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<Block>,
    > Send for RuntimeApiImpl<Block, C>
{
}
#[cfg(any(feature = "std", test))]
unsafe impl<
        Block: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT,
        C: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<Block>,
    > Sync for RuntimeApiImpl<Block, C>
{
}
#[cfg(any(feature = "std", test))]
impl<
        Block: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT,
        C: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<Block>,
    > self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiExt<Block>
    for RuntimeApiImpl<Block, C>
{
    type StateBackend = C::StateBackend;
    fn execute_in_transaction<
        F: FnOnce(
            &Self,
        )
            -> self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::TransactionOutcome<R>,
        R,
    >(
        &self,
        call: F,
    ) -> R
    where
        Self: Sized,
    {
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::OverlayedChanges::start_transaction(
            &mut std::cell::RefCell::borrow_mut(&self.changes),
        );
        *std::cell::RefCell::borrow_mut(&self.commit_on_success) = false;
        let res = call(self);
        *std::cell::RefCell::borrow_mut(&self.commit_on_success) = true;
        self.commit_or_rollback(match res {
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::TransactionOutcome::Commit(
                _,
            ) => true,
            _ => false,
        });
        res.into_inner()
    }
    fn has_api<A: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::RuntimeApiInfo + ?Sized>(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<Block>,
    ) -> std::result::Result<bool, self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError>
    where
        Self: Sized,
    {
        self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: CallApiAt :: < Block > :: runtime_version_at (self . call , at) . map (| v | self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: RuntimeVersion :: has_api_with (& v , & A :: ID , | v | v == A :: VERSION))
    }
    fn has_api_with<
        A: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::RuntimeApiInfo + ?Sized,
        P: Fn(u32) -> bool,
    >(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<Block>,
        pred: P,
    ) -> std::result::Result<bool, self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError>
    where
        Self: Sized,
    {
        self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: CallApiAt :: < Block > :: runtime_version_at (self . call , at) . map (| v | self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: RuntimeVersion :: has_api_with (& v , & A :: ID , pred))
    }
    fn api_version<
        A: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::RuntimeApiInfo + ?Sized,
    >(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<Block>,
    ) -> std::result::Result<
        Option<u32>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    >
    where
        Self: Sized,
    {
        self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: CallApiAt :: < Block > :: runtime_version_at (self . call , at) . map (| v | self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: RuntimeVersion :: api_version (& v , & A :: ID))
    }
    fn record_proof(&mut self) {
        self.recorder = std::option::Option::Some(std::default::Default::default());
    }
    fn proof_recorder(
        &self,
    ) -> std::option::Option<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ProofRecorder<Block>,
    > {
        std::clone::Clone::clone(&self.recorder)
    }
    fn extract_proof(
        &mut self,
    ) -> std::option::Option<self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StorageProof>
    {
        std :: option :: Option :: take (& mut self . recorder) . map (| recorder | self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: ProofRecorder :: < Block > :: to_storage_proof (& recorder))
    }
    fn into_storage_changes(
        &self,
        backend: &Self::StateBackend,
        parent_hash: Block::Hash,
    ) -> core::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StorageChanges<
            C::StateBackend,
            Block,
        >,
        String,
    >
    where
        Self: Sized,
    {
        let at = self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId::Hash(
            std::clone::Clone::clone(&parent_hash),
        );
        let state_version = self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt::<
            Block,
        >::runtime_version_at(self.call, &at)
        .map(|v| {
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::RuntimeVersion::state_version(
                &v,
            )
        })
        .map_err(|e| {
            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                &["Failed to get state version: "],
                &[::core::fmt::ArgumentV1::new_display(&e)],
            ));
            res
        })?;
        self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: OverlayedChanges :: into_storage_changes (std :: cell :: RefCell :: take (& self . changes) , backend , core :: cell :: RefCell :: take (& self . storage_transaction_cache) , state_version)
    }
}
#[cfg(any(feature = "std", test))]
impl<Block: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT, C>
    self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ConstructRuntimeApi<Block, C>
    for RuntimeApi
where
    C: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<Block> + 'static,
{
    type RuntimeApi = RuntimeApiImpl<Block, C>;
    fn construct_runtime_api<'a>(
        call: &'a C,
    ) -> self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiRef<'a, Self::RuntimeApi> {
        RuntimeApiImpl {
            call: unsafe { std::mem::transmute(call) },
            commit_on_success: true.into(),
            changes: std::default::Default::default(),
            recorder: std::default::Default::default(),
            storage_transaction_cache: std::default::Default::default(),
        }
        .into()
    }
}
#[cfg(any(feature = "std", test))]
impl<
        Block: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT,
        C: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<Block>,
    > RuntimeApiImpl<Block, C>
{
    fn call_api_at<
        R: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode
            + self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Decode
            + std::cmp::PartialEq,
        F: FnOnce(
            &C,
            &std::cell::RefCell<
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::OverlayedChanges,
            >,
            &std::cell::RefCell<
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StorageTransactionCache<
                    Block,
                    C::StateBackend,
                >,
            >,
            &std::option::Option<
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ProofRecorder<Block>,
            >,
        ) -> std::result::Result<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<R>,
            E,
        >,
        E,
    >(
        &self,
        call_api_at: F,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<R>,
        E,
    > {
        if *std::cell::RefCell::borrow(&self.commit_on_success) {
            self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: OverlayedChanges :: start_transaction (& mut std :: cell :: RefCell :: borrow_mut (& self . changes)) ;
        }
        let res = call_api_at(
            &self.call,
            &self.changes,
            &self.storage_transaction_cache,
            &self.recorder,
        );
        self.commit_or_rollback(std::result::Result::is_ok(&res));
        res
    }
    fn commit_or_rollback(&self, commit: bool) {
        let proof = "\
					We only close a transaction when we opened one ourself.
					Other parts of the runtime that make use of transactions (state-machine)
					also balance their transactions. The runtime cannot close client initiated
					transactions; qed";
        if *std::cell::RefCell::borrow(&self.commit_on_success) {
            let res = if commit {
                self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: OverlayedChanges :: commit_transaction (& mut std :: cell :: RefCell :: borrow_mut (& self . changes))
            } else {
                self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: OverlayedChanges :: rollback_transaction (& mut std :: cell :: RefCell :: borrow_mut (& self . changes))
            };
            std::result::Result::expect(res, proof);
        }
    }
}
impl sp_api::runtime_decl_for_Core::Core<Block> for Runtime {
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
impl sp_block_builder::runtime_decl_for_BlockBuilder::BlockBuilder<Block> for Runtime {
    fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
        Executive::apply_extrinsic(extrinsic)
    }
    fn finalize_block() -> <Block as BlockT>::Header {
        Executive::finalize_block()
    }
    fn inherent_extrinsics(_: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
        ::core::panicking::panic("not implemented")
    }
    fn check_inherents(
        _: Block,
        _: sp_inherents::InherentData,
    ) -> sp_inherents::CheckInherentsResult {
        ::core::panicking::panic("not implemented")
    }
}
impl sp_transaction_pool :: runtime_api :: runtime_decl_for_TaggedTransactionQueue :: TaggedTransactionQueue < Block > for Runtime { fn validate_transaction (source : TransactionSource , tx : < Block as BlockT > :: Extrinsic , block_hash : < Block as BlockT > :: Hash) -> TransactionValidity { Executive :: validate_transaction (source , tx , block_hash) } }
#[cfg(any(feature = "std", test))]
impl<
        __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
            + std::panic::UnwindSafe
            + std::panic::RefUnwindSafe,
        RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
            + 'static,
    > sp_api::Core<__SR_API_BLOCK__> for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
    RuntimeVersion: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    <__SR_API_BLOCK__ as BlockT>::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn Core_version_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<RuntimeVersion>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self.call_api_at(
            |call_runtime_at, changes, storage_transaction_cache, recorder| {
                sp_api::runtime_decl_for_Core::version_call_api_at(
                    call_runtime_at,
                    at,
                    params_encoded,
                    changes,
                    storage_transaction_cache,
                    params.map(|p| {
                        sp_api::runtime_decl_for_Core::version_native_call_generator::<
                            Runtime,
                            __SR_API_BLOCK__,
                            Block,
                        >()
                    }),
                    context,
                    recorder,
                )
            },
        )
    }
    fn Core_execute_block_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(__SR_API_BLOCK__)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<()>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self.call_api_at(
            |call_runtime_at, changes, storage_transaction_cache, recorder| {
                sp_api::runtime_decl_for_Core::execute_block_call_api_at(
                    call_runtime_at,
                    at,
                    params_encoded,
                    changes,
                    storage_transaction_cache,
                    params.map(|p| {
                        sp_api::runtime_decl_for_Core::execute_block_native_call_generator::<
                            Runtime,
                            __SR_API_BLOCK__,
                            Block,
                        >(p)
                    }),
                    context,
                    recorder,
                )
            },
        )
    }
    fn Core_initialize_block_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(&<__SR_API_BLOCK__ as BlockT>::Header)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<()>,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self.call_api_at(
            |call_runtime_at, changes, storage_transaction_cache, recorder| {
                sp_api::runtime_decl_for_Core::initialize_block_call_api_at(
                    call_runtime_at,
                    at,
                    params_encoded,
                    changes,
                    storage_transaction_cache,
                    params.map(|p| {
                        sp_api::runtime_decl_for_Core::initialize_block_native_call_generator::<
                            Runtime,
                            __SR_API_BLOCK__,
                            Block,
                        >(p)
                    }),
                    context,
                    recorder,
                )
            },
        )
    }
}
#[cfg(any(feature = "std", test))]
impl<
        __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
            + std::panic::UnwindSafe
            + std::panic::RefUnwindSafe,
        RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
            + 'static,
    > sp_block_builder::BlockBuilder<__SR_API_BLOCK__>
    for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
    <__SR_API_BLOCK__ as BlockT>::Extrinsic: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    ApplyExtrinsicResult: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    <__SR_API_BLOCK__ as BlockT>::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    sp_inherents::InherentData: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    Vec<<__SR_API_BLOCK__ as BlockT>::Extrinsic>:
        std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    sp_inherents::InherentData: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    sp_inherents::CheckInherentsResult: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn BlockBuilder_apply_extrinsic_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(<__SR_API_BLOCK__ as BlockT>::Extrinsic)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            ApplyExtrinsicResult,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { sp_block_builder :: runtime_decl_for_BlockBuilder :: apply_extrinsic_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { sp_block_builder :: runtime_decl_for_BlockBuilder :: apply_extrinsic_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p) }) , context , recorder) })
    }
    fn BlockBuilder_finalize_block_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<()>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            <__SR_API_BLOCK__ as BlockT>::Header,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { sp_block_builder :: runtime_decl_for_BlockBuilder :: finalize_block_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { sp_block_builder :: runtime_decl_for_BlockBuilder :: finalize_block_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > () }) , context , recorder) })
    }
    fn BlockBuilder_inherent_extrinsics_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(sp_inherents::InherentData)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            Vec<<__SR_API_BLOCK__ as BlockT>::Extrinsic>,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { sp_block_builder :: runtime_decl_for_BlockBuilder :: inherent_extrinsics_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { sp_block_builder :: runtime_decl_for_BlockBuilder :: inherent_extrinsics_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p) }) , context , recorder) })
    }
    fn BlockBuilder_check_inherents_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(__SR_API_BLOCK__, sp_inherents::InherentData)>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            sp_inherents::CheckInherentsResult,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { sp_block_builder :: runtime_decl_for_BlockBuilder :: check_inherents_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { sp_block_builder :: runtime_decl_for_BlockBuilder :: check_inherents_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p . 0 , p . 1) }) , context , recorder) })
    }
}
#[cfg(any(feature = "std", test))]
impl<
        __SR_API_BLOCK__: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockT
            + std::panic::UnwindSafe
            + std::panic::RefUnwindSafe,
        RuntimeApiImplCall: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::CallApiAt<__SR_API_BLOCK__>
            + 'static,
    > sp_transaction_pool::runtime_api::TaggedTransactionQueue<__SR_API_BLOCK__>
    for RuntimeApiImpl<__SR_API_BLOCK__, RuntimeApiImplCall>
where
    RuntimeApiImplCall::StateBackend:
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::StateBackend<
            self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::HashFor<__SR_API_BLOCK__>,
        >,
    TransactionSource: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    <__SR_API_BLOCK__ as BlockT>::Extrinsic: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    <__SR_API_BLOCK__ as BlockT>::Hash: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    TransactionValidity: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    __SR_API_BLOCK__::Header: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn TaggedTransactionQueue_validate_transaction_runtime_api_impl(
        &self,
        at: &self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::BlockId<__SR_API_BLOCK__>,
        context: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ExecutionContext,
        params: Option<(
            TransactionSource,
            <__SR_API_BLOCK__ as BlockT>::Extrinsic,
            <__SR_API_BLOCK__ as BlockT>::Hash,
        )>,
        params_encoded: Vec<u8>,
    ) -> std::result::Result<
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::NativeOrEncoded<
            TransactionValidity,
        >,
        self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApiError,
    > {
        self . call_api_at (| call_runtime_at , changes , storage_transaction_cache , recorder | { sp_transaction_pool :: runtime_api :: runtime_decl_for_TaggedTransactionQueue :: validate_transaction_call_api_at (call_runtime_at , at , params_encoded , changes , storage_transaction_cache , params . map (| p | { sp_transaction_pool :: runtime_api :: runtime_decl_for_TaggedTransactionQueue :: validate_transaction_native_call_generator :: < Runtime , __SR_API_BLOCK__ , Block > (p . 0 , p . 1 , p . 2) }) , context , recorder) })
    }
}
const RUNTIME_API_VERSIONS: self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::ApisVec =
    ::sp_version::sp_std::borrow::Cow::Borrowed(&[
        (
            sp_api::runtime_decl_for_Core::ID,
            sp_api::runtime_decl_for_Core::VERSION,
        ),
        (
            sp_block_builder::runtime_decl_for_BlockBuilder::ID,
            sp_block_builder::runtime_decl_for_BlockBuilder::VERSION,
        ),
        (
            sp_transaction_pool::runtime_api::runtime_decl_for_TaggedTransactionQueue::ID,
            sp_transaction_pool::runtime_api::runtime_decl_for_TaggedTransactionQueue::VERSION,
        ),
    ]);
const _: () = {};
const _: () = {};
const _: () = {};
pub mod api {
    use super::*;
    #[cfg(feature = "std")]
    pub fn dispatch(method: &str, mut __sp_api__input_data: &[u8]) -> Option<Vec<u8>> {
        match method {
            "Core_version" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let () : () = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "version") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    #[allow(deprecated)]
                    <Runtime as sp_api::runtime_decl_for_Core::Core<Block>>::version()
                }),
            ),
            "Core_execute_block" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (block) : (Block) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "execute_block") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    #[allow(deprecated)]
                    <Runtime as sp_api::runtime_decl_for_Core::Core<Block>>::execute_block(block)
                }),
            ),
            "Core_initialize_block" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (header) : (< Block as BlockT > :: Header) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "initialize_block") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    #[allow(deprecated)]
                    <Runtime as sp_api::runtime_decl_for_Core::Core<Block>>::initialize_block(
                        &header,
                    )
                }),
            ),
            "BlockBuilder_apply_extrinsic" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (extrinsic) : (< Block as BlockT > :: Extrinsic) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "apply_extrinsic") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    #[allow(deprecated)]
                    <Runtime as sp_block_builder::runtime_decl_for_BlockBuilder::BlockBuilder<
                        Block,
                    >>::apply_extrinsic(extrinsic)
                }),
            ),
            "BlockBuilder_finalize_block" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let () : () = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "finalize_block") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    #[allow(deprecated)]
                    <Runtime as sp_block_builder::runtime_decl_for_BlockBuilder::BlockBuilder<
                        Block,
                    >>::finalize_block()
                }),
            ),
            "BlockBuilder_inherent_extrinsics" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (__runtime_api_generated_name_0__) : (sp_inherents :: InherentData) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "inherent_extrinsics") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    #[allow(deprecated)]
                    <Runtime as sp_block_builder::runtime_decl_for_BlockBuilder::BlockBuilder<
                        Block,
                    >>::inherent_extrinsics(__runtime_api_generated_name_0__)
                }),
            ),
            "BlockBuilder_check_inherents" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (__runtime_api_generated_name_0__ , __runtime_api_generated_name_1__) : (Block , sp_inherents :: InherentData) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "check_inherents") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    #[allow(deprecated)]
                    <Runtime as sp_block_builder::runtime_decl_for_BlockBuilder::BlockBuilder<
                        Block,
                    >>::check_inherents(
                        __runtime_api_generated_name_0__,
                        __runtime_api_generated_name_1__,
                    )
                }),
            ),
            "TaggedTransactionQueue_validate_transaction" => Some(
                self::sp_api_hidden_includes_IMPL_RUNTIME_APIS::sp_api::Encode::encode(&{
                    let (source , tx , block_hash) : (TransactionSource , < Block as BlockT > :: Extrinsic , < Block as BlockT > :: Hash) = match self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: DecodeLimit :: decode_all_with_depth_limit (self :: sp_api_hidden_includes_IMPL_RUNTIME_APIS :: sp_api :: MAX_EXTRINSIC_DEPTH , & mut __sp_api__input_data) { Ok (res) => res , Err (e) => :: core :: panicking :: panic_fmt (:: core :: fmt :: Arguments :: new_v1 (& ["Bad input data provided to " , ": "] , & [:: core :: fmt :: ArgumentV1 :: new_display (& "validate_transaction") , :: core :: fmt :: ArgumentV1 :: new_display (& e)])) , } ;
                    # [allow (deprecated)] < Runtime as sp_transaction_pool :: runtime_api :: runtime_decl_for_TaggedTransactionQueue :: TaggedTransactionQueue < Block > > :: validate_transaction (source , tx , block_hash)
                }),
            ),
            _ => None,
        }
    }
}
