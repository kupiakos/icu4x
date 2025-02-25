import { FFIError } from "./diplomat-runtime"
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";

/**

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.ComposingNormalizer.html Rust documentation for `ComposingNormalizer`} for more information.
 */
export class ICU4XComposingNormalizer {

  /**

   * Construct a new ICU4XComposingNormalizer instance for NFC

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.ComposingNormalizer.html#method.try_new_nfc_unstable Rust documentation for `try_new_nfc_unstable`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new_nfc(provider: ICU4XDataProvider): ICU4XComposingNormalizer | never;

  /**

   * Construct a new ICU4XComposingNormalizer instance for NFKC

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.ComposingNormalizer.html#method.try_new_nfkc_unstable Rust documentation for `try_new_nfkc_unstable`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new_nfkc(provider: ICU4XDataProvider): ICU4XComposingNormalizer | never;

  /**

   * Normalize a (potentially ill-formed) UTF8 string

   * Errors are mapped to REPLACEMENT CHARACTER

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.ComposingNormalizer.html#method.normalize_utf8 Rust documentation for `normalize_utf8`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  normalize(s: string): string | never;

  /**

   * Check if a (potentially ill-formed) UTF8 string is normalized

   * Errors are mapped to REPLACEMENT CHARACTER

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.ComposingNormalizer.html#method.is_normalized_utf8 Rust documentation for `is_normalized_utf8`} for more information.
   */
  is_normalized(s: string): boolean;
}
