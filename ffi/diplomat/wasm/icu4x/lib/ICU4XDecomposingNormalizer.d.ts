import { FFIError } from "./diplomat-runtime"
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XError } from "./ICU4XError";

/**

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.DecomposingNormalizer.html Rust documentation for `DecomposingNormalizer`} for more information.
 */
export class ICU4XDecomposingNormalizer {

  /**

   * Construct a new ICU4XDecomposingNormalizer instance for NFC

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.DecomposingNormalizer.html#method.try_new_nfd_unstable Rust documentation for `try_new_nfd_unstable`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new_nfd(provider: ICU4XDataProvider): ICU4XDecomposingNormalizer | never;

  /**

   * Construct a new ICU4XDecomposingNormalizer instance for NFKC

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.DecomposingNormalizer.html#method.try_new_nfkd_unstable Rust documentation for `try_new_nfkd_unstable`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new_nfkd(provider: ICU4XDataProvider): ICU4XDecomposingNormalizer | never;

  /**

   * Normalize a (potentially ill-formed) UTF8 string

   * Errors are mapped to REPLACEMENT CHARACTER

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.DecomposingNormalizer.html#method.normalize_utf8 Rust documentation for `normalize_utf8`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  normalize(s: string): string | never;

  /**

   * Check if a (potentially ill-formed) UTF8 string is normalized

   * Errors are mapped to REPLACEMENT CHARACTER

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/struct.DecomposingNormalizer.html#method.is_normalized_utf8 Rust documentation for `is_normalized_utf8`} for more information.
   */
  is_normalized(s: string): boolean;
}
