// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::*;
use alloc::borrow::Cow;
use core::fmt;

macro_rules! impl_write_num {
    ($u:ty, $i:ty, $test:ident, $log10:ident) => {
        impl $crate::Writeable for $u {
            fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
                let mut buf = [b'0'; $log10(<$u>::MAX) as usize + 1];
                let mut n = *self;
                let mut i = buf.len();
                #[allow(clippy::indexing_slicing)] // n < 10^i
                while n != 0 {
                    i -= 1;
                    buf[i] = b'0' + (n % 10) as u8;
                    n /= 10;
                }
                if i == buf.len() {
                    debug_assert_eq!(*self, 0);
                    i -= 1;
                }
                #[allow(clippy::indexing_slicing)] // buf is ASCII
                let s = unsafe { core::str::from_utf8_unchecked(&buf[i..]) };
                sink.write_str(s)
            }

            fn writeable_length_hint(&self) -> $crate::LengthHint {
                $crate::LengthHint::exact(if *self == 0 {
                    1
                } else {
                    $log10(*self) as usize + 1
                })
            }
        }

        // TODO: use the library functions once stabilized.
        // https://github.com/unicode-org/icu4x/issues/1428
        #[inline]
        const fn $log10(s: $u) -> u32 {
            let b = (<$u>::BITS - 1) - s.leading_zeros();
            // s ∈ [2ᵇ, 2ᵇ⁺¹-1] => ⌊log₁₀(s)⌋ ∈ [⌊log₁₀(2ᵇ)⌋, ⌊log₁₀(2ᵇ⁺¹-1)⌋]
            //                 <=> ⌊log₁₀(s)⌋ ∈ [⌊log₁₀(2ᵇ)⌋, ⌊log₁₀(2ᵇ⁺¹)⌋]
            //                 <=> ⌊log₁₀(s)⌋ ∈ [⌊b log₁₀(2)⌋, ⌊(b+1) log₁₀(2)⌋]
            // The second line holds because there is no integer in
            // [log₁₀(2ᶜ-1), log₁₀(2ᶜ)], if there were, there'd be some 10ⁿ in
            // [2ᶜ-1, 2ᶜ], but it can't be 2ᶜ-1 due to parity nor 2ᶜ due to prime
            // factors.

            const M: u32 = (core::f64::consts::LOG10_2 * (1 << 26) as f64) as u32;
            let low = (b * M) >> 26;
            let high = ((b + 1) * M) >> 26;

            // If the bounds aren't tight (e.g. 87 ∈ [64, 127] ⟹ ⌊log₁₀(87)⌋ ∈ [1,2]),
            // compare to 10ʰ (100). This shouldn't happen too often as there are more
            // powers of 2 than 10 (it happens for 14% of u32s).
            if high == low {
                low
            } else if s < (10 as $u).pow(high) {
                low
            } else {
                high
            }
        }

        impl $crate::Writeable for $i {
            fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
                if self.is_negative() {
                    sink.write_str("-")?;
                }
                self.unsigned_abs().write_to(sink)
            }

            fn writeable_length_hint(&self) -> $crate::LengthHint {
                $crate::LengthHint::exact(if self.is_negative() { 1 } else { 0 })
                    + self.unsigned_abs().writeable_length_hint()
            }
        }

        #[test]
        fn $test() {
            use $crate::assert_writeable_eq;
            assert_writeable_eq!(&(0 as $u), "0");
            assert_writeable_eq!(&(0 as $u), "0");
            assert_writeable_eq!(&(-0 as $i), "0");
            assert_writeable_eq!(&(1 as $u), "1");
            assert_writeable_eq!(&(1 as $i), "1");
            assert_writeable_eq!(&(-1 as $i), "-1");
            assert_writeable_eq!(&(10 as $u), "10");
            assert_writeable_eq!(&(10 as $i), "10");
            assert_writeable_eq!(&(-10 as $i), "-10");
            assert_writeable_eq!(&(99 as $u), "99");
            assert_writeable_eq!(&(99 as $i), "99");
            assert_writeable_eq!(&(-99 as $i), "-99");
            assert_writeable_eq!(&(100 as $u), "100");
            assert_writeable_eq!(&(-100 as $i), "-100");
            assert_writeable_eq!(&<$u>::MAX, <$u>::MAX.to_string());
            assert_writeable_eq!(&<$i>::MAX, <$i>::MAX.to_string());
            assert_writeable_eq!(&<$i>::MIN, <$i>::MIN.to_string());

            use rand::{rngs::SmallRng, Rng, SeedableRng};
            let mut rng = SmallRng::seed_from_u64(4); // chosen by fair dice roll.
                                                      // guaranteed to be random.
            for _ in 0..1000 {
                let rand = rng.gen::<$u>();
                assert_writeable_eq!(rand, rand.to_string());
            }
        }
    };
}

impl_write_num!(u8, i8, test_u8, log10_u8);
impl_write_num!(u16, i16, test_u16, log10_u16);
impl_write_num!(u32, i32, test_u32, log10_u32);
impl_write_num!(u64, i64, test_u64, log10_u64);
impl_write_num!(u128, i128, test_u128, log10_u128);

#[test]
fn assert_log10_approximation() {
    for i in 1..u128::BITS {
        assert_eq!(i * 59 / 196, 2f64.powf(i.into()).log10().floor() as u32);
    }
}

impl Writeable for str {
    #[inline]
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        sink.write_str(self)
    }

    #[inline]
    fn writeable_length_hint(&self) -> LengthHint {
        LengthHint::exact(self.len())
    }

    /// Returns a borrowed `str`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::borrow::Cow;
    /// use writeable::Writeable;
    ///
    /// let cow = "foo".write_to_string();
    /// assert!(matches!(cow, Cow::Borrowed(_)));
    /// ```
    #[inline]
    fn write_to_string(&self) -> Cow<str> {
        Cow::Borrowed(self)
    }
}

impl Writeable for String {
    #[inline]
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        sink.write_str(self)
    }

    #[inline]
    fn writeable_length_hint(&self) -> LengthHint {
        LengthHint::exact(self.len())
    }

    #[inline]
    fn write_to_string(&self) -> Cow<str> {
        Cow::Borrowed(self)
    }
}

impl<'a, T: Writeable + ?Sized> Writeable for &T {
    #[inline]
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        (*self).write_to(sink)
    }

    #[inline]
    fn write_to_parts<W: PartsWrite + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        (*self).write_to_parts(sink)
    }

    #[inline]
    fn writeable_length_hint(&self) -> LengthHint {
        (*self).writeable_length_hint()
    }

    #[inline]
    fn write_to_string(&self) -> Cow<str> {
        (*self).write_to_string()
    }
}

#[test]
fn test_string_impls() {
    fn check_writeable_slice<W: Writeable>(writeables: &[W]) {
        assert_writeable_eq!(&writeables[0], "");
        assert_writeable_eq!(&writeables[1], "abc");
    }

    // test str impl
    let arr: &[&str] = &["", "abc"];
    check_writeable_slice(arr);

    // test String impl
    let arr: &[String] = &["".to_string(), "abc".to_string()];
    check_writeable_slice(arr);

    // test &T impl
    let arr: &[&String] = &[&"".to_string(), &"abc".to_string()];
    check_writeable_slice(arr);
}
