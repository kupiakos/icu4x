``pluralrules::ffi``
====================

.. cpp:struct:: ICU4XPluralCategories

    FFI version of ``PluralRules::categories()`` data.


    .. cpp:member:: bool zero

    .. cpp:member:: bool one

    .. cpp:member:: bool two

    .. cpp:member:: bool few

    .. cpp:member:: bool many

    .. cpp:member:: bool other

.. cpp:enum-struct:: ICU4XPluralCategory

    FFI version of ``PluralCategory``.

    See the `Rust documentation for PluralCategory <https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/enum.PluralCategory.html>`__ for more information.


    .. cpp:enumerator:: Zero

    .. cpp:enumerator:: One

    .. cpp:enumerator:: Two

    .. cpp:enumerator:: Few

    .. cpp:enumerator:: Many

    .. cpp:enumerator:: Other

    .. cpp:function:: static diplomat::result<ICU4XPluralCategory, std::monostate> from_tr35_string(const std::string_view s)

        Construct from a string in the format `specified in TR35 <https://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules>`__

        See the `Rust documentation for from_tr35_string <https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/enum.PluralCategory.html#method.from_tr35_string>`__ for more information.

        See the `Rust documentation for from_tr35_bytes <https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/enum.PluralCategory.html#method.from_tr35_bytes>`__ for more information.


.. cpp:struct:: ICU4XPluralOperands

    FFI version of ``PluralOperands``.

    See the `Rust documentation for PluralOperands <https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/struct.PluralOperands.html>`__ for more information.


    .. cpp:member:: uint64_t i

    .. cpp:member:: size_t v

    .. cpp:member:: size_t w

    .. cpp:member:: uint64_t f

    .. cpp:member:: uint64_t t

    .. cpp:member:: size_t c

    .. cpp:function:: static diplomat::result<ICU4XPluralOperands, ICU4XError> create(const std::string_view s)

        Construct for a given string representing a number

        See the `Rust documentation for from_str <https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/struct.PluralOperands.html#method.from_str>`__ for more information.


.. cpp:class:: ICU4XPluralRules

    FFI version of ``PluralRules``.

    See the `Rust documentation for PluralRules <https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/struct.PluralRules.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XPluralRules, ICU4XError> try_new_cardinal(const ICU4XDataProvider& provider, const ICU4XLocale& locale)

        Construct an :cpp:class:`ICU4XPluralRules` for the given locale, for cardinal numbers

        See the `Rust documentation for try_new_cardinal_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/struct.PluralRules.html#method.try_new_cardinal_unstable>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XPluralRules, ICU4XError> try_new_ordinal(const ICU4XDataProvider& provider, const ICU4XLocale& locale)

        Construct an :cpp:class:`ICU4XPluralRules` for the given locale, for ordinal numbers

        See the `Rust documentation for try_new_ordinal_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/struct.PluralRules.html#method.try_new_ordinal_unstable>`__ for more information.


    .. cpp:function:: ICU4XPluralCategory category_for(ICU4XPluralOperands op) const

        Get the category for a given number represented as operands

        See the `Rust documentation for category_for <https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/struct.PluralRules.html#method.category_for>`__ for more information.


    .. cpp:function:: ICU4XPluralCategories categories() const

        Get all of the categories needed in the current locale

        See the `Rust documentation for categories <https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/struct.PluralRules.html#method.categories>`__ for more information.

