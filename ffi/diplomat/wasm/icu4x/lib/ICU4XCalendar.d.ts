import { FFIError } from "./diplomat-runtime"
import { ICU4XAnyCalendarKind } from "./ICU4XAnyCalendarKind";
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";
import { ICU4XLocale } from "./ICU4XLocale";

/**

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendar.html Rust documentation for `AnyCalendar`} for more information.
 */
export class ICU4XCalendar {

  /**

   * Creates a new {@link ICU4XCalendar `ICU4XCalendar`} from the specified date and time.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendar.html#method.try_new_for_locale_unstable Rust documentation for `try_new_for_locale_unstable`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new_for_locale(provider: ICU4XDataProvider, locale: ICU4XLocale): ICU4XCalendar | never;

  /**

   * Creates a new {@link ICU4XCalendar `ICU4XCalendar`} from the specified date and time.

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendar.html#method.try_new_unstable Rust documentation for `try_new_unstable`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new_for_kind(provider: ICU4XDataProvider, kind: ICU4XAnyCalendarKind): ICU4XCalendar | never;

  /**

   * Returns the kind of this calendar

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.AnyCalendar.html#method.kind Rust documentation for `kind`} for more information.
   */
  kind(): ICU4XAnyCalendarKind;
}
