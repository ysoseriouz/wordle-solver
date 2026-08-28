//! Solver state machine: filter survivors, rank candidates, suggest guesses.

use crate::bitset::Survivors;
use crate::entropy::Scorer;
use crate::evaluator::evaluate;
use crate::pattern::Pattern;
use crate::word::Word;
use crate::wordlist::WordLists;

#[derive(Debug)]
pub enum Error {
    /// Guess isn't 5 lowercase ASCII letters.
    BadGuess,
    /// Guess isn't in the allowed list.
    UnknownWord,
    /// Feedback isn't 5 chars of B/G/Y.
    BadFeedback,
    /// Applying this feedback eliminates every remaining candidate.
    Contradiction,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Error::BadGuess => "guess is not 5 lowercase ASCII letters",
            Error::UnknownWord => "guess is not a valid Wordle word",
            Error::BadFeedback => "feedback must be 5 chars of B/G/Y",
            Error::Contradiction => "feedback contradicts earlier results (no candidates left)",
        })
    }
}

impl std::error::Error for Error {}

/// Optimal opening guess (max information over the full answer set), computed
/// offline by `src/bin/best_opener.rs` (score 139,883). Saves the ~30M-step
/// first-turn search.
const BEST_OPENER: &str = "roate";

pub struct Solver {
    answers: Box<[Word]>,
    allowed: Box<[Word]>,
    survivors: Survivors,
    /// Clone-play mode: Answers until feedback eliminates every official
    /// answer, then Allowed (clones accept allowed words as answers).
    mode: Mode,
    /// Applied (guess, feedback) constraints, replayed when switching mode.
    history: Vec<(Word, Pattern)>,
    scorer: Scorer,
}

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Answers,
    Allowed,
}

fn active_list<'a>(answers: &'a [Word], allowed: &'a [Word], mode: Mode) -> &'a [Word] {
    match mode {
        Mode::Answers => answers,
        Mode::Allowed => allowed,
    }
}

impl Solver {
    pub fn new() -> Self {
        let lists = WordLists::embedded();
        Solver {
            answers: lists.answers.clone(),
            allowed: lists.allowed.clone(),
            survivors: Survivors::all(lists.answers.len()),
            mode: Mode::Answers,
            history: Vec::new(),
            scorer: Scorer::new(),
        }
    }

    pub fn suggest_guess(&mut self) -> Option<String> {
        let n = self.survivors.count();
        if n == 0 {
            return None;
        }

        let base = active_list(&self.answers, &self.allowed, self.mode);

        // Turn 1: answer set unconstrained → hard-coded opener (D5).
        if n == self.answers.len() && self.mode == Mode::Answers {
            return Some(BEST_OPENER.to_string());
        }

        if n == 1 {
            return Some(base[self.survivors.iter().next().unwrap()].to_string());
        }

        // Scan the full guess space: with small survivor sets a discriminating
        // non-answer word (e.g. probing a shared-suffix family) must be
        // available, and scoring scales as guesses × survivors so it's cheap.
        let candidates: Vec<Word> = self.allowed.to_vec();

        // Best tuple: lower Σn² first, then is-survivor, then lexicographic word.
        // (Word u32 value == lex order for fixed-5 a-z words.) `best` guarded below.
        let mut best: Option<(u32, bool, Word)> = None;
        for &guess in &candidates {
            let score = self.scorer.score(guess, &self.survivors, base);
            let is_survivor = base.binary_search(&guess).is_ok();
            let cand = (score, is_survivor, guess);
            let take = match best {
                None => true,
                Some(b) => (cand.0, !cand.1, cand.2 .0) < (b.0, !b.1, b.2 .0),
            };
            if take {
                best = Some(cand);
            }
        }
        best.map(|(_, _, w)| w.to_string())
    }

    pub fn apply_feedback(&mut self, guess: &str, feedback: &str) -> Result<(), Error> {
        let w = Word::parse(guess).ok_or(Error::BadGuess)?;
        if self.allowed.binary_search(&w).is_err() {
            return Err(Error::UnknownWord);
        }
        let p = parse_feedback(feedback).ok_or(Error::BadFeedback)?;
        self.history.push((w, p));
        self.survivors
            .retain_matching(w, p, active_list(&self.answers, &self.allowed, self.mode));
        if self.survivors.count() == 0 && self.mode == Mode::Answers {
            // No official answer fits — the game may be a clone accepting
            // allowed words as answers. Replay every constraint on that list.
            self.mode = Mode::Allowed;
            self.survivors = Survivors::all(self.allowed.len());
            for &(g, pp) in &self.history {
                self.survivors.retain_matching(g, pp, &self.allowed);
            }
        }
        if self.survivors.count() == 0 {
            return Err(Error::Contradiction);
        }
        Ok(())
    }

    pub fn remaining_count(&self) -> u32 {
        self.survivors.count() as u32
    }

    pub fn remaining_words(&self) -> Vec<String> {
        let base = active_list(&self.answers, &self.allowed, self.mode);
        self.survivors
            .iter()
            .map(|a| base[a].to_string())
            .collect()
    }

    pub fn reset(&mut self) {
        self.survivors = Survivors::all(self.answers.len());
        self.mode = Mode::Answers;
        self.history.clear();
    }
}

impl Default for Solver {
    fn default() -> Self {
        Self::new()
    }
}

fn parse_feedback(s: &str) -> Option<Pattern> {
    if s.len() != 5 {
        return None;
    }
    let mut tiles = [0u8; 5];
    for (i, c) in s.bytes().enumerate() {
        tiles[i] = match c {
            b'B' => 0,
            b'Y' => 1,
            b'G' => 2,
            _ => return None,
        };
    }
    Some(Pattern::from_tiles(tiles))
}

// --- end-to-end gameplay helper living with the solver ---------------------------------

/// Play one full game against `answer`, returning the number of guesses used.
/// Uses the solver's own suggestions and the exact feedback.
pub fn play_game(answer: Word) -> u32 {
    let mut s = Solver::new();
    for turns in 1..=6u32 {
        let guess = s.suggest_guess().unwrap();
        let p = evaluate(Word::parse(&guess).unwrap(), answer);
        if p == Pattern::from_tiles([2; 5]) {
            return turns;
        }
        s.apply_feedback(&guess, &p.to_feedback_str()).unwrap();
    }
    7
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guesses_respect_candidates() {
        let mut s = Solver::new();
        assert_eq!(s.suggest_guess().unwrap(), BEST_OPENER);
    }

    #[test]
    fn feedback_filters_and_resets() {
        let mut s = Solver::new();
        // feedback for guessing "crane" against answer "crane" → all green,
        // so "crane" stays and the candidate set collapses to exactly it.
        s.apply_feedback("crane", "GGGGG").unwrap();
        assert_eq!(s.remaining_count(), 1);
        assert!(s.remaining_words().contains(&"crane".to_string()));
        s.reset();
        assert_eq!(s.remaining_count(), 2_315);
    }

    #[test]
    fn clone_play_falls_back_to_allowed() {
        let mut s = Solver::new();
        s.apply_feedback("roate", "BBBYY").unwrap();
        s.apply_feedback("sleet", "BBGGY").unwrap();
        // "tweed"→GGGGB matches no official answer (previously an error),
        // but matches allowed-only words under clone rules.
        s.apply_feedback("tweed", "GGGGB").unwrap();
        assert_eq!(
            s.remaining_words(),
            vec!["tween".to_string(), "tweep".to_string()]
        );
    }
}
