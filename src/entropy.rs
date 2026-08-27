//! Candidate-scoring by expected remaining-candidate count (sum of squares).
//!
//! The spec mandates Shannon entropy `H = -Σ p·log p`, but for ranking
//! guesses we only need the *ordering*, not absolute bits. Minimizing the
//! sum of pattern-bucket squares `Σ nᵢ²` is the same argmax as maximizing
//! entropy (both are Schur-concave in the bucket-size distribution) while
//! needing zero floats/logs. Scores are lower-is-better.

use crate::bitset::Survivors;
use crate::evaluator::evaluate;
use crate::pattern::ALL;
use crate::word::Word;

pub struct Scorer {
    /// Reused histogram buffer (never re-allocated during a search).
    histogram: [u16; ALL],
}

impl Scorer {
    pub fn new() -> Self {
        Scorer {
            histogram: [0; ALL],
        }
    }

    /// Sum of squared pattern-bucket sizes over survivors. Lower is better.
    pub fn score(&mut self, guess: Word, survivors: &Survivors, answers: &[Word]) -> u32 {
        let mut touched = [false; ALL];
        for a in survivors.iter() {
            let p = evaluate(guess, answers[a]).0 as usize;
            self.histogram[p] += 1;
            touched[p] = true;
        }
        let mut s = 0;
        for (i, &n) in self.histogram.iter().enumerate() {
            if touched[i] {
                s += (n as u32) * (n as u32);
            }
        }
        self.histogram = [0; ALL];
        s
    }
}

impl Default for Scorer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bitset::Survivors;

    #[test]
    fn score_equals_bucket_count_for_singletons() {
        let words: Vec<Word> = ["crane", "crate", "plumb"]
            .iter()
            .map(|w| Word::parse(w).unwrap())
            .collect();
        let s = Survivors::all(words.len());
        let mut sc = Scorer::new();
        // splits into distinct buckets → sum of 1s = n (== 3)
        assert_eq!(sc.score(Word::parse("crane").unwrap(), &s, &words), 3);
        // all one bucket (no letters match) → n² = 9
        assert_eq!(sc.score(Word::parse("qzwxv").unwrap(), &s, &words), 9);
    }
}
