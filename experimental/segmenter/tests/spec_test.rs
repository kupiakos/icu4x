// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_segmenter::GraphemeClusterBreakSegmenter;
use icu_segmenter::LineBreakSegmenter;
use icu_segmenter::SentenceBreakSegmenter;
use icu_segmenter::WordBreakSegmenter;
use std::char;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::u32;

struct TestContentIterator {
    reader: std::io::BufReader<File>,
}

struct TestData {
    original_line: String,
    utf8_vec: Vec<char>,
    utf16_vec: Vec<u16>,
    latin1_vec: Vec<u8>,
    break_result_utf8: Vec<usize>,
    break_result_utf16: Vec<usize>,
    break_result_latin1: Option<Vec<usize>>,
}

impl TestContentIterator {
    pub fn new(filename: &str) -> Self {
        let f = File::open(filename);
        Self {
            reader: BufReader::new(f.unwrap()),
        }
    }
}

impl Iterator for TestContentIterator {
    type Item = TestData;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut line = String::new();
            let len = self.reader.read_line(&mut line).ok()?;
            if len == 0 {
                // EOF
                return None;
            }
            if line.starts_with('#') {
                // Comment
                continue;
            }

            let mut r = line.split('#');
            let r = r.next();
            let v = r.unwrap().split_ascii_whitespace();
            let mut char_break: Vec<_> = Vec::new();
            let mut u8_break: Vec<_> = Vec::new();
            let mut u16_break: Vec<_> = Vec::new();
            let mut char_vec: Vec<_> = Vec::new();
            let mut u8_vec: Vec<_> = Vec::new();
            let mut u16_vec: Vec<_> = Vec::new();

            let mut char_len = 0;
            let mut u8_len = 0;
            let mut u16_len = 0;

            let mut ascii_only = true;
            for (count, item) in v.enumerate() {
                if count % 2 == 1 {
                    let ch = char::from_u32(u32::from_str_radix(item, 16).unwrap()).unwrap();
                    char_vec.push(ch);
                    char_len += ch.len_utf8();

                    if ch as u32 >= 0x100 {
                        ascii_only = false;
                    } else {
                        u8_vec.push(ch as u8);
                        u8_len += 1;
                    }

                    let mut u16_buf = [0; 2];
                    let ch_u16 = ch.encode_utf16(&mut u16_buf);
                    u16_vec.extend_from_slice(ch_u16);
                    u16_len += ch_u16.len();
                } else if item != "\u{00d7}" {
                    assert_eq!(item, "\u{00f7}");
                    char_break.push(char_len);
                    u8_break.push(u8_len);
                    u16_break.push(u16_len);
                }
            }
            return Some(Self::Item {
                original_line: line,
                utf8_vec: char_vec,
                utf16_vec: u16_vec,
                latin1_vec: u8_vec,
                break_result_utf8: char_break,
                break_result_utf16: u16_break,
                break_result_latin1: if ascii_only { Some(u8_break) } else { None },
            });
        }
    }
}

#[test]
fn run_line_break_test() {
    let test_iter = TestContentIterator::new("./tests/testdata/LineBreakTest.txt");
    let segmenter = LineBreakSegmenter::try_new(&icu_testdata::unstable()).expect("Data exists");
    for test in test_iter {
        let s: String = test.utf8_vec.into_iter().collect();
        let iter = segmenter.segment_str(&s);
        let result: Vec<usize> = iter.collect();
        assert_eq!(result, test.break_result_utf8, "{}", test.original_line);

        let iter = segmenter.segment_utf16(&test.utf16_vec);
        let result: Vec<usize> = iter.collect();
        assert_eq!(
            result, test.break_result_utf16,
            "UTF16: {}",
            test.original_line
        );

        // Test data is Latin-1 character only, it can run for Latin-1 segmenter test.
        if let Some(break_result_latin1) = test.break_result_latin1 {
            let iter = segmenter.segment_latin1(&test.latin1_vec);
            let result: Vec<usize> = iter.collect();
            assert_eq!(
                result, break_result_latin1,
                "Latin1: {}",
                test.original_line
            );
        }
    }
}

#[test]
fn run_word_break_test() {
    let test_iter = TestContentIterator::new("./tests/testdata/WordBreakTest.txt");
    let segmenter = WordBreakSegmenter::try_new(&icu_testdata::unstable()).expect("Data exists");
    for test in test_iter {
        let s: String = test.utf8_vec.into_iter().collect();
        let iter = segmenter.segment_str(&s);
        let result: Vec<usize> = iter.collect();
        assert_eq!(result, test.break_result_utf8, "{}", test.original_line);

        let iter = segmenter.segment_utf16(&test.utf16_vec);
        let result: Vec<usize> = iter.collect();
        assert_eq!(
            result, test.break_result_utf16,
            "UTF16: {}",
            test.original_line
        );

        // Test data is Latin-1 character only, it can run for Latin-1 segmenter test.
        if let Some(break_result_latin1) = test.break_result_latin1 {
            let iter = segmenter.segment_latin1(&test.latin1_vec);
            let result: Vec<usize> = iter.collect();
            assert_eq!(
                result, break_result_latin1,
                "Latin1: {}",
                test.original_line
            );
        }
    }
}

#[test]
fn run_grapheme_break_test() {
    let test_iter = TestContentIterator::new("./tests/testdata/GraphemeBreakTest.txt");
    let segmenter =
        GraphemeClusterBreakSegmenter::try_new(&icu_testdata::unstable()).expect("Data exists");
    for test in test_iter {
        let s: String = test.utf8_vec.into_iter().collect();
        let iter = segmenter.segment_str(&s);
        let result: Vec<usize> = iter.collect();
        assert_eq!(result, test.break_result_utf8, "{}", test.original_line);

        let iter = segmenter.segment_utf16(&test.utf16_vec);
        let result: Vec<usize> = iter.collect();
        assert_eq!(
            result, test.break_result_utf16,
            "UTF16: {}",
            test.original_line
        );

        // Test data is Latin-1 character only, it can run for Latin-1 segmenter test.
        if let Some(break_result_latin1) = test.break_result_latin1 {
            let iter = segmenter.segment_latin1(&test.latin1_vec);
            let result: Vec<usize> = iter.collect();
            assert_eq!(
                result, break_result_latin1,
                "Latin1: {}",
                test.original_line
            );
        }
    }
}

#[test]
fn run_sentence_break_test() {
    let test_iter = TestContentIterator::new("./tests/testdata/SentenceBreakTest.txt");
    let segmenter =
        SentenceBreakSegmenter::try_new(&icu_testdata::unstable()).expect("Data exists");
    for test in test_iter {
        let s: String = test.utf8_vec.into_iter().collect();
        let iter = segmenter.segment_str(&s);
        let result: Vec<usize> = iter.collect();
        assert_eq!(result, test.break_result_utf8, "{}", test.original_line);

        let iter = segmenter.segment_utf16(&test.utf16_vec);
        let result: Vec<usize> = iter.collect();
        assert_eq!(
            result, test.break_result_utf16,
            "UTF16: {}",
            test.original_line
        );

        // Test data is Latin-1 character only, it can run for Latin-1 segmenter test.
        if let Some(break_result_latin1) = test.break_result_latin1 {
            let iter = segmenter.segment_latin1(&test.latin1_vec);
            let result: Vec<usize> = iter.collect();
            assert_eq!(
                result, break_result_latin1,
                "Latin1: {}",
                test.original_line
            );
        }
    }
}
