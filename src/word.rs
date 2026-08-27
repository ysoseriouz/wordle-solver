//! 5-letter word packed losslessly into a `u32` (3 bits per letter, 0–25).

/// A word of exactly 5 lowercase ASCII letters.
///
/// Each letter occupies 5 bits; the `i`-th character (0 = leftmost) lives in
/// bits `[5i, 5i+5)`. Because all letters are 5 bits and the string is fixed
/// at 5 chars, the packed `u32` orders lexicographically like the word itself.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Word(pub u32);

impl Word {
    /// Parse 5 lowercase ASCII letters. Returns `None` on any other input.
    #[inline]
    pub fn parse(s: &str) -> Option<Word> {
        let b = s.as_bytes();
        if b.len() != 5 {
            return None;
        }
        let mut w = 0u32;
        for (i, &c) in b.iter().enumerate() {
            // Fast branchless a-z check (rejects uppercase + non-letters).
            let low = c.wrapping_sub(b'a');
            if low >= 26 {
                return None;
            }
            w |= (low as u32) << (5 * i);
        }
        Some(Word(w))
    }

    /// Letter (char index `i`) as a value in 0..=25; `i` = 0 is the leftmost.
    #[inline]
    pub fn letter(self, i: usize) -> u8 {
        ((self.0 >> (5 * i)) & 0x1F) as u8
    }
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..5 {
            write!(f, "{}", (b'a' + self.letter(i)) as char)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_roundtrip() {
        let w = Word::parse("crane").unwrap();
        assert_eq!(w.to_string(), "crane");

        assert!(Word::parse("cra").is_none());
        assert!(Word::parse("CRANE").is_none());
        assert!(Word::parse("cr!ne").is_none());
    }
}
