//! Offline tool: compute the single best opening guess (D5 constant).
//!
//! Runs the full first-turn search once (12,972 guesses × 2,315 answers) and
//! prints the optimal opener.
//! Run: `cargo run --release --bin best_opener`

use wordle_solver::{bitset::Survivors, entropy::Scorer, word::Word, wordlist::WordLists};

fn main() {
    let lists = WordLists::embedded();
    let survivors = Survivors::all(lists.answers.len());
    let mut scorer = Scorer::new();

    let mut best: Option<(u32, bool, Word)> = None;
    for &guess in lists.allowed.iter() {
        let score = scorer.score(guess, &survivors, &lists.answers);
        let is_survivor = lists.answers.binary_search(&guess).is_ok();
        let cand = (score, is_survivor, guess);
        if best.is_none_or(|b| (cand.0, !cand.1, cand.2 .0) < (b.0, !b.1, b.2 .0)) {
            best = Some(cand);
        }
    }
    let (score, _, word) = best.unwrap();
    println!("{} {score}", word);
}
