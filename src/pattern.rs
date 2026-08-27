//! Ternary-encoded feedback tile (Gray/Yellow/Green) packed into a `u8`.

pub const GRAY: u8 = 0;
pub const YELLOW: u8 = 1;
pub const GREEN: u8 = 2;

/// Another per-position scaler: tile `i` × POW3[i].
pub(crate) const POW3: [u8; 5] = [1, 3, 9, 27, 81];

/// Total number of distinct 5-tile patterns (3^5).
pub const ALL: usize = 243;

/// Feedback for a guess: tile `i` contributes `tile * 3^i` to the value.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Pattern(pub u8);

impl Pattern {
    #[inline]
    pub fn tile(self, i: usize) -> u8 {
        (self.0 / POW3[i]) % 3
    }

    pub fn from_tiles(tiles: [u8; 5]) -> Pattern {
        let mut v = 0u8;
        for (i, t) in tiles.iter().enumerate() {
            v += t * POW3[i];
        }
        Pattern(v)
    }

    /// 'B' (gray/black) / 'Y' (yellow) / 'G' (green). API boundary only.
    pub fn to_feedback_str(self) -> String {
        let mut out = [0u8; 5];
        for (i, b) in out.iter_mut().enumerate() {
            *b = b"BYG"[self.tile(i) as usize];
        }
        String::from_utf8(out.to_vec()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_tiles() {
        let p = Pattern::from_tiles([2, 1, 0, 1, 2]);
        assert_eq!(p.to_feedback_str(), "GYBYG");
        assert_eq!(p.tile(0), GREEN);
        assert_eq!(p.tile(4), GREEN);
    }
}
