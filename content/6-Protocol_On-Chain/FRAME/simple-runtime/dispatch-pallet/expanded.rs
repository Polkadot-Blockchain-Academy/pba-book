#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub use pallet::*;
pub struct WithDebug {
    foo: u32,
    bar: u32,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for WithDebug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "WithDebug",
            "foo",
            &&self.foo,
            "bar",
            &&self.bar,
        )
    }
}
pub struct WithRuntimeDebug {
    foo: u32,
    bar: u32,
}
impl core::fmt::Debug for WithRuntimeDebug {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_struct("WithRuntimeDebug")
            .field("foo", &self.foo)
            .field("bar", &self.bar)
            .finish()
    }
}
///
///			The module that hosts all the
///			[FRAME](https://docs.substrate.io/v3/runtime/frame)
///			types needed to add this pallet to a
///			runtime.
///
pub mod pallet {
    use codec::{Encode, Decode};
    use frame_support::{
        pallet_prelude::*,
        dispatch::{DispatchResultWithPostInfo, PostDispatchInfo},
        weights::Pays,
    };
    use frame_system::pallet_prelude::*;
    ///
    ///			Configuration trait of this pallet.
    ///
    ///			Implement this type for a runtime in order to customize this pallet.
    ///
    pub trait Config: frame_system::Config {}
    ///
    ///			The [pallet](https://docs.substrate.io/v3/runtime/frame#pallets) implementing
    ///			the on-chain logic.
    ///
    pub struct Pallet<T>(PhantomData<T>);
    const _: () = {
        impl<T> core::clone::Clone for Pallet<T> {
            fn clone(&self) -> Self {
                Self(core::clone::Clone::clone(&self.0))
            }
        }
    };
    const _: () = {
        impl<T> core::cmp::Eq for Pallet<T> {}
    };
    const _: () = {
        impl<T> core::cmp::PartialEq for Pallet<T> {
            fn eq(&self, other: &Self) -> bool {
                true && self.0 == other.0
            }
        }
    };
    const _: () = {
        impl<T> core::fmt::Debug for Pallet<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_tuple("Pallet").field(&self.0).finish()
            }
        }
    };
    pub struct TransferConfig<AccountId> {
        from: AccountId,
        to: AccountId,
        amount: u64,
    }
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<AccountId> ::codec::Encode for TransferConfig<AccountId>
        where
            AccountId: ::codec::Encode,
            AccountId: ::codec::Encode,
            AccountId: ::codec::Encode,
            AccountId: ::codec::Encode,
        {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                ::codec::Encode::encode_to(&self.from, __codec_dest_edqy);
                ::codec::Encode::encode_to(&self.to, __codec_dest_edqy);
                ::codec::Encode::encode_to(&self.amount, __codec_dest_edqy);
            }
        }
        #[automatically_derived]
        impl<AccountId> ::codec::EncodeLike for TransferConfig<AccountId>
        where
            AccountId: ::codec::Encode,
            AccountId: ::codec::Encode,
            AccountId: ::codec::Encode,
            AccountId: ::codec::Encode,
        {
        }
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<AccountId> ::codec::Decode for TransferConfig<AccountId>
        where
            AccountId: ::codec::Decode,
            AccountId: ::codec::Decode,
            AccountId: ::codec::Decode,
            AccountId: ::codec::Decode,
        {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                ::core::result::Result::Ok(TransferConfig::<AccountId> {
                    from: {
                        let __codec_res_edqy =
                            <AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `TransferConfig::from`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                    to: {
                        let __codec_res_edqy =
                            <AccountId as ::codec::Decode>::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `TransferConfig::to`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                    amount: {
                        let __codec_res_edqy = <u64 as ::codec::Decode>::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            ::core::result::Result::Err(e) => {
                                return ::core::result::Result::Err(
                                    e.chain("Could not decode `TransferConfig::amount`"),
                                )
                            }
                            ::core::result::Result::Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    },
                })
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<AccountId: ::core::fmt::Debug> ::core::fmt::Debug for TransferConfig<AccountId> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "TransferConfig",
                "from",
                &&self.from,
                "to",
                &&self.to,
                "amount",
                &&self.amount,
            )
        }
    }
    impl<AccountId> ::core::marker::StructuralPartialEq for TransferConfig<AccountId> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<AccountId: ::core::cmp::PartialEq> ::core::cmp::PartialEq for TransferConfig<AccountId> {
        #[inline]
        fn eq(&self, other: &TransferConfig<AccountId>) -> bool {
            self.from == other.from && self.to == other.to && self.amount == other.amount
        }
        #[inline]
        fn ne(&self, other: &TransferConfig<AccountId>) -> bool {
            self.from != other.from || self.to != other.to || self.amount != other.amount
        }
    }
    impl<AccountId> ::core::marker::StructuralEq for TransferConfig<AccountId> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<AccountId: ::core::cmp::Eq> ::core::cmp::Eq for TransferConfig<AccountId> {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<AccountId>;
                let _: ::core::cmp::AssertParamIsEq<AccountId>;
                let _: ::core::cmp::AssertParamIsEq<u64>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<AccountId: ::core::clone::Clone> ::core::clone::Clone for TransferConfig<AccountId> {
        #[inline]
        fn clone(&self) -> TransferConfig<AccountId> {
            TransferConfig {
                from: ::core::clone::Clone::clone(&self.from),
                to: ::core::clone::Clone::clone(&self.to),
                amount: ::core::clone::Clone::clone(&self.amount),
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl<AccountId> ::scale_info::TypeInfo for TransferConfig<AccountId>
        where
            AccountId: ::scale_info::TypeInfo + 'static,
            AccountId: ::scale_info::TypeInfo + 'static,
            AccountId: ::scale_info::TypeInfo + 'static,
        {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new(
                        "TransferConfig",
                        "dispatch_pallet::pallet",
                    ))
                    .type_params(<[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([::scale_info::TypeParameter::new(
                            "AccountId",
                            ::core::option::Option::Some(::scale_info::meta_type::<AccountId>()),
                        )]),
                    ))
                    .docs(&[])
                    .composite(
                        ::scale_info::build::Fields::named()
                            .field(|f| {
                                f.ty::<AccountId>()
                                    .name("from")
                                    .type_name("AccountId")
                                    .docs(&[])
                            })
                            .field(|f| {
                                f.ty::<AccountId>()
                                    .name("to")
                                    .type_name("AccountId")
                                    .docs(&[])
                            })
                            .field(|f| f.ty::<u64>().name("amount").type_name("u64").docs(&[])),
                    )
            }
        };
    };
    impl<T: Config> Pallet<T> {
        pub fn first_transaction(_origin: OriginFor<T>, _inc: u32) -> DispatchResult {
            Ok(())
        }
        pub fn transfer(
            _origin: OriginFor<T>,
            config: TransferConfig<T::AccountId>,
        ) -> DispatchResultWithPostInfo {
            if config.amount == 42 {
                Ok(PostDispatchInfo {
                    actual_weight: Some(0),
                    pays_fee: Pays::No,
                })
            } else {
                Ok(None.into())
            }
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn pallet_constants_metadata(
        ) -> frame_support::sp_std::vec::Vec<frame_support::metadata::PalletConstantMetadata>
        {
            ::alloc::vec::Vec::new()
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn error_metadata() -> Option<frame_support::metadata::PalletErrorMetadata> {
            None
        }
    }
    /// Type alias to `Pallet`, to be used by `construct_runtime`.
    ///
    /// Generated by `pallet` attribute macro.
    #[deprecated(note = "use `Pallet` instead")]
    #[allow(dead_code)]
    pub type Module<T> = Pallet<T>;
    impl<T: Config> frame_support::traits::GetStorageVersion for Pallet<T> {
        fn current_storage_version() -> frame_support::traits::StorageVersion {
            frame_support::traits::StorageVersion::default()
        }
        fn on_chain_storage_version() -> frame_support::traits::StorageVersion {
            frame_support::traits::StorageVersion::get::<Self>()
        }
    }
    impl<T: Config> frame_support::traits::OnGenesis for Pallet<T> {
        fn on_genesis() {
            let storage_version = frame_support::traits::StorageVersion::default();
            storage_version.put::<Self>();
        }
    }
    impl<T: Config> frame_support::traits::PalletInfoAccess for Pallet<T> {
        fn index() -> usize {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::index::<
                Self,
            >()
            .expect(
                "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
            )
        }
        fn name() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Self,
            >()
            .expect(
                "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
            )
        }
        fn module_name() -> &'static str {
            < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: module_name :: < Self > () . expect ("Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime")
        }
        fn crate_version() -> frame_support::traits::CrateVersion {
            frame_support::traits::CrateVersion {
                major: 0u16,
                minor: 1u8,
                patch: 0u8,
            }
        }
    }
    impl<T: Config> frame_support::traits::PalletsInfoAccess for Pallet<T> {
        fn count() -> usize {
            1
        }
        fn accumulate(
            acc: &mut frame_support::sp_std::vec::Vec<frame_support::traits::PalletInfoData>,
        ) {
            use frame_support::traits::PalletInfoAccess;
            let item = frame_support::traits::PalletInfoData {
                index: Self::index(),
                name: Self::name(),
                module_name: Self::module_name(),
                crate_version: Self::crate_version(),
            };
            acc.push(item);
        }
    }
    impl<T: Config> frame_support::traits::StorageInfoTrait for Pallet<T> {
        fn storage_info() -> frame_support::sp_std::vec::Vec<frame_support::traits::StorageInfo> {
            #[allow(unused_mut)]
            let mut res = ::alloc::vec::Vec::new();
            res
        }
    }
    #[doc(hidden)]
    pub mod __substrate_call_check {
        #[doc(hidden)]
        pub use __is_call_part_defined_0 as is_call_part_defined;
    }
    ///Contains one variant per dispatchable that can be called by an extrinsic.
    #[codec(encode_bound())]
    #[codec(decode_bound())]
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    #[allow(non_camel_case_types)]
    pub enum Call<T: Config> {
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(
            frame_support::sp_std::marker::PhantomData<(T,)>,
            frame_support::Never,
        ),
        #[codec(index = 0u8)]
        first_transaction {
            #[allow(missing_docs)]
            inc: u32,
        },
        #[codec(index = 1u8)]
        transfer {
            #[allow(missing_docs)]
            config: TransferConfig<T::AccountId>,
        },
    }
    const _: () = {
        impl<T: Config> core::fmt::Debug for Call<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    Self::__Ignore(ref _0, ref _1) => fmt
                        .debug_tuple("Call::__Ignore")
                        .field(&_0)
                        .field(&_1)
                        .finish(),
                    Self::first_transaction { ref inc } => fmt
                        .debug_struct("Call::first_transaction")
                        .field("inc", &inc)
                        .finish(),
                    Self::transfer { ref config } => fmt
                        .debug_struct("Call::transfer")
                        .field("config", &config)
                        .finish(),
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::clone::Clone for Call<T> {
            fn clone(&self) -> Self {
                match self {
                    Self::__Ignore(ref _0, ref _1) => {
                        Self::__Ignore(core::clone::Clone::clone(_0), core::clone::Clone::clone(_1))
                    }
                    Self::first_transaction { ref inc } => Self::first_transaction {
                        inc: core::clone::Clone::clone(inc),
                    },
                    Self::transfer { ref config } => Self::transfer {
                        config: core::clone::Clone::clone(config),
                    },
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::cmp::Eq for Call<T> {}
    };
    const _: () = {
        impl<T: Config> core::cmp::PartialEq for Call<T> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::first_transaction { inc }, Self::first_transaction { inc: _0 }) => {
                        true && inc == _0
                    }
                    (Self::transfer { config }, Self::transfer { config: _0 }) => {
                        true && config == _0
                    }
                    (Self::__Ignore { .. }, Self::first_transaction { .. }) => false,
                    (Self::__Ignore { .. }, Self::transfer { .. }) => false,
                    (Self::first_transaction { .. }, Self::__Ignore { .. }) => false,
                    (Self::first_transaction { .. }, Self::transfer { .. }) => false,
                    (Self::transfer { .. }, Self::__Ignore { .. }) => false,
                    (Self::transfer { .. }, Self::first_transaction { .. }) => false,
                }
            }
        }
    };
    #[allow(deprecated)]
    const _: () = {
        #[allow(non_camel_case_types)]
        #[automatically_derived]
        impl<T: Config> ::codec::Encode for Call<T> {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Call::first_transaction { ref inc } => {
                        __codec_dest_edqy.push_byte(0u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(inc, __codec_dest_edqy);
                    }
                    Call::transfer { ref config } => {
                        __codec_dest_edqy.push_byte(1u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(config, __codec_dest_edqy);
                    }
                    _ => (),
                }
            }
        }
        #[automatically_derived]
        impl<T: Config> ::codec::EncodeLike for Call<T> {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[allow(non_camel_case_types)]
        #[automatically_derived]
        impl<T: Config> ::codec::Decode for Call<T> {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| e.chain("Could not decode `Call`, failed to read variant byte"))?
                {
                    __codec_x_edqy if __codec_x_edqy == 0u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::<T>::first_transaction {
                            inc: {
                                let __codec_res_edqy =
                                    <u32 as ::codec::Decode>::decode(__codec_input_edqy);
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(e.chain(
                                            "Could not decode `Call::first_transaction::inc`",
                                        ))
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        })
                    }
                    __codec_x_edqy if __codec_x_edqy == 1u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::<T>::transfer {
                            config: {
                                let __codec_res_edqy =
                                    <TransferConfig<T::AccountId> as ::codec::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::transfer::config`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        })
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
        impl<T: Config> ::scale_info::TypeInfo for Call<T>
        where
            frame_support::sp_std::marker::PhantomData<(T,)>: ::scale_info::TypeInfo + 'static,
            TransferConfig<T::AccountId>: ::scale_info::TypeInfo + 'static,
            T: Config + 'static,
        {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new("Call", "dispatch_pallet::pallet"))
                    .type_params(<[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([::scale_info::TypeParameter::new(
                            "T",
                            ::core::option::Option::None,
                        )]),
                    ))
                    .docs_always(&[
                        "Contains one variant per dispatchable that can be called by an extrinsic.",
                    ])
                    .variant(
                        ::scale_info::build::Variants::new()
                            .variant("first_transaction", |v| {
                                v.index(0u8 as ::core::primitive::u8)
                                    .fields(::scale_info::build::Fields::named().field(|f| {
                                        f.ty::<u32>().name("inc").type_name("u32").docs_always(&[])
                                    }))
                                    .docs_always(&[])
                            })
                            .variant("transfer", |v| {
                                v.index(1u8 as ::core::primitive::u8)
                                    .fields(::scale_info::build::Fields::named().field(|f| {
                                        f.ty::<TransferConfig<T::AccountId>>()
                                            .name("config")
                                            .type_name("TransferConfig<T::AccountId>")
                                            .docs_always(&[])
                                    }))
                                    .docs_always(&[])
                            }),
                    )
            }
        };
    };
    impl<T: Config> Call<T> {
        ///Create a call with the variant `first_transaction`.
        pub fn new_call_variant_first_transaction(inc: u32) -> Self {
            Self::first_transaction { inc }
        }
        ///Create a call with the variant `transfer`.
        pub fn new_call_variant_transfer(config: TransferConfig<T::AccountId>) -> Self {
            Self::transfer { config }
        }
    }
    impl<T: Config> frame_support::dispatch::GetDispatchInfo for Call<T> {
        fn get_dispatch_info(&self) -> frame_support::dispatch::DispatchInfo {
            match *self {
                Self::first_transaction { inc: ref _inc } => {
                    let __pallet_base_weight = 42;
                    let __pallet_weight =
                        <dyn frame_support::dispatch::WeighData<(&u32,)>>::weigh_data(
                            &__pallet_base_weight,
                            (_inc,),
                        );
                    let __pallet_class =
                        <dyn frame_support::dispatch::ClassifyDispatch<(&u32,)>>::classify_dispatch(
                            &__pallet_base_weight,
                            (_inc,),
                        );
                    let __pallet_pays_fee =
                        <dyn frame_support::dispatch::PaysFee<(&u32,)>>::pays_fee(
                            &__pallet_base_weight,
                            (_inc,),
                        );
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::transfer { ref config } => {
                    let __pallet_base_weight = 42;
                    let __pallet_weight = <dyn frame_support::dispatch::WeighData<(
                        &TransferConfig<T::AccountId>,
                    )>>::weigh_data(
                        &__pallet_base_weight, (config,)
                    );
                    let __pallet_class = <dyn frame_support::dispatch::ClassifyDispatch<(
                        &TransferConfig<T::AccountId>,
                    )>>::classify_dispatch(
                        &__pallet_base_weight, (config,)
                    );
                    let __pallet_pays_fee = <dyn frame_support::dispatch::PaysFee<(
                        &TransferConfig<T::AccountId>,
                    )>>::pays_fee(
                        &__pallet_base_weight, (config,)
                    );
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &[::core::fmt::ArgumentV1::new_display(
                            &::core::fmt::Arguments::new_v1(&["__Ignore cannot be used"], &[]),
                        )],
                    ))
                }
            }
        }
    }
    impl<T: Config> frame_support::dispatch::GetCallName for Call<T> {
        fn get_call_name(&self) -> &'static str {
            match *self {
                Self::first_transaction { .. } => "first_transaction",
                Self::transfer { .. } => "transfer",
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &[::core::fmt::ArgumentV1::new_display(
                            &::core::fmt::Arguments::new_v1(
                                &["__PhantomItem cannot be used."],
                                &[],
                            ),
                        )],
                    ))
                }
            }
        }
        fn get_call_names() -> &'static [&'static str] {
            &["first_transaction", "transfer"]
        }
    }
    impl<T: Config> frame_support::traits::UnfilteredDispatchable for Call<T> {
        type Origin = frame_system::pallet_prelude::OriginFor<T>;
        fn dispatch_bypass_filter(
            self,
            origin: Self::Origin,
        ) -> frame_support::dispatch::DispatchResultWithPostInfo {
            match self {
                Self::first_transaction { inc: _inc } => {
                    let __within_span__ = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "first_transaction",
                                    "dispatch_pallet::pallet",
                                    ::tracing::Level::TRACE,
                                    Some("src/lib.rs"),
                                    Some(11u32),
                                    Some("dispatch_pallet::pallet"),
                                    ::tracing_core::field::FieldSet::new(
                                        &[],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && ::tracing::__macro_support::__is_enabled(
                                CALLSITE.metadata(),
                                interest,
                            )
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                        } else {
                            let span =
                                ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
                            {};
                            span
                        }
                    };
                    let __tracing_guard__ = __within_span__.enter();
                    frame_support::storage::in_storage_layer(|| {
                        <Pallet<T>>::first_transaction(origin, _inc)
                            .map(Into::into)
                            .map_err(Into::into)
                    })
                }
                Self::transfer { config } => {
                    let __within_span__ = {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "transfer",
                                    "dispatch_pallet::pallet",
                                    ::tracing::Level::TRACE,
                                    Some("src/lib.rs"),
                                    Some(11u32),
                                    Some("dispatch_pallet::pallet"),
                                    ::tracing_core::field::FieldSet::new(
                                        &[],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = CALLSITE.interest();
                                !interest.is_never()
                            }
                            && ::tracing::__macro_support::__is_enabled(
                                CALLSITE.metadata(),
                                interest,
                            )
                        {
                            let meta = CALLSITE.metadata();
                            ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                        } else {
                            let span =
                                ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
                            {};
                            span
                        }
                    };
                    let __tracing_guard__ = __within_span__.enter();
                    frame_support::storage::in_storage_layer(|| {
                        <Pallet<T>>::transfer(origin, config)
                            .map(Into::into)
                            .map_err(Into::into)
                    })
                }
                Self::__Ignore(_, _) => {
                    let _ = origin;
                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &[::core::fmt::ArgumentV1::new_display(
                            &::core::fmt::Arguments::new_v1(
                                &["__PhantomItem cannot be used."],
                                &[],
                            ),
                        )],
                    ));
                }
            }
        }
    }
    impl<T: Config> frame_support::dispatch::Callable<T> for Pallet<T> {
        type Call = Call<T>;
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn call_functions() -> frame_support::metadata::PalletCallMetadata {
            frame_support::scale_info::meta_type::<Call<T>>().into()
        }
    }
    pub use __tt_error_token_1 as tt_error_token;
    #[doc(hidden)]
    pub mod __substrate_event_check {
        #[doc(hidden)]
        pub use __is_event_part_defined_2 as is_event_part_defined;
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn storage_metadata() -> frame_support::metadata::PalletStorageMetadata {
            frame_support :: metadata :: PalletStorageMetadata { prefix : < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: name :: < Pallet < T > > () . expect ("Every active pallet has a name in the runtime; qed") , entries : { # [allow (unused_mut)] let mut entries = :: alloc :: vec :: Vec :: new () ; entries } , }
        }
    }
    #[doc(hidden)]
    pub mod __substrate_inherent_check {
        #[doc(hidden)]
        pub use __is_inherent_part_defined_3 as is_inherent_part_defined;
    }
    /// Hidden instance generated to be internally used when module is used without
    /// instance.
    #[doc(hidden)]
    pub type __InherentHiddenInstance = ();
    impl<T: Config> frame_support::traits::Hooks<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
    }
    impl<T: Config> frame_support::traits::OnFinalize<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn on_finalize(n: <T as frame_system::Config>::BlockNumber) {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_finalize",
                            "dispatch_pallet::pallet",
                            ::tracing::Level::TRACE,
                            Some("src/lib.rs"),
                            Some(11u32),
                            Some("dispatch_pallet::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(CALLSITE.metadata(), interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_finalize (n)
        }
    }
    impl<T: Config> frame_support::traits::OnIdle<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn on_idle(
            n: <T as frame_system::Config>::BlockNumber,
            remaining_weight: frame_support::weights::Weight,
        ) -> frame_support::weights::Weight {
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_idle (n , remaining_weight)
        }
    }
    impl<T: Config> frame_support::traits::OnInitialize<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn on_initialize(
            n: <T as frame_system::Config>::BlockNumber,
        ) -> frame_support::weights::Weight {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_initialize",
                            "dispatch_pallet::pallet",
                            ::tracing::Level::TRACE,
                            Some("src/lib.rs"),
                            Some(11u32),
                            Some("dispatch_pallet::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(CALLSITE.metadata(), interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_initialize (n)
        }
    }
    impl<T: Config> frame_support::traits::OnRuntimeUpgrade for Pallet<T> {
        fn on_runtime_upgrade() -> frame_support::weights::Weight {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_runtime_update",
                            "dispatch_pallet::pallet",
                            ::tracing::Level::TRACE,
                            Some("src/lib.rs"),
                            Some(11u32),
                            Some("dispatch_pallet::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(CALLSITE.metadata(), interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(CALLSITE.metadata());
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            let pallet_name = < < T as frame_system :: Config > :: PalletInfo as frame_support :: traits :: PalletInfo > :: name :: < Self > () . unwrap_or ("<unknown pallet name>") ;
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["\u{2705} no migration for "],
                            &[::core::fmt::ArgumentV1::new_display(&pallet_name)],
                        ),
                        lvl,
                        &(
                            frame_support::LOG_TARGET,
                            "dispatch_pallet::pallet",
                            "src/lib.rs",
                            11u32,
                        ),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: on_runtime_upgrade ()
        }
    }
    impl<T: Config> frame_support::traits::OffchainWorker<<T as frame_system::Config>::BlockNumber>
        for Pallet<T>
    {
        fn offchain_worker(n: <T as frame_system::Config>::BlockNumber) {
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: offchain_worker (n)
        }
    }
    impl<T: Config> frame_support::traits::IntegrityTest for Pallet<T> {
        fn integrity_test() {
            < Self as frame_support :: traits :: Hooks < < T as frame_system :: Config > :: BlockNumber > > :: integrity_test ()
        }
    }
    #[doc(hidden)]
    pub mod __substrate_genesis_config_check {
        #[doc(hidden)]
        pub use __is_genesis_config_defined_4 as is_genesis_config_defined;
        #[doc(hidden)]
        pub use __is_std_enabled_for_genesis_4 as is_std_enabled_for_genesis;
    }
    #[doc(hidden)]
    pub mod __substrate_origin_check {
        #[doc(hidden)]
        pub use __is_origin_part_defined_5 as is_origin_part_defined;
    }
    #[doc(hidden)]
    pub mod __substrate_validate_unsigned_check {
        #[doc(hidden)]
        pub use __is_validate_unsigned_part_defined_6 as is_validate_unsigned_part_defined;
    }
    pub use __tt_default_parts_7 as tt_default_parts;
}
