// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt;

use crate::error::DateTimeFormatterError as Error;
use crate::{
    input::TimeZoneInput,
    time_zone::{FormatTimeZone, TimeZoneFormatter, TimeZoneFormatterUnit},
    DateTimeFormatterError,
};
use writeable::Writeable;

/// [`FormattedTimeZone`] is a intermediate structure which can be retrieved as an output from [`TimeZoneFormatter`].
pub struct FormattedTimeZone<'l, T>
where
    T: TimeZoneInput,
{
    pub(crate) time_zone_format: &'l TimeZoneFormatter,
    pub(crate) time_zone: &'l T,
}

impl<'l, T> Writeable for FormattedTimeZone<'l, T>
where
    T: TimeZoneInput,
{
    /// Format time zone with fallbacks.
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        match self.write_no_fallback(sink) {
            Ok(Ok(r)) => Ok(r),
            _ => match self.time_zone_format.fallback_unit {
                TimeZoneFormatterUnit::LocalizedGmt(fallback) => {
                    match fallback.format(
                        sink,
                        self.time_zone,
                        &self.time_zone_format.data_payloads,
                    ) {
                        Ok(Ok(r)) => Ok(r),
                        Ok(Err(e)) => Err(e),
                        Err(e) => self.handle_last_resort_error(e, sink),
                    }
                }
                TimeZoneFormatterUnit::Iso8601(fallback) => {
                    match fallback.format(
                        sink,
                        self.time_zone,
                        &self.time_zone_format.data_payloads,
                    ) {
                        Ok(Ok(r)) => Ok(r),
                        Ok(Err(e)) => Err(e),
                        Err(e) => self.handle_last_resort_error(e, sink),
                    }
                }
                _ => Err(core::fmt::Error),
            },
        }
    }

    // TODO(#489): Implement writeable_length_hint
}

impl<'l, T> fmt::Display for FormattedTimeZone<'l, T>
where
    T: TimeZoneInput,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_to(f)
    }
}

impl<'l, T> FormattedTimeZone<'l, T>
where
    T: TimeZoneInput,
{
    /// Write time zone with no fallback.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::timezone::CustomTimeZone;
    /// use icu_datetime::{TimeZoneFormatter, DateTimeFormatterError};
    /// use icu_locid::locale;
    /// use tinystr::tinystr;
    /// use writeable::Writeable;
    ///
    /// let mut tzf = TimeZoneFormatter::try_new_unstable(
    ///     &icu_testdata::unstable(),
    ///     &locale!("en").into(),
    ///     Default::default(),
    /// )
    /// .unwrap();
    ///
    /// let mut time_zone = "Z".parse::<CustomTimeZone>().unwrap();
    /// time_zone.time_zone_id = Some(tinystr!(8, "gblon").into());
    /// let mut sink = String::new();
    ///
    /// // There are no non-fallback formats enabled:
    /// assert!(matches!(
    ///     tzf.format(&time_zone).write_no_fallback(&mut sink),
    ///     Err(DateTimeFormatterError::UnsupportedOptions)
    /// ));
    /// assert!(sink.is_empty());
    ///
    /// // Enable a non-fallback format:
    /// tzf.load_generic_location_format(&icu_testdata::unstable()).unwrap();
    /// assert!(matches!(
    ///     tzf.format(&time_zone).write_no_fallback(&mut sink),
    ///     Ok(Ok(_))
    /// ));
    /// assert_eq!("London Time", sink);
    ///
    /// // Errors still occur if the time zone is not supported:
    /// sink.clear();
    /// time_zone.time_zone_id = Some(tinystr!(8, "zzzzz").into());
    /// assert!(matches!(
    ///     tzf.format(&time_zone).write_no_fallback(&mut sink),
    ///     Err(DateTimeFormatterError::UnsupportedOptions)
    /// ));
    ///
    /// // Use `write` instead to enable infallible formatting:
    /// assert!(matches!(
    ///     tzf.format(&time_zone).write_to(&mut sink),
    ///     Ok(_)
    /// ));
    /// assert_eq!("GMT", sink);
    /// ```
    pub fn write_no_fallback<W>(&self, w: &mut W) -> Result<fmt::Result, Error>
    where
        W: core::fmt::Write + ?Sized,
    {
        for unit in self.time_zone_format.format_units.iter() {
            match unit.format(w, self.time_zone, &self.time_zone_format.data_payloads) {
                Ok(r) => return Ok(r),
                Err(DateTimeFormatterError::UnsupportedOptions) => continue,
                Err(e) => return Err(e),
            }
        }
        Err(DateTimeFormatterError::UnsupportedOptions)
    }

    fn handle_last_resort_error<W>(&self, e: DateTimeFormatterError, sink: &mut W) -> fmt::Result
    where
        W: core::fmt::Write + ?Sized,
    {
        match e {
            DateTimeFormatterError::MissingInputField(Some("gmt_offset")) => {
                debug_assert!(
                    false,
                    "Warning: using last-resort time zone fallback: {:?}.\
 To fix this warning, ensure the gmt_offset field is present.",
                    &self
                        .time_zone_format
                        .data_payloads
                        .zone_formats
                        .get()
                        .gmt_offset_fallback
                );
                sink.write_str(
                    &self
                        .time_zone_format
                        .data_payloads
                        .zone_formats
                        .get()
                        .gmt_offset_fallback,
                )
            }
            _ => {
                debug_assert!(false, "{:?}", e);
                Err(core::fmt::Error)
            }
        }
    }
}
