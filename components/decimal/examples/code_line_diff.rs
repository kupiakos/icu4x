// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// An example application which displays the number of lines added and
// removed from a series of pull requests.

#![no_main] // https://github.com/unicode-org/icu4x/issues/395

use fixed_decimal::FixedDecimal;
use icu_decimal::FixedDecimalFormatter;
use icu_locid::locale;
use writeable::Writeable;

icu_benchmark_macros::static_setup!();

const LINES_REMOVED_ADDED: [(i64, i64); 5] = [
    (-50, 72),
    (0, 3750),
    (-1201, 0),
    (-9876, 5432),
    (-5000000, 3000000),
];

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    icu_benchmark_macros::main_setup!();

    let fdf = FixedDecimalFormatter::try_new_unstable(
        &icu_testdata::unstable(),
        &locale!("bn").into(),
        Default::default(),
    )
    .expect("Failed to create FixedDecimalFormatter instance.");

    for line in LINES_REMOVED_ADDED.iter() {
        let decimals: (FixedDecimal, FixedDecimal) = (line.0.into(), line.1.into());
        let removed = fdf.format(&decimals.0);
        let added = fdf.format(&decimals.1);
        assert_ne!("", removed.write_to_string());
        assert_ne!("", added.write_to_string());
        #[cfg(debug_assertions)]
        println!(
            "Added/Removed: {}/{}",
            removed.write_to_string(),
            added.write_to_string()
        );
    }

    0
}
