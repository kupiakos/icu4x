import wasm from "./diplomat-wasm.mjs"
import * as diplomatRuntime from "./diplomat-runtime.js"

export const ICU4XError_js_to_rust = {
  "UnknownError": 0,
  "WriteableError": 1,
  "OutOfBoundsError": 2,
  "DataMissingDataKeyError": 256,
  "DataMissingVariantError": 257,
  "DataMissingLocaleError": 258,
  "DataNeedsVariantError": 259,
  "DataNeedsLocaleError": 260,
  "DataExtraneousLocaleError": 261,
  "DataFilteredResourceError": 262,
  "DataMismatchedTypeError": 263,
  "DataMissingPayloadError": 264,
  "DataInvalidStateError": 265,
  "DataCustomError": 266,
  "DataIoError": 267,
  "DataUnavailableBufferFormatError": 268,
  "LocaleUndefinedSubtagError": 512,
  "LocaleParserLanguageError": 513,
  "LocaleParserSubtagError": 514,
  "LocaleParserExtensionError": 515,
  "DataStructValidityError": 768,
  "PropertyUnknownScriptIdError": 1024,
  "PropertyUnknownGeneralCategoryGroupError": 1025,
  "FixedDecimalLimitError": 1280,
  "FixedDecimalSyntaxError": 1281,
  "PluralParserError": 1536,
  "DateTimeParseError": 1792,
  "DateTimeOverflowError": 1793,
  "DateTimeUnderflowError": 1794,
  "DateTimeOutOfRangeError": 1795,
  "DateTimeUnknownEraError": 1796,
  "DateTimeUnknownMonthCodeError": 1797,
  "DateTimeMissingInputError": 1798,
  "DateTimeUnknownAnyCalendarKindError": 1799,
  "DateTimeFormatPatternError": 2048,
  "DateTimeFormatMissingInputFieldError": 2049,
  "DateTimeFormatSkeletonError": 2050,
  "DateTimeFormatUnsupportedFieldError": 2051,
  "DateTimeFormatUnsupportedOptionsError": 2052,
  "DateTimeFormatMissingWeekdaySymbolError": 2053,
  "DateTimeFormatMissingMonthSymbolError": 2054,
  "DateTimeFormatFixedDecimalError": 2055,
  "DateTimeFormatMismatchedAnyCalendarError": 2056,
  "TinyStrTooLargeError": 2304,
  "TinyStrContainsNullError": 2305,
  "TinyStrNonAsciiError": 2306,
  "TimeZoneOffsetOutOfBoundsError": 2560,
  "TimeZoneInvalidOffsetError": 2561,
  "TimeZoneMissingInputError": 2562,
  "NormalizerFutureExtensionError": 2816,
  "NormalizerValidationError": 2817,
};

export const ICU4XError_rust_to_js = {
  [0]: "UnknownError",
  [1]: "WriteableError",
  [2]: "OutOfBoundsError",
  [256]: "DataMissingDataKeyError",
  [257]: "DataMissingVariantError",
  [258]: "DataMissingLocaleError",
  [259]: "DataNeedsVariantError",
  [260]: "DataNeedsLocaleError",
  [261]: "DataExtraneousLocaleError",
  [262]: "DataFilteredResourceError",
  [263]: "DataMismatchedTypeError",
  [264]: "DataMissingPayloadError",
  [265]: "DataInvalidStateError",
  [266]: "DataCustomError",
  [267]: "DataIoError",
  [268]: "DataUnavailableBufferFormatError",
  [512]: "LocaleUndefinedSubtagError",
  [513]: "LocaleParserLanguageError",
  [514]: "LocaleParserSubtagError",
  [515]: "LocaleParserExtensionError",
  [768]: "DataStructValidityError",
  [1024]: "PropertyUnknownScriptIdError",
  [1025]: "PropertyUnknownGeneralCategoryGroupError",
  [1280]: "FixedDecimalLimitError",
  [1281]: "FixedDecimalSyntaxError",
  [1536]: "PluralParserError",
  [1792]: "DateTimeParseError",
  [1793]: "DateTimeOverflowError",
  [1794]: "DateTimeUnderflowError",
  [1795]: "DateTimeOutOfRangeError",
  [1796]: "DateTimeUnknownEraError",
  [1797]: "DateTimeUnknownMonthCodeError",
  [1798]: "DateTimeMissingInputError",
  [1799]: "DateTimeUnknownAnyCalendarKindError",
  [2048]: "DateTimeFormatPatternError",
  [2049]: "DateTimeFormatMissingInputFieldError",
  [2050]: "DateTimeFormatSkeletonError",
  [2051]: "DateTimeFormatUnsupportedFieldError",
  [2052]: "DateTimeFormatUnsupportedOptionsError",
  [2053]: "DateTimeFormatMissingWeekdaySymbolError",
  [2054]: "DateTimeFormatMissingMonthSymbolError",
  [2055]: "DateTimeFormatFixedDecimalError",
  [2056]: "DateTimeFormatMismatchedAnyCalendarError",
  [2304]: "TinyStrTooLargeError",
  [2305]: "TinyStrContainsNullError",
  [2306]: "TinyStrNonAsciiError",
  [2560]: "TimeZoneOffsetOutOfBoundsError",
  [2561]: "TimeZoneInvalidOffsetError",
  [2562]: "TimeZoneMissingInputError",
  [2816]: "NormalizerFutureExtensionError",
  [2817]: "NormalizerValidationError",
};

export const ICU4XError = {
  "UnknownError": "UnknownError",
  "WriteableError": "WriteableError",
  "OutOfBoundsError": "OutOfBoundsError",
  "DataMissingDataKeyError": "DataMissingDataKeyError",
  "DataMissingVariantError": "DataMissingVariantError",
  "DataMissingLocaleError": "DataMissingLocaleError",
  "DataNeedsVariantError": "DataNeedsVariantError",
  "DataNeedsLocaleError": "DataNeedsLocaleError",
  "DataExtraneousLocaleError": "DataExtraneousLocaleError",
  "DataFilteredResourceError": "DataFilteredResourceError",
  "DataMismatchedTypeError": "DataMismatchedTypeError",
  "DataMissingPayloadError": "DataMissingPayloadError",
  "DataInvalidStateError": "DataInvalidStateError",
  "DataCustomError": "DataCustomError",
  "DataIoError": "DataIoError",
  "DataUnavailableBufferFormatError": "DataUnavailableBufferFormatError",
  "LocaleUndefinedSubtagError": "LocaleUndefinedSubtagError",
  "LocaleParserLanguageError": "LocaleParserLanguageError",
  "LocaleParserSubtagError": "LocaleParserSubtagError",
  "LocaleParserExtensionError": "LocaleParserExtensionError",
  "DataStructValidityError": "DataStructValidityError",
  "PropertyUnknownScriptIdError": "PropertyUnknownScriptIdError",
  "PropertyUnknownGeneralCategoryGroupError": "PropertyUnknownGeneralCategoryGroupError",
  "FixedDecimalLimitError": "FixedDecimalLimitError",
  "FixedDecimalSyntaxError": "FixedDecimalSyntaxError",
  "PluralParserError": "PluralParserError",
  "DateTimeParseError": "DateTimeParseError",
  "DateTimeOverflowError": "DateTimeOverflowError",
  "DateTimeUnderflowError": "DateTimeUnderflowError",
  "DateTimeOutOfRangeError": "DateTimeOutOfRangeError",
  "DateTimeUnknownEraError": "DateTimeUnknownEraError",
  "DateTimeUnknownMonthCodeError": "DateTimeUnknownMonthCodeError",
  "DateTimeMissingInputError": "DateTimeMissingInputError",
  "DateTimeUnknownAnyCalendarKindError": "DateTimeUnknownAnyCalendarKindError",
  "DateTimeFormatPatternError": "DateTimeFormatPatternError",
  "DateTimeFormatMissingInputFieldError": "DateTimeFormatMissingInputFieldError",
  "DateTimeFormatSkeletonError": "DateTimeFormatSkeletonError",
  "DateTimeFormatUnsupportedFieldError": "DateTimeFormatUnsupportedFieldError",
  "DateTimeFormatUnsupportedOptionsError": "DateTimeFormatUnsupportedOptionsError",
  "DateTimeFormatMissingWeekdaySymbolError": "DateTimeFormatMissingWeekdaySymbolError",
  "DateTimeFormatMissingMonthSymbolError": "DateTimeFormatMissingMonthSymbolError",
  "DateTimeFormatFixedDecimalError": "DateTimeFormatFixedDecimalError",
  "DateTimeFormatMismatchedAnyCalendarError": "DateTimeFormatMismatchedAnyCalendarError",
  "TinyStrTooLargeError": "TinyStrTooLargeError",
  "TinyStrContainsNullError": "TinyStrContainsNullError",
  "TinyStrNonAsciiError": "TinyStrNonAsciiError",
  "TimeZoneOffsetOutOfBoundsError": "TimeZoneOffsetOutOfBoundsError",
  "TimeZoneInvalidOffsetError": "TimeZoneInvalidOffsetError",
  "TimeZoneMissingInputError": "TimeZoneMissingInputError",
  "NormalizerFutureExtensionError": "NormalizerFutureExtensionError",
  "NormalizerValidationError": "NormalizerValidationError",
};
