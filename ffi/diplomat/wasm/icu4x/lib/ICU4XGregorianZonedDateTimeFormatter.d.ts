import { FFIError } from "./diplomat-runtime"
import { ICU4XCustomTimeZone } from "./ICU4XCustomTimeZone";
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XDateLength } from "./ICU4XDateLength";
import { ICU4XError } from "./ICU4XError";
import { ICU4XIsoDateTime } from "./ICU4XIsoDateTime";
import { ICU4XIsoTimeZoneOptions } from "./ICU4XIsoTimeZoneOptions";
import { ICU4XLocale } from "./ICU4XLocale";
import { ICU4XTimeLength } from "./ICU4XTimeLength";

/**

 * An object capable of formatting a date time with time zone to a string.

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TypedZonedDateTimeFormatter.html Rust documentation for `TypedZonedDateTimeFormatter`} for more information.
 */
export class ICU4XGregorianZonedDateTimeFormatter {

  /**

   * Creates a new {@link ICU4XGregorianZonedDateTimeFormatter `ICU4XGregorianZonedDateTimeFormatter`} from locale data.

   * This function has `date_length` and `time_length` arguments and uses default options for the time zone.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TypedZonedDateTimeFormatter.html#method.try_new_unstable Rust documentation for `try_new_unstable`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new(provider: ICU4XDataProvider, locale: ICU4XLocale, date_length: ICU4XDateLength, time_length: ICU4XTimeLength): ICU4XGregorianZonedDateTimeFormatter | never;

  /**

   * Creates a new {@link ICU4XGregorianZonedDateTimeFormatter `ICU4XGregorianZonedDateTimeFormatter`} from locale data.

   * This function has `date_length` and `time_length` arguments and uses an ISO-8601 style fallback for the time zone with the given configurations.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TypedZonedDateTimeFormatter.html#method.try_new_unstable Rust documentation for `try_new_unstable`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new_with_iso_8601_time_zone_fallback(provider: ICU4XDataProvider, locale: ICU4XLocale, date_length: ICU4XDateLength, time_length: ICU4XTimeLength, zone_options: ICU4XIsoTimeZoneOptions): ICU4XGregorianZonedDateTimeFormatter | never;

  /**

   * Formats a {@link ICU4XIsoDateTime `ICU4XIsoDateTime`} and {@link ICU4XCustomTimeZone `ICU4XCustomTimeZone`} to a string.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/struct.TypedZonedDateTimeFormatter.html#method.format_to_write Rust documentation for `format_to_write`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  format_iso_datetime_with_custom_time_zone(datetime: ICU4XIsoDateTime, time_zone: ICU4XCustomTimeZone): string | never;
}
