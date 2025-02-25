``zoned_formatter::ffi``
========================

.. js:class:: ICU4XGregorianZonedDateTimeFormatter

    An object capable of formatting a date time with time zone to a string.

    See the `Rust documentation for TypedZonedDateTimeFormatter <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TypedZonedDateTimeFormatter.html>`__ for more information.


    .. js:staticfunction:: try_new(provider, locale, date_length, time_length)

        Creates a new :js:class:`ICU4XGregorianZonedDateTimeFormatter` from locale data.

        This function has ``date_length`` and ``time_length`` arguments and uses default options for the time zone.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TypedZonedDateTimeFormatter.html#method.try_new_unstable>`__ for more information.


    .. js:staticfunction:: try_new_with_iso_8601_time_zone_fallback(provider, locale, date_length, time_length, zone_options)

        Creates a new :js:class:`ICU4XGregorianZonedDateTimeFormatter` from locale data.

        This function has ``date_length`` and ``time_length`` arguments and uses an ISO-8601 style fallback for the time zone with the given configurations.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TypedZonedDateTimeFormatter.html#method.try_new_unstable>`__ for more information.


    .. js:function:: format_iso_datetime_with_custom_time_zone(datetime, time_zone)

        Formats a :js:class:`ICU4XIsoDateTime` and :js:class:`ICU4XCustomTimeZone` to a string.

        See the `Rust documentation for format_to_write <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TypedZonedDateTimeFormatter.html#method.format_to_write>`__ for more information.


.. js:class:: ICU4XZonedDateTimeFormatter

    An object capable of formatting a date time with time zone to a string.

    See the `Rust documentation for ZonedDateTimeFormatter <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.ZonedDateTimeFormatter.html>`__ for more information.


    .. js:staticfunction:: try_new(provider, locale, date_length, time_length)

        Creates a new :js:class:`ICU4XZonedDateTimeFormatter` from locale data.

        This function has ``date_length`` and ``time_length`` arguments and uses default options for the time zone.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.ZonedDateTimeFormatter.html#method.try_new_unstable>`__ for more information.


    .. js:staticfunction:: try_new_with_iso_8601_time_zone_fallback(provider, locale, date_length, time_length, zone_options)

        Creates a new :js:class:`ICU4XZonedDateTimeFormatter` from locale data.

        This function has ``date_length`` and ``time_length`` arguments and uses an ISO-8601 style fallback for the time zone with the given configurations.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.ZonedDateTimeFormatter.html#method.try_new_unstable>`__ for more information.


    .. js:function:: format_datetime_with_custom_time_zone(datetime, time_zone)

        Formats a :js:class:`ICU4XDateTime` and :js:class:`ICU4XCustomTimeZone` to a string.

        See the `Rust documentation for format_to_write <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.ZonedDateTimeFormatter.html#method.format_to_write>`__ for more information.


    .. js:function:: format_iso_datetime_with_custom_time_zone(datetime, time_zone)

        Formats a :js:class:`ICU4XIsoDateTime` and :js:class:`ICU4XCustomTimeZone` to a string.

        See the `Rust documentation for format_to_write <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.ZonedDateTimeFormatter.html#method.format_to_write>`__ for more information.

