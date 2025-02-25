// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![no_main] // https://github.com/unicode-org/icu4x/issues/395

icu_benchmark_macros::static_setup!();

use icu_list::{ListFormatter, ListStyle};
use icu_locid::locale;

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    icu_benchmark_macros::main_setup!();

    let list_formatter = ListFormatter::try_new_and_unstable(
        &icu_testdata::unstable(),
        &locale!("es").into(),
        ListStyle::Wide,
    )
    .unwrap();

    println!(
        "{}",
        list_formatter.format(["España", "Francia", "Suiza", "Italia"].iter())
    );

    0
}
