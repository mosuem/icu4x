// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    use core::fmt::Write;

    #[cfg(feature = "buffer_provider")]
    use crate::unstable::{errors::ffi::DataError, provider::ffi::DataProvider};

    #[non_exhaustive]
    pub enum BidiDirection {
        // This is an output type, so the default mostly impacts deferred initialization.
        // We pick Ltr since the algorithm defaults to Ltr in the absence of other info.
        #[diplomat::attr(auto, default)]
        Ltr,
        Rtl,
        Mixed,
    }

    #[diplomat::opaque]
    /// An ICU4X Bidi object, containing loaded bidi data
    #[diplomat::rust_link(icu::properties::props::BidiClass, Struct)]
    #[diplomat::attr(demo_gen, disable)] // TODO needs custom page
    pub struct Bidi(pub icu_properties::CodePointMapData<icu_properties::props::BidiClass>);

    impl Bidi {
        /// Creates a new [`Bidi`] from locale data using compiled data.
        #[diplomat::attr(auto, constructor)]
        #[cfg(feature = "compiled_data")]
        pub fn create() -> Box<Bidi> {
            Box::new(Bidi(
                icu_properties::CodePointMapData::new().static_to_owned(),
            ))
        }

        /// Creates a new [`Bidi`] from locale data, and a particular data source.
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "with_provider")]
        #[cfg(feature = "buffer_provider")]
        pub fn create_with_provider(provider: &DataProvider) -> Result<Box<Bidi>, DataError> {
            Ok(Box::new(Bidi(
                icu_properties::CodePointMapData::try_new_unstable(&provider.get_unstable()?)?,
            )))
        }
        /// Use the data loaded in this object to process a string and calculate bidi information
        ///
        /// Takes in a Level for the default level, if it is an invalid value or None it will default to Auto.
        ///
        /// Returns nothing if `text` is invalid UTF-8.
        #[diplomat::rust_link(unicode_bidi::BidiInfo::new_with_data_source, FnInStruct)]
        #[diplomat::rust_link(
            icu::properties::CodePointMapDataBorrowed::bidi_class,
            FnInStruct,
            hidden
        )]
        #[diplomat::attr(not(supports = utf8_strings), disable)]
        #[diplomat::attr(*, rename = "for_text")]
        pub fn for_text_utf8<'text>(
            &self,
            text: &'text DiplomatStr,
            default_level: Option<u8>,
        ) -> Option<Box<BidiInfo<'text>>> {
            let text = core::str::from_utf8(text).ok()?;

            Some(Box::new(BidiInfo(
                unicode_bidi::BidiInfo::new_with_data_source(
                    &self.0.as_borrowed(),
                    text,
                    default_level.and_then(|l| unicode_bidi::Level::new(l).ok()),
                ),
            )))
        }

        /// Use the data loaded in this object to process a string and calculate bidi information
        ///
        /// Takes in a Level for the default level, if it is an invalid value it will default to LTR
        #[diplomat::rust_link(unicode_bidi::BidiInfo::new_with_data_source, FnInStruct)]
        #[diplomat::rust_link(
            icu::properties::CodePointMapDataBorrowed::bidi_class,
            FnInStruct,
            hidden
        )]
        // The only safe UTF-8 strings are those generated by the Diplomat layer in UTF-16 languages
        #[diplomat::attr(supports = utf8_strings, disable)]
        #[diplomat::attr(supports = utf16_strings, rename = "for_text")]
        pub fn for_text_valid_utf8<'text>(
            &self,
            text: &'text str,
            default_level: Option<u8>,
        ) -> Box<BidiInfo<'text>> {
            Box::new(BidiInfo(unicode_bidi::BidiInfo::new_with_data_source(
                &self.0.as_borrowed(),
                text,
                default_level.and_then(|l| unicode_bidi::Level::new(l).ok()),
            )))
        }

        /// Utility function for producing reorderings given a list of levels
        ///
        /// Produces a map saying which visual index maps to which source index.
        ///
        /// The levels array must not have values greater than 126 (this is the
        /// Bidi maximum explicit depth plus one).
        /// Failure to follow this invariant may lead to incorrect results,
        /// but is still safe.
        #[diplomat::rust_link(unicode_bidi::BidiInfo::reorder_visual, FnInStruct)]
        pub fn reorder_visual(&self, levels: &[u8]) -> Box<ReorderedIndexMap> {
            let levels = unicode_bidi::Level::from_slice_unchecked(levels);
            Box::new(ReorderedIndexMap(unicode_bidi::BidiInfo::reorder_visual(
                levels,
            )))
        }

        /// Check if a Level returned by level_at is an RTL level.
        ///
        /// Invalid levels (numbers greater than 125) will be assumed LTR
        #[diplomat::rust_link(unicode_bidi::level::Level::is_rtl, FnInStruct)]
        pub fn level_is_rtl(level: u8) -> bool {
            unicode_bidi::Level::new(level)
                .unwrap_or_else(|_| unicode_bidi::Level::ltr())
                .is_rtl()
        }

        /// Check if a Level returned by level_at is an LTR level.
        ///
        /// Invalid levels (numbers greater than 125) will be assumed LTR
        #[diplomat::rust_link(unicode_bidi::level::Level::is_ltr, FnInStruct)]
        pub fn level_is_ltr(level: u8) -> bool {
            unicode_bidi::Level::new(level)
                .unwrap_or_else(|_| unicode_bidi::Level::ltr())
                .is_ltr()
        }

        /// Get a basic RTL Level value
        #[diplomat::rust_link(unicode_bidi::level::Level::rtl, FnInStruct)]
        pub fn level_rtl() -> u8 {
            unicode_bidi::Level::rtl().number()
        }

        /// Get a simple LTR Level value
        #[diplomat::rust_link(unicode_bidi::level::Level::ltr, FnInStruct)]
        pub fn level_ltr() -> u8 {
            unicode_bidi::Level::ltr().number()
        }
    }

    /// Thin wrapper around a vector that maps visual indices to source indices
    ///
    /// `map[visualIndex] = sourceIndex`
    ///
    /// Produced by `reorder_visual()` on [`Bidi`].
    #[diplomat::opaque]
    #[diplomat::attr(demo_gen, disable)] // TODO needs custom page
    pub struct ReorderedIndexMap(pub Vec<usize>);

    impl ReorderedIndexMap {
        /// Get this as a slice/array of indices
        #[diplomat::attr(auto, getter)]
        pub fn as_slice<'a>(&'a self) -> &'a [usize] {
            &self.0
        }

        /// The length of this map
        #[diplomat::attr(auto, getter = "length")]
        pub fn len(&self) -> usize {
            self.0.len()
        }

        /// Whether this map is empty
        #[diplomat::attr(auto, getter)]
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }

        /// Get element at `index`. Returns 0 when out of bounds
        /// (note that 0 is also a valid in-bounds value, please use `len()`
        /// to avoid out-of-bounds)
        #[diplomat::attr(auto, indexer)]
        pub fn get(&self, index: usize) -> usize {
            self.0.get(index).copied().unwrap_or(0)
        }
    }

    /// An object containing bidi information for a given string, produced by `for_text()` on `Bidi`
    #[diplomat::rust_link(unicode_bidi::BidiInfo, Struct)]
    #[diplomat::opaque]
    #[diplomat::attr(demo_gen, disable)] // TODO needs custom page
    pub struct BidiInfo<'text>(pub unicode_bidi::BidiInfo<'text>);

    impl<'text> BidiInfo<'text> {
        /// The number of paragraphs contained here
        #[diplomat::attr(auto, getter)]
        pub fn paragraph_count(&self) -> usize {
            self.0.paragraphs.len()
        }

        /// Get the nth paragraph, returning `None` if out of bounds
        pub fn paragraph_at(&'text self, n: usize) -> Option<Box<BidiParagraph<'text>>> {
            self.0
                .paragraphs
                .get(n)
                .map(|p| Box::new(BidiParagraph(unicode_bidi::Paragraph::new(&self.0, p))))
        }

        /// The number of bytes in this full text
        #[diplomat::attr(auto, getter)]
        pub fn size(&self) -> usize {
            self.0.levels.len()
        }

        /// Get the BIDI level at a particular byte index in the full text.
        /// This integer is conceptually a `unicode_bidi::Level`,
        /// and can be further inspected using the static methods on Bidi.
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
    #[diplomat::attr(demo_gen, disable)] // TODO needs custom page
    pub struct BidiParagraph<'info>(pub unicode_bidi::Paragraph<'info, 'info>);

    impl<'info> BidiParagraph<'info> {
        /// Given a paragraph index `n` within the surrounding text, this sets this
        /// object to the paragraph at that index. Returns nothing when out of bounds.
        ///
        /// This is equivalent to calling `paragraph_at()` on `BidiInfo` but doesn't
        /// create a new object
        pub fn set_paragraph_in_text(&mut self, n: usize) -> bool {
            let Some(para) = self.0.info.paragraphs.get(n) else {
                return false;
            };
            self.0 = unicode_bidi::Paragraph::new(self.0.info, para);
            true
        }

        #[diplomat::rust_link(unicode_bidi::Paragraph::level_at, FnInStruct)]
        #[diplomat::attr(auto, getter)]
        /// The primary direction of this paragraph
        pub fn direction(&self) -> BidiDirection {
            self.0.direction().into()
        }

        /// The number of bytes in this paragraph
        #[diplomat::rust_link(unicode_bidi::ParagraphInfo::len, FnInStruct)]
        #[diplomat::attr(auto, getter)]
        pub fn size(&self) -> usize {
            self.0.para.len()
        }

        /// The start index of this paragraph within the source text
        #[diplomat::attr(auto, getter)]
        pub fn range_start(&self) -> usize {
            self.0.para.range.start
        }

        /// The end index of this paragraph within the source text
        #[diplomat::attr(auto, getter)]
        pub fn range_end(&self) -> usize {
            self.0.para.range.end
        }

        /// Reorder a line based on display order. The ranges are specified relative to the source text and must be contained
        /// within this paragraph's range.
        #[diplomat::rust_link(unicode_bidi::Paragraph::level_at, FnInStruct)]
        #[diplomat::attr(demo_gen, disable)] // TODO needs custom page
        pub fn reorder_line(
            &self,
            range_start: usize,
            range_end: usize,
            out: &mut DiplomatWrite,
        ) -> Option<()> {
            if range_start < self.range_start() || range_end > self.range_end() {
                return None;
            }

            let info = self.0.info;
            let para = self.0.para;

            let reordered = info.reorder_line(para, range_start..range_end);

            let _infallible = out.write_str(&reordered);

            Some(())
        }

        /// Get the BIDI level at a particular byte index in this paragraph.
        /// This integer is conceptually a `unicode_bidi::Level`,
        /// and can be further inspected using the static methods on Bidi.
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

impl From<Direction> for ffi::BidiDirection {
    fn from(other: Direction) -> Self {
        match other {
            Direction::Ltr => Self::Ltr,
            Direction::Rtl => Self::Rtl,
            Direction::Mixed => Self::Mixed,
        }
    }
}
