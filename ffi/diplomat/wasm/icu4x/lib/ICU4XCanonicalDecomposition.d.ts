import { char } from "./diplomat-runtime"
import { FFIError } from "./diplomat-runtime"
import { ICU4XDataProvider } from "./ICU4XDataProvider";
import { ICU4XDecomposed } from "./ICU4XDecomposed";
import { ICU4XError } from "./ICU4XError";

/**

 * The raw (non-recursive) canonical decomposition operation.

 * Callers should generally use ICU4XDecomposingNormalizer unless they specifically need raw composition operations

 * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/properties/struct.CanonicalDecomposition.html Rust documentation for `CanonicalDecomposition`} for more information.
 */
export class ICU4XCanonicalDecomposition {

  /**

   * Construct a new ICU4XCanonicalDecomposition instance for NFC

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/properties/struct.CanonicalDecomposition.html#method.try_new_unstable Rust documentation for `try_new_unstable`} for more information.
   * @throws {@link FFIError}<{@link ICU4XError}>
   */
  static try_new(provider: ICU4XDataProvider): ICU4XCanonicalDecomposition | never;

  /**

   * Performs non-recursive canonical decomposition (including for Hangul).

   * See the {@link https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/properties/struct.CanonicalDecomposition.html#method.decompose Rust documentation for `decompose`} for more information.
   */
  decompose(c: char): ICU4XDecomposed;
}
