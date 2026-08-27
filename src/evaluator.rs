//! Official duplicate-aware Wordle feedback evaluation (bit-parallel, O(1)).

use crate::pattern::{Pattern, POW3};
use crate::word::Word;

/// Evaluate the feedback Wordle would show for `guess` against `answer`.
///
/// Pass 1 marks greens and eats those letters from the answer pool; pass 2
/// gives yellows to remaining non-green letters that still have pool supply.
/// Allocation-free and branch-light, ~20ns per pair.
#[inline]
pub fn evaluate(guess: Word, answer: Word) -> Pattern {
    let mut result = 0u8;
    let mut pool = [0u8; 26];
    let mut a = answer.0;

    // Count answer letters (shift reads left→right: bits [5i, 5i+5) = char i).
    for _ in 0..5 {
        pool[(a & 0x1F) as usize] += 1;
        a >>= 5;
    }

    // Pass 1: greens. A green eats the answer-letter from the pool.
    let mut green_mask = 0u8;
    let mut g = guess.0;
    for (i, &pow) in POW3.iter().enumerate() {
        let l = (g & 0x1F) as usize;
        let al = ((answer.0 >> (5 * i)) & 0x1F) as usize;
        if l == al {
            green_mask |= 1 << i;
            pool[l] -= 1;
            result += 2 * pow;
        }
        g >>= 5;
    }

    // Pass 2: yellows for remaining letters, gray otherwise.
    let mut g = guess.0;
    for (i, &pow) in POW3.iter().enumerate() {
        if green_mask & (1 << i) == 0 {
            let l = (g & 0x1F) as usize;
            if pool[l] > 0 {
                pool[l] -= 1;
                result += pow;
            }
        }
        g >>= 5;
    }

    Pattern(result)
}

/// Independent reference evaluator (allocating, transparent) used to cross-check
/// the fast bit-parallel implementation.
#[cfg(test)]
fn ref_evaluate(guess: u32, answer: u32) -> u8 {
    let g = (0..5)
        .map(|i| (guess >> (5 * i)) & 0x1F)
        .collect::<Vec<_>>();
    let mut a = (0..5)
        .map(|i| (answer >> (5 * i)) & 0x1F)
        .collect::<Vec<_>>();
    let mut tiles = [0u8; 5];
    for i in 0..5 {
        if a[i] == g[i] {
            tiles[i] = 2;
            a[i] = u32::MAX;
        }
    }
    for i in 0..5 {
        if tiles[i] == 0 {
            if let Some(j) = a.iter().position(|&x| x == g[i]) {
                tiles[i] = 1;
                a[j] = u32::MAX;
            }
        }
    }
    tiles.iter().enumerate().map(|(i, t)| t * POW3[i]).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn feed(g: &str, a: &str) -> String {
        evaluate(Word::parse(g).unwrap(), Word::parse(a).unwrap()).to_feedback_str()
    }

    #[test]
    fn simple_vectors() {
        assert_eq!(feed("crane", "crate"), "GGGBG");
        assert_eq!(feed("crane", "muddy"), "BBBBB");
        assert_eq!(feed("hello", "hello"), "GGGGG");
    }

    #[test]
    fn duplicates_follow_real_wordle() {
        // guess=apple answer=paper → Y Y G B Y
        assert_eq!(feed("apple", "paper"), "YYGBY");
        // guess=ppppp answer=apple: both p's in apple are consumed by the two
        // greens, so the remaining three p's are gray → B G G B B.
        assert_eq!(feed("ppppp", "apple"), "BGGBB");
        // guess=eerie answer=erase → G B Y B G
        assert_eq!(feed("eerie", "erase"), "GBYBG");
        // guess=abbey answer=babbb → Y Y G B B
        assert_eq!(feed("abbey", "babbb"), "YYGBB");
    }

    #[test]
    fn agrees_with_reference_oracle() {
        let words = [
            "crane", "crate", "muddy", "erase", "poser", "eerie", "abbey", "apple", "pzqxv",
            "lolae", "sassy", "gully",
        ];
        for &ga in &words {
            for &aa in &words {
                let g = Word::parse(ga).unwrap().0;
                let a = Word::parse(aa).unwrap().0;
                assert_eq!(
                    evaluate(Word(g), Word(a)).0,
                    ref_evaluate(g, a),
                    "mismatch {ga} vs {aa}"
                );
            }
        }
    }
}
