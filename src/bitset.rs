//! Survivor bitset over the answer id space. Zero allocation in steady state.

use crate::evaluator::evaluate;
use crate::pattern::Pattern;
use crate::word::Word;

pub const WORDS_PER_BLOCK: usize = 64;

/// Set of still-possible answer ids; a fixed-size bitset of 64-bit blocks.
pub struct Survivors {
    blocks: Vec<u64>,
    len: usize, // number of answers represented (last block may be partially unused)
}

#[inline]
fn get_bit(bits: &[u64], i: usize) -> bool {
    (bits[i / WORDS_PER_BLOCK] >> (i % WORDS_PER_BLOCK)) & 1 == 1
}

fn set_bit(bits: &mut [u64], i: usize, value: bool) {
    let m = 1u64 << (i % WORDS_PER_BLOCK);
    if value {
        bits[i / WORDS_PER_BLOCK] |= m
    } else {
        bits[i / WORDS_PER_BLOCK] &= !m
    }
}

impl Survivors {
    /// All answers initially possible.
    pub fn all(n_answers: usize) -> Self {
        let mut s = Survivors {
            blocks: vec![0; n_answers.div_ceil(WORDS_PER_BLOCK)],
            len: n_answers,
        };
        for i in 0..n_answers {
            set_bit(&mut s.blocks, i, true);
        }
        s
    }

    /// Number of live bits. Recomputed each call (≤ ~37 blocks, cheap).
    pub fn count(&self) -> usize {
        self.blocks.iter().map(|w| w.count_ones() as usize).sum()
    }

    /// Keep only ids `a` where `evaluate(guess, answers[a]) == pattern`.
    pub fn retain_matching(&mut self, guess: Word, pattern: Pattern, answers: &[Word]) {
        for (a, w) in answers.iter().enumerate() {
            if get_bit(&self.blocks, a) && evaluate(guess, *w) != pattern {
                set_bit(&mut self.blocks, a, false);
            }
        }
    }

    /// Iterate over live answers in ascending id order (sequential reads).
    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        (0..self.len).filter(move |&i| get_bit(&self.blocks, i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::GRAY;

    #[test]
    fn all_then_retain() {
        let words: Vec<Word> = ["crane", "crate", "muddy", "plumb"]
            .iter()
            .map(|w| Word::parse(w).unwrap())
            .collect();
        let mut s = Survivors::all(words.len());
        assert_eq!(s.count(), 4);
        assert_eq!(s.iter().collect::<Vec<_>>(), vec![0, 1, 2, 3]);
        let all_gray = Pattern::from_tiles([GRAY; 5]);
        s.retain_matching(Word::parse("qzzxw").unwrap(), all_gray, &words);
        // none of the four contain q/z/x/w → all stay
        assert_eq!(s.count(), 4);
    }
}
