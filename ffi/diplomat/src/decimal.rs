// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use diplomat_runtime::DiplomatResult;
    use icu_decimal::{
        options::{FixedDecimalFormatterOptions, GroupingStrategy},
        provider::DecimalSymbolsV1Marker,
        FixedDecimalFormatter,
    };
    use icu_locid::Locale;
    use icu_provider::DataProvider;
    use icu_provider::KeyedDataMarker;
    use icu_provider_adapters::any_payload::AnyPayloadProvider;
    use writeable::Writeable;

    use crate::{
        data_struct::ffi::ICU4XDataStruct, errors::ffi::ICU4XError,
        fixed_decimal::ffi::ICU4XFixedDecimal, locale::ffi::ICU4XLocale,
        provider::ffi::ICU4XDataProvider,
    };

    #[diplomat::opaque]
    /// An ICU4X Fixed Decimal Format object, capable of formatting a [`ICU4XFixedDecimal`] as a string.
    #[diplomat::rust_link(icu::decimal::FixedDecimalFormatter, Struct)]
    pub struct ICU4XFixedDecimalFormatter(pub FixedDecimalFormatter);

    #[diplomat::rust_link(icu::decimal::GroupingStrategy, Enum)]
    pub enum ICU4XFixedDecimalGroupingStrategy {
        Auto,
        Never,
        Always,
        Min2,
    }

    impl ICU4XFixedDecimalFormatter {
        /// Creates a new [`ICU4XFixedDecimalFormatter`] from locale data.
        #[diplomat::rust_link(icu::decimal::FixedDecimalFormatter::try_new, FnInStruct)]
        pub fn try_new_with_grouping_strategy(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
            grouping_strategy: ICU4XFixedDecimalGroupingStrategy,
        ) -> DiplomatResult<Box<ICU4XFixedDecimalFormatter>, ICU4XError> {
            Self::try_new_impl(&provider.0, locale, grouping_strategy)
        }

        /// Creates a new [`ICU4XFixedDecimalFormatter`] from preconstructed locale data in the form of an [`ICU4XDataStruct`]
        /// constructed from `ICU4XDataStruct::create_decimal_symbols()`.
        ///
        /// The contents of the data struct will be consumed: if you wish to use the struct again it will have to be reconstructed.
        /// Passing a consumed struct to this method will return an error.
        pub fn try_new_from_decimal_symbols_v1(
            data_struct: &ICU4XDataStruct,
            grouping_strategy: ICU4XFixedDecimalGroupingStrategy,
        ) -> DiplomatResult<Box<ICU4XFixedDecimalFormatter>, ICU4XError> {
            use icu_provider::prelude::AsDowncastingAnyProvider;
            let provider = AnyPayloadProvider {
                key: DecimalSymbolsV1Marker::KEY,
                // None: This clone is free, since cloning AnyPayload is free.
                data: data_struct.0.clone(),
            };
            Self::try_new_impl(
                &provider.as_downcasting(),
                &ICU4XLocale(Locale::UND),
                grouping_strategy,
            )
        }

        fn try_new_impl<D>(
            provider: &D,
            locale: &ICU4XLocale,
            grouping_strategy: ICU4XFixedDecimalGroupingStrategy,
        ) -> DiplomatResult<Box<ICU4XFixedDecimalFormatter>, ICU4XError>
        where
            D: DataProvider<DecimalSymbolsV1Marker> + ?Sized,
        {
            let locale = locale.to_datalocale();

            let grouping_strategy = match grouping_strategy {
                ICU4XFixedDecimalGroupingStrategy::Auto => GroupingStrategy::Auto,
                ICU4XFixedDecimalGroupingStrategy::Never => GroupingStrategy::Never,
                ICU4XFixedDecimalGroupingStrategy::Always => GroupingStrategy::Always,
                ICU4XFixedDecimalGroupingStrategy::Min2 => GroupingStrategy::Min2,
            };
            let mut options = FixedDecimalFormatterOptions::default();
            options.grouping_strategy = grouping_strategy;
            FixedDecimalFormatter::try_new_unstable(provider, &locale, options)
                .map(|fdf| Box::new(ICU4XFixedDecimalFormatter(fdf)))
                .map_err(Into::into)
                .into()
        }

        /// Formats a [`ICU4XFixedDecimal`] to a string.
        #[diplomat::rust_link(icu::decimal::FixedDecimalFormatter::format, FnInStruct)]
        pub fn format(
            &self,
            value: &ICU4XFixedDecimal,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            #[allow(unused_variables)]
            let result = self
                .0
                .format(&value.0)
                .write_to(write)
                .map_err(Into::into)
                .into();
            write.flush();
            result
        }
    }
}
