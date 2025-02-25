// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Formatting date and time.
//!
//! This module is published as its own crate ([`icu_datetime`](https://docs.rs/icu_datetime/latest/icu_datetime/))
//! and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.
//!
//! [`TypedDateTimeFormatter`] and [`DateTimeFormatter`] are the main types of the component. They accepts a set of arguments which
//! allow it to collect necessary data from the [data provider], and once instantiated, can be
//! used to quickly format any date and time provided. There are variants of these types that can format greater or fewer components,
//! including [`TypedDateFormatter`] & [`DateFormatter`], [`TypedZonedDateTimeFormatter`] & [`ZonedDateTimeFormatter`], [`TimeFormatter`],
//! and [`TimeZoneFormatter`]
//!
//! These formatters work with types from the [`icu_calendar`] crate, like [`Date`], [`DateTime`], and [`Time`],
//! and [`icu_timezone::CustomTimeZone`], however other types may be used provided they implement the traits from the [`input`] module.
//!
//! Each instance of a date-related formatter (i.e. not [`TimeFormatter`] or [`TimeZoneFormatter`]
//! is associated with a particular [calendar](icu_calendar::Calendar).
//! The "Typed" vs untyped formatter distinction is to help with this. For example, if you know at compile time that you
//! will only be formatting Gregorian dates, you can use [`TypedDateTimeFormatter<Gregorian>`](TypedDateTimeFormatter) and the
//! APIs will make sure that only Gregorian [`DateTime`]s are used with the calendar. On the other hand, if you want to be able to select
//! the calendar at runtime, you can use [`DateTimeFormatter`] with the calendar specified in the locale, and use it with
//! [`DateTime`](icu_calendar::DateTime)[`<AnyCalendar>`](icu_calendar::any_calendar::AnyCalendar). These formatters still require dates associated
//! with the appropriate calendar (though they will convert ISO dates to the calendar if provided), they just do not force the
//! programmer to pick the calendar at compile time.
//!
//!
//! # Examples
//!
//! ```
//! use icu::calendar::{DateTime, Gregorian};
//! use icu::datetime::{
//!     options::length, DateTimeFormatter, TypedDateTimeFormatter, DateTimeFormatterOptions,
//! };
//! use icu::locid::{locale, Locale};
//! use std::str::FromStr;
//!
//! // See the next code example for a more ergonomic example with .into().
//! let options =
//!     DateTimeFormatterOptions::Length(length::Bag::from_date_time_style(
//!         length::Date::Medium,
//!         length::Time::Short,
//!     ));
//!
//! // You can work with a formatter that can select the calendar at runtime:
//! let locale = Locale::from_str("en-u-ca-gregory").unwrap();
//! let dtf = DateTimeFormatter::try_new_unstable(
//!     &icu_testdata::unstable(),
//!     &locale.into(),
//!     options.clone(),
//! )
//! .expect("Failed to create DateTimeFormatter instance.");
//!
//! // Or one that selects a calendar at compile time:
//! let typed_dtf = TypedDateTimeFormatter::<Gregorian>::try_new_unstable(
//!     &icu_testdata::unstable(),
//!     &locale!("en").into(),
//!     options,
//! )
//! .expect("Failed to create TypedDateTimeFormatter instance.");
//!
//! let typed_date = DateTime::new_gregorian_datetime(2020, 9, 12, 12, 34, 28).unwrap();
//! // prefer using ISO dates with DateTimeFormatter
//! let date = typed_date.to_iso().to_any();
//!
//! let formatted_date = dtf.format(&date).expect("Formatting should succeed");
//! let typed_formatted_date = typed_dtf.format(&typed_date);
//!
//! assert_eq!(formatted_date.to_string(), "Sep 12, 2020, 12:34 PM");
//! assert_eq!(typed_formatted_date.to_string(), "Sep 12, 2020, 12:34 PM");
//! ```
//!
//! The options can be created more ergonomically using the `Into` trait to automatically
//! convert a [`options::length::Bag`] into a [`DateTimeFormatterOptions::Length`].
//!
//! ```
//! use icu::calendar::Gregorian;
//! use icu::datetime::{
//!     options::length, DateTimeFormatterOptions, TypedDateTimeFormatter,
//! };
//! use icu::locid::locale;
//! let options = length::Bag::from_date_time_style(
//!     length::Date::Medium,
//!     length::Time::Short,
//! )
//! .into();
//!
//! let dtf = TypedDateTimeFormatter::<Gregorian>::try_new_unstable(
//!     &icu_testdata::unstable(),
//!     &locale!("en").into(),
//!     options,
//! );
//! ```
//!
//! At the moment, the crate provides only options using the [`Length`] bag, but in the future,
//! we expect to add more ways to customize the output, like skeletons, and components.
//!
//! [data provider]: icu_provider
//! [`ICU4X`]: ../icu/index.html
//! [`Length`]: options::length
//! [`DateTime`]: icu_calendar::DateTime
//! [`Date`]: icu_calendar::Date
//! [`Time`]: icu_calendar::types::Time

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        // TODO(#2266): enable missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

extern crate alloc;

mod calendar;
mod datetime;
mod error;
pub mod fields;
mod format;
pub mod input;
pub mod options;
#[doc(hidden)]
pub mod pattern;
pub mod provider;
pub(crate) mod raw;
#[doc(hidden)]
#[allow(clippy::exhaustive_structs, clippy::exhaustive_enums)] // private-ish module
#[cfg(feature = "experimental_skeleton_matching")]
pub mod skeleton;
#[allow(missing_docs)] // TODO(#686) - Add missing docs.
pub mod time_zone;
mod zoned_datetime;

mod any;

pub use any::{DateFormatter, DateTimeFormatter, ZonedDateTimeFormatter};
pub use calendar::CldrCalendar;
pub use datetime::{TimeFormatter, TypedDateFormatter, TypedDateTimeFormatter};
pub use error::DateTimeFormatterError;
pub use format::datetime::FormattedDateTime;
pub use format::time_zone::FormattedTimeZone;
pub use format::zoned_datetime::FormattedZonedDateTime;
pub use options::DateTimeFormatterOptions;
pub use time_zone::TimeZoneFormatter;
pub use time_zone::TimeZoneFormatterOptions;
pub use zoned_datetime::TypedZonedDateTimeFormatter;
