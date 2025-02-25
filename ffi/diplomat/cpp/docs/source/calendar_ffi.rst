``calendar::ffi``
=================

.. cpp:enum-struct:: ICU4XAnyCalendarKind

    The various calendar types currently supported by :cpp:class:`ICU4XCalendar`

    See the `Rust documentation for AnyCalendarKind <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendarKind.html>`__ for more information.


    .. cpp:enumerator:: Iso

        The kind of an Iso calendar


    .. cpp:enumerator:: Gregorian

        The kind of a Gregorian calendar


    .. cpp:enumerator:: Buddhist

        The kind of a Buddhist calendar


    .. cpp:enumerator:: Japanese

        The kind of a Japanese calendar with modern eras


    .. cpp:enumerator:: JapaneseExtended

        The kind of a Japanese calendar with modern and historic eras


    .. cpp:enumerator:: Ethiopian

        The kind of an Ethiopian calendar, with Amete Mihret era


    .. cpp:enumerator:: EthiopianAmeteAlem

        The kind of an Ethiopian calendar, with Amete Alem era


    .. cpp:enumerator:: Indian

        The kind of a Indian calendar


    .. cpp:enumerator:: Coptic

        The kind of a Coptic calendar


    .. cpp:function:: static diplomat::result<ICU4XAnyCalendarKind, ICU4XError> from_locale(const ICU4XLocale& locale)

        Read the calendar type off of the -u-ca- extension on a locale

        See the `Rust documentation for from_locale <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendarKind.html#method.from_locale>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XAnyCalendarKind, ICU4XError> from_bcp47(const std::string_view s)

        Obtain the calendar type given a BCP-47 -u-ca- extension string

        See the `Rust documentation for from_bcp47 <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendarKind.html#method.from_bcp47>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> bcp47_to_writeable(W& write)

        Obtain the string suitable for use in the -u-ca- extension in a BCP47 locale

        See the `Rust documentation for as_bcp47_string <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendarKind.html#method.as_bcp47_string>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> bcp47()

        Obtain the string suitable for use in the -u-ca- extension in a BCP47 locale

        See the `Rust documentation for as_bcp47_string <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendarKind.html#method.as_bcp47_string>`__ for more information.


.. cpp:class:: ICU4XCalendar

    See the `Rust documentation for AnyCalendar <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendar.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCalendar, ICU4XError> try_new_for_locale(const ICU4XDataProvider& provider, const ICU4XLocale& locale)

        Creates a new :cpp:class:`ICU4XCalendar` from the specified date and time.

        See the `Rust documentation for try_new_for_locale_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendar.html#method.try_new_for_locale_unstable>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCalendar, ICU4XError> try_new_for_kind(const ICU4XDataProvider& provider, ICU4XAnyCalendarKind kind)

        Creates a new :cpp:class:`ICU4XCalendar` from the specified date and time.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendar.html#method.try_new_unstable>`__ for more information.


    .. cpp:function:: ICU4XAnyCalendarKind kind() const

        Returns the kind of this calendar

        See the `Rust documentation for kind <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendar.html#method.kind>`__ for more information.

