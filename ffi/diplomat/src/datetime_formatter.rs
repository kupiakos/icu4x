// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use diplomat_runtime::DiplomatResult;
    use icu_calendar::{Date, DateTime, Gregorian};
    use icu_datetime::{
        options::length, DateFormatter, DateTimeFormatter, TimeFormatter, TypedDateFormatter,
        TypedDateTimeFormatter,
    };

    use crate::{
        date::ffi::{ICU4XDate, ICU4XIsoDate},
        datetime::ffi::ICU4XDateTime,
        datetime::ffi::ICU4XIsoDateTime,
        errors::ffi::ICU4XError,
        locale::ffi::ICU4XLocale,
        provider::ffi::ICU4XDataProvider,
        time::ffi::ICU4XTime,
    };

    #[diplomat::opaque]
    /// An ICU4X TimeFormatter object capable of formatting an [`ICU4XTime`] type (and others) as a string
    #[diplomat::rust_link(icu::datetime::TimeFormatter, Struct)]
    // TODO(#2153) - Rename to ICU4XTimeFormatter when we remove the dependency on calendar
    // from TimeFormatter.
    pub struct ICU4XTimeFormatter(pub TimeFormatter);

    #[diplomat::enum_convert(length::Time, needs_wildcard)]
    #[diplomat::rust_link(icu::datetime::options::length::Time, Enum)]
    pub enum ICU4XTimeLength {
        Full,
        Long,
        Medium,
        Short,
    }

    impl ICU4XTimeFormatter {
        /// Creates a new [`ICU4XTimeFormatter`] from locale data.
        #[diplomat::rust_link(icu::datetime::TimeFormatter::try_new_unstable, FnInStruct)]
        pub fn try_new(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
            length: ICU4XTimeLength,
        ) -> DiplomatResult<Box<ICU4XTimeFormatter>, ICU4XError> {
            let locale = locale.to_datalocale();

            TimeFormatter::try_new_unstable(&provider.0, &locale, length.into())
                .map(|tf| Box::new(ICU4XTimeFormatter(tf)))
                .map_err(Into::into)
                .into()
        }

        /// Formats a [`ICU4XTime`] to a string.
        #[diplomat::rust_link(icu::datetime::TimeFormatter::format_to_write, FnInStruct)]
        pub fn format_time(
            &self,
            value: &ICU4XTime,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            let result = self
                .0
                .format_to_write(write, &value.0)
                .map_err(Into::into)
                .into();
            write.flush();
            result
        }

        /// Formats a [`ICU4XDateTime`] to a string.
        #[diplomat::rust_link(icu::datetime::TimeFormatter::format_to_write, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::TimeFormatter::format, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::TimeFormatter::format_to_string, FnInStruct, hidden)]
        pub fn format_datetime(
            &self,
            value: &ICU4XDateTime,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            let result = self
                .0
                .format_to_write(write, &value.0)
                .map_err(Into::into)
                .into();
            write.flush();
            result
        }

        /// Formats a [`ICU4XIsoDateTime`] to a string.
        #[diplomat::rust_link(icu::datetime::TimeFormatter::format_to_write, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::TimeFormatter::format, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::TimeFormatter::format_to_string, FnInStruct, hidden)]
        pub fn format_iso_datetime(
            &self,
            value: &ICU4XIsoDateTime,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            let result = self
                .0
                .format_to_write(write, &value.0)
                .map_err(Into::into)
                .into();
            write.flush();
            result
        }
    }

    #[diplomat::opaque]
    /// An ICU4X TypedDateFormatter object capable of formatting a [`ICU4XIsoDateTime`] as a string,
    /// using the Gregorian Calendar.
    #[diplomat::rust_link(icu::datetime::TypedDateFormatter, Struct)]
    pub struct ICU4XGregorianDateFormatter(pub TypedDateFormatter<Gregorian>);

    #[diplomat::enum_convert(length::Date, needs_wildcard)]
    #[diplomat::rust_link(icu::datetime::options::length::Date, Enum)]
    pub enum ICU4XDateLength {
        Full,
        Long,
        Medium,
        Short,
    }

    impl ICU4XGregorianDateFormatter {
        /// Creates a new [`ICU4XGregorianDateFormatter`] from locale data.
        #[diplomat::rust_link(icu::datetime::TypedDateFormatter::try_new_unstable, FnInStruct)]
        pub fn try_new(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
            length: ICU4XDateLength,
        ) -> DiplomatResult<Box<ICU4XGregorianDateFormatter>, ICU4XError> {
            let locale = locale.to_datalocale();

            TypedDateFormatter::try_new_unstable(&provider.0, &locale, length.into())
                .map(|df| Box::new(ICU4XGregorianDateFormatter(df)))
                .map_err(Into::into)
                .into()
        }
        /// Formats a [`ICU4XIsoDate`] to a string.
        #[diplomat::rust_link(icu::datetime::TypedDateFormatter::format_to_write, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::TypedDateFormatter::format, FnInStruct, hidden)]
        #[diplomat::rust_link(
            icu::datetime::TypedDateFormatter::format_to_string,
            FnInStruct,
            hidden
        )]
        pub fn format_iso_date(
            &self,
            value: &ICU4XIsoDate,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            let greg = Date::new_from_iso(value.0, Gregorian);
            let result = self
                .0
                .format_to_write(write, &greg)
                .map_err(Into::into)
                .into();
            write.flush();
            result
        }
        /// Formats a [`ICU4XIsoDateTime`] to a string.
        #[diplomat::rust_link(icu::datetime::TypedDateFormatter::format_to_write, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::TypedDateFormatter::format, FnInStruct, hidden)]
        #[diplomat::rust_link(
            icu::datetime::TypedDateFormatter::format_to_string,
            FnInStruct,
            hidden
        )]
        pub fn format_iso_datetime(
            &self,
            value: &ICU4XIsoDateTime,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            let greg = DateTime::new_from_iso(value.0, Gregorian);
            let result = self
                .0
                .format_to_write(write, &greg)
                .map_err(Into::into)
                .into();
            write.flush();
            result
        }
    }

    #[diplomat::opaque]
    /// An ICU4X TypedDateTimeFormatter object capable of formatting a [`ICU4XIsoDateTime`] as a string,
    /// using the Gregorian Calendar.
    #[diplomat::rust_link(icu::datetime::TypedDateTimeFormatter, Struct)]
    pub struct ICU4XGregorianDateTimeFormatter(pub TypedDateTimeFormatter<Gregorian>);

    impl ICU4XGregorianDateTimeFormatter {
        /// Creates a new [`ICU4XGregorianDateFormatter`] from locale data.
        #[diplomat::rust_link(icu::datetime::TypedDateTimeFormatter::try_new_unstable, FnInStruct)]
        pub fn try_new(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
            date_length: ICU4XDateLength,
            time_length: ICU4XTimeLength,
        ) -> DiplomatResult<Box<ICU4XGregorianDateTimeFormatter>, ICU4XError> {
            let locale = locale.to_datalocale();

            let options = length::Bag::from_date_time_style(date_length.into(), time_length.into());

            TypedDateTimeFormatter::try_new_unstable(&provider.0, &locale, options.into())
                .map(|dtf| Box::new(ICU4XGregorianDateTimeFormatter(dtf)))
                .map_err(Into::into)
                .into()
        }

        /// Formats a [`ICU4XIsoDateTime`] to a string.
        #[diplomat::rust_link(icu::datetime::TypedDateTimeFormatter::format_to_write, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::TypedDateTimeFormatter::format, FnInStruct, hidden)]
        #[diplomat::rust_link(
            icu::datetime::TypedDateTimeFormatter::format_to_string,
            FnInStruct,
            hidden
        )]
        pub fn format_iso_datetime(
            &self,
            value: &ICU4XIsoDateTime,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            let greg = DateTime::new_from_iso(value.0, Gregorian);
            let result = self
                .0
                .format_to_write(write, &greg)
                .map_err(Into::into)
                .into();
            write.flush();
            result
        }
    }

    #[diplomat::opaque]
    /// An ICU4X DateFormatter object capable of formatting a [`ICU4XDate`] as a string,
    /// using some calendar specified at runtime in the locale.
    #[diplomat::rust_link(icu::datetime::DateFormatter, Struct)]
    pub struct ICU4XDateFormatter(pub DateFormatter);

    impl ICU4XDateFormatter {
        /// Creates a new [`ICU4XDateFormatter`] from locale data.
        #[diplomat::rust_link(icu::datetime::DateFormatter::try_new_unstable, FnInStruct)]
        pub fn try_new(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
            date_length: ICU4XDateLength,
        ) -> DiplomatResult<Box<ICU4XDateFormatter>, ICU4XError> {
            let locale = locale.to_datalocale();

            DateFormatter::try_new_unstable(&provider.0, &locale, date_length.into())
                .map(|dtf| Box::new(ICU4XDateFormatter(dtf)))
                .map_err(Into::into)
                .into()
        }
        /// Formats a [`ICU4XDate`] to a string.
        #[diplomat::rust_link(icu::datetime::DateFormatter::format_to_write, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::DateFormatter::format, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::DateFormatter::format_to_string, FnInStruct, hidden)]
        pub fn format_date(
            &self,
            value: &ICU4XDate,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            #[allow(unused_variables)]
            let result = self
                .0
                .format_to_write(write, &value.0)
                .map_err(Into::into)
                .into();
            write.flush();
            result
        }
        /// Formats a [`ICU4XIsoDate`] to a string.
        ///
        /// Will convert to this formatter's calendar first
        #[diplomat::rust_link(icu::datetime::DateFormatter::format_to_write, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::DateFormatter::format, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::DateFormatter::format_to_string, FnInStruct, hidden)]
        pub fn format_iso_date(
            &self,
            value: &ICU4XIsoDate,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            let any = value.0.to_any();
            let result = self
                .0
                .format_to_write(write, &any)
                .map_err(Into::into)
                .into();
            write.flush();
            result
        }
        /// Formats a [`ICU4XDateTime`] to a string.
        #[diplomat::rust_link(icu::datetime::DateFormatter::format_to_write, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::DateFormatter::format, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::DateFormatter::format_to_string, FnInStruct, hidden)]
        pub fn format_datetime(
            &self,
            value: &ICU4XDateTime,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            let result = self
                .0
                .format_to_write(write, &value.0)
                .map_err(Into::into)
                .into();
            write.flush();
            result
        }
        /// Formats a [`ICU4XIsoDateTime`] to a string.
        ///
        /// Will convert to this formatter's calendar first
        #[diplomat::rust_link(icu::datetime::DateFormatter::format_to_write, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::DateFormatter::format, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::datetime::DateFormatter::format_to_string, FnInStruct, hidden)]
        pub fn format_iso_datetime(
            &self,
            value: &ICU4XIsoDateTime,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            let any = value.0.to_any();
            let result = self
                .0
                .format_to_write(write, &any)
                .map_err(Into::into)
                .into();
            write.flush();
            result
        }
    }

    #[diplomat::opaque]
    /// An ICU4X DateFormatter object capable of formatting a [`ICU4XDateTime`] as a string,
    /// using some calendar specified at runtime in the locale.
    #[diplomat::rust_link(icu::datetime::DateTimeFormatter, Struct)]
    pub struct ICU4XDateTimeFormatter(pub DateTimeFormatter);

    impl ICU4XDateTimeFormatter {
        /// Creates a new [`ICU4XDateTimeFormatter`] from locale data.
        #[diplomat::rust_link(icu::datetime::DateTimeFormatter::try_new_unstable, FnInStruct)]
        pub fn try_new(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
            date_length: ICU4XDateLength,
            time_length: ICU4XTimeLength,
        ) -> DiplomatResult<Box<ICU4XDateTimeFormatter>, ICU4XError> {
            let locale = locale.to_datalocale();
            let options = length::Bag::from_date_time_style(date_length.into(), time_length.into());

            DateTimeFormatter::try_new_unstable(&provider.0, &locale, options.into())
                .map(|dtf| Box::new(ICU4XDateTimeFormatter(dtf)))
                .map_err(Into::into)
                .into()
        }

        /// Formats a [`ICU4XDateTime`] to a string.
        #[diplomat::rust_link(icu::datetime::DateTimeFormatter::format_to_write, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::DateTimeFormatter::format, FnInStruct, hidden)]
        #[diplomat::rust_link(
            icu::datetime::DateTimeFormatter::format_to_string,
            FnInStruct,
            hidden
        )]
        pub fn format_datetime(
            &self,
            value: &ICU4XDateTime,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            #[allow(unused_variables)]
            let result = self
                .0
                .format_to_write(write, &value.0)
                .map_err(Into::into)
                .into();
            write.flush();
            result
        }

        /// Formats a [`ICU4XIsoDateTime`] to a string.
        ///
        /// Will convert to this formatter's calendar first
        #[diplomat::rust_link(icu::datetime::DateTimeFormatter::format_to_write, FnInStruct)]
        #[diplomat::rust_link(icu::datetime::DateTimeFormatter::format, FnInStruct, hidden)]
        #[diplomat::rust_link(
            icu::datetime::DateTimeFormatter::format_to_string,
            FnInStruct,
            hidden
        )]
        pub fn format_iso_datetime(
            &self,
            value: &ICU4XIsoDateTime,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            let any = value.0.to_any();
            #[allow(unused_variables)]
            let result = self
                .0
                .format_to_write(write, &any)
                .map_err(Into::into)
                .into();
            write.flush();
            result
        }
    }
}
