//! A [Vertical Header Table](
//! https://docs.microsoft.com/en-us/typography/opentype/spec/vhea) implementation.

use crate::parser::Stream;

/// A [Vertical Header Table](https://docs.microsoft.com/en-us/typography/opentype/spec/vhea).
#[derive(Clone, Copy, Default, Debug)]
pub struct Table {
    /// Face ascender.
    pub ascender: i16,
    /// Face descender.
    pub descender: i16,
    /// Face line gap.
    pub line_gap: i16,
    /// Number of metrics in the `vmtx` table.
    pub number_of_metrics: u16,
}

impl Table {
    /// Parses a table from raw data.
    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() != 36 {
            return None
        }

        let mut s = Stream::new(data);
        s.skip::<u32>(); // version
        let ascender: i16 = s.read()?;
        let descender: i16 = s.read()?;
        let line_gap: i16 = s.read()?;
        s.advance(24);
        let number_of_metrics: u16 = s.read()?;

        Some(Table {
            ascender,
            descender,
            line_gap,
            number_of_metrics,
        })
    }
}
