``normalizer::ffi``
===================

.. cpp:class:: ICU4XComposingNormalizer

    See the `Rust documentation for ComposingNormalizer <https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.ComposingNormalizer.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XComposingNormalizer, ICU4XError> try_new_nfc(const ICU4XDataProvider& provider)

        Construct a new ICU4XComposingNormalizer instance for NFC

        See the `Rust documentation for try_new_nfc_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.ComposingNormalizer.html#method.try_new_nfc_unstable>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XComposingNormalizer, ICU4XError> try_new_nfkc(const ICU4XDataProvider& provider)

        Construct a new ICU4XComposingNormalizer instance for NFKC

        See the `Rust documentation for try_new_nfkc_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.ComposingNormalizer.html#method.try_new_nfkc_unstable>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> normalize_to_writeable(const std::string_view s, W& write) const

        Normalize a (potentially ill-formed) UTF8 string

        Errors are mapped to REPLACEMENT CHARACTER

        See the `Rust documentation for normalize_utf8 <https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.ComposingNormalizer.html#method.normalize_utf8>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> normalize(const std::string_view s) const

        Normalize a (potentially ill-formed) UTF8 string

        Errors are mapped to REPLACEMENT CHARACTER

        See the `Rust documentation for normalize_utf8 <https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.ComposingNormalizer.html#method.normalize_utf8>`__ for more information.


    .. cpp:function:: bool is_normalized(const std::string_view s) const

        Check if a (potentially ill-formed) UTF8 string is normalized

        Errors are mapped to REPLACEMENT CHARACTER

        See the `Rust documentation for is_normalized_utf8 <https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.ComposingNormalizer.html#method.is_normalized_utf8>`__ for more information.


.. cpp:class:: ICU4XDecomposingNormalizer

    See the `Rust documentation for DecomposingNormalizer <https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.DecomposingNormalizer.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XDecomposingNormalizer, ICU4XError> try_new_nfd(const ICU4XDataProvider& provider)

        Construct a new ICU4XDecomposingNormalizer instance for NFC

        See the `Rust documentation for try_new_nfd_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.DecomposingNormalizer.html#method.try_new_nfd_unstable>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XDecomposingNormalizer, ICU4XError> try_new_nfkd(const ICU4XDataProvider& provider)

        Construct a new ICU4XDecomposingNormalizer instance for NFKC

        See the `Rust documentation for try_new_nfkd_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.DecomposingNormalizer.html#method.try_new_nfkd_unstable>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> normalize_to_writeable(const std::string_view s, W& write) const

        Normalize a (potentially ill-formed) UTF8 string

        Errors are mapped to REPLACEMENT CHARACTER

        See the `Rust documentation for normalize_utf8 <https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.DecomposingNormalizer.html#method.normalize_utf8>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> normalize(const std::string_view s) const

        Normalize a (potentially ill-formed) UTF8 string

        Errors are mapped to REPLACEMENT CHARACTER

        See the `Rust documentation for normalize_utf8 <https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.DecomposingNormalizer.html#method.normalize_utf8>`__ for more information.


    .. cpp:function:: bool is_normalized(const std::string_view s) const

        Check if a (potentially ill-formed) UTF8 string is normalized

        Errors are mapped to REPLACEMENT CHARACTER

        See the `Rust documentation for is_normalized_utf8 <https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.DecomposingNormalizer.html#method.is_normalized_utf8>`__ for more information.

