// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use diplomat_runtime::{DiplomatResult, DiplomatWriteable};

    use core::fmt::Write;
    use icu_properties::bidi::BidiClassAdapter;
    use icu_properties::maps;
    use icu_properties::BidiClass;
    use unicode_bidi::BidiInfo;
    use unicode_bidi::Level;
    use unicode_bidi::Paragraph;

    use crate::errors::ffi::ICU4XError;
    use crate::provider::ffi::ICU4XDataProvider;

    pub enum ICU4XBidiDirection {
        Ltr,
        Rtl,
        Mixed,
    }

    #[diplomat::opaque]
    /// An ICU4X Bidi object, containing loaded bidi data
    #[diplomat::rust_link(icu::properties::bidi::BidiClassAdapter, Struct)]
    // #[diplomat::rust_link(icu::properties::maps::load_bidi_class, Struct)]
    pub struct ICU4XBidi(pub maps::CodePointMapData<BidiClass>);

    impl ICU4XBidi {
        /// Creates a new [`ICU4XBidi`] from locale data.
        #[diplomat::rust_link(icu::properties::bidi::BidiClassAdapter::new, FnInStruct)]
        pub fn try_new(provider: &ICU4XDataProvider) -> DiplomatResult<Box<ICU4XBidi>, ICU4XError> {
            maps::load_bidi_class(&provider.0)
                .map(|bidi| Box::new(ICU4XBidi(bidi)))
                .map_err(Into::into)
                .into()
        }

        /// Use the data loaded in this object to process a string and calculate bidi information
        ///
        /// Takes in a Level for the default level, if it is an invalid value it will default to LTR
        #[diplomat::rust_link(unicode_bidi::BidiInfo::new_with_data_source, FnInStruct)]
        #[diplomat::rust_link(icu::properties::BidiClassAdapter::bidi_class, FnInStruct, hidden)]
        pub fn for_text<'text>(
            &self,
            text: &'text str,
            default_level: u8,
        ) -> Box<ICU4XBidiInfo<'text>> {
            let data = self.0.as_borrowed();
            let adapter = BidiClassAdapter::new(data);

            Box::new(ICU4XBidiInfo(BidiInfo::new_with_data_source(
                &adapter,
                text,
                Level::new(default_level).ok(),
            )))
        }

        /// Check if a Level returned by level_at is an RTL level.
        ///
        /// Invalid levels (numbers greater than 125) will be assumed LTR
        #[diplomat::rust_link(unicode_bidi::Level::is_rtl, FnInStruct)]
        pub fn level_is_rtl(level: u8) -> bool {
            Level::new(level).unwrap_or_else(|_| Level::ltr()).is_rtl()
        }

        /// Check if a Level returned by level_at is an LTR level.
        ///
        /// Invalid levels (numbers greater than 125) will be assumed LTR
        #[diplomat::rust_link(unicode_bidi::Level::is_ltr, FnInStruct)]
        pub fn level_is_ltr(level: u8) -> bool {
            Level::new(level).unwrap_or_else(|_| Level::ltr()).is_ltr()
        }

        /// Get a basic RTL Level value
        #[diplomat::rust_link(unicode_bidi::Level::rtl, FnInStruct)]
        pub fn level_rtl() -> u8 {
            Level::rtl().number()
        }

        /// Get a simple LTR Level value
        #[diplomat::rust_link(unicode_bidi::Level::ltr, FnInStruct)]
        pub fn level_ltr() -> u8 {
            Level::ltr().number()
        }
    }

    /// An object containing bidi information for a given string, produced by `for_text()` on `ICU4XBidi`
    #[diplomat::rust_link(unicode_bidi::BidiInfo, Struct)]
    #[diplomat::opaque]
    pub struct ICU4XBidiInfo<'text>(pub BidiInfo<'text>);

    impl<'text> ICU4XBidiInfo<'text> {
        /// The number of paragraphs contained here
        pub fn paragraph_count(&self) -> usize {
            self.0.paragraphs.len()
        }

        /// Get the nth paragraph, returning None if out of bounds
        pub fn paragraph_at(&'text self, n: usize) -> Option<Box<ICU4XBidiParagraph<'text>>> {
            self.0
                .paragraphs
                .get(n)
                .map(|p| Box::new(ICU4XBidiParagraph(Paragraph::new(&self.0, p))))
        }

        /// The number of bytes in this full text
        pub fn size(&self) -> usize {
            self.0.levels.len()
        }

        /// Get the BIDI level at a particular byte index in the full text.
        /// This integer is conceptually a `unicode_bidi::Level`,
        /// and can be further inspected using the static methods on ICU4XBidi.
        ///
        /// Returns 0 (equivalent to `Level::ltr()`) on error
        pub fn level_at(&self, pos: usize) -> u8 {
            if let Some(l) = self.0.levels.get(pos) {
                l.number()
            } else {
                0
            }
        }
    }

    /// Bidi information for a single processed paragraph
    #[diplomat::opaque]
    pub struct ICU4XBidiParagraph<'info>(pub Paragraph<'info, 'info>);

    impl<'info> ICU4XBidiParagraph<'info> {
        /// Given a paragraph index `n` within the surrounding text, this sets this
        /// object to the paragraph at that index. Returns `ICU4XError::OutOfBoundsError` when out of bounds.
        ///
        /// This is equivalent to calling `paragraph_at()` on `ICU4XBidiInfo` but doesn't
        /// create a new object
        pub fn set_paragraph_in_text(&mut self, n: usize) -> DiplomatResult<(), ICU4XError> {
            let para = self.0.info.paragraphs.get(n);
            let para = if let Some(para) = para {
                para
            } else {
                return Err(ICU4XError::OutOfBoundsError).into();
            };

            self.0 = Paragraph::new(self.0.info, para);
            Ok(()).into()
        }
        #[diplomat::rust_link(unicode_bidi::Paragraph::level_at, FnInStruct)]
        /// The primary direction of this paragraph
        pub fn direction(&self) -> ICU4XBidiDirection {
            self.0.direction().into()
        }

        /// The number of bytes in this paragraph
        #[diplomat::rust_link(unicode_bidi::ParagraphInfo::len, FnInStruct)]
        pub fn size(&self) -> usize {
            self.0.para.len()
        }

        /// The start index of this paragraph within the source text
        pub fn range_start(&self) -> usize {
            self.0.para.range.start
        }

        /// The end index of this paragraph within the source text
        pub fn range_end(&self) -> usize {
            self.0.para.range.end
        }

        /// Reorder a line based on display order. The ranges are specified relative to the source text and must be contained
        /// within this paragraph's range.
        #[diplomat::rust_link(unicode_bidi::Paragraph::level_at, FnInStruct)]
        pub fn reorder_line(
            &self,
            range_start: usize,
            range_end: usize,
            out: &mut DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            if range_start < self.range_start() || range_end > self.range_end() {
                return Err(ICU4XError::OutOfBoundsError).into();
            }

            let info = self.0.info;
            let para = self.0.para;

            let reordered = info.reorder_line(para, range_start..range_end);

            out.write_str(&reordered).map_err(Into::into).into()
        }

        /// Get the BIDI level at a particular byte index in this paragraph.
        /// This integer is conceptually a `unicode_bidi::Level`,
        /// and can be further inspected using the static methods on ICU4XBidi.
        ///
        /// Returns 0 (equivalent to `Level::ltr()`) on error
        #[diplomat::rust_link(unicode_bidi::Paragraph::level_at, FnInStruct)]
        pub fn level_at(&self, pos: usize) -> u8 {
            if pos >= self.size() {
                return 0;
            }

            self.0.level_at(pos).number()
        }
    }
}

use unicode_bidi::Direction;

impl From<Direction> for ffi::ICU4XBidiDirection {
    fn from(other: Direction) -> Self {
        match other {
            Direction::Ltr => Self::Ltr,
            Direction::Rtl => Self::Rtl,
            Direction::Mixed => Self::Mixed,
        }
    }
}
