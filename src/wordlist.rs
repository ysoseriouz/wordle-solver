//! Embedded word lists, parsed once at startup into sorted, unique `Word`s.

use crate::word::Word;
use std::sync::OnceLock;

const ANSWERS: &str = include_str!("../data/answers.txt");
const ALLOWED: &str = include_str!("../data/allowed.txt");

pub struct WordLists {
    /// Answers in id-space (ascending); `answers[a]` is answer id `a`.
    pub answers: Box<[Word]>,
    /// Legal guess space = superset of answers (ascending, unique).
    pub allowed: Box<[Word]>,
}

fn parse_sorted_unique(src: &str) -> Box<[Word]> {
    let mut words: Vec<Word> = src
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| Word::parse(l).unwrap_or_else(|| panic!("data list has an invalid word: {l:?}")))
        .collect();
    words.sort_by_key(|w| w.0);
    words.dedup_by_key(|w| w.0);
    words.into_boxed_slice()
}

impl WordLists {
    /// Lazily parse the embedded lists once. Panics on corrupt data.
    pub fn embedded() -> &'static WordLists {
        static L: OnceLock<Box<WordLists>> = OnceLock::new();
        L.get_or_init(|| {
            let lists = WordLists {
                answers: parse_sorted_unique(ANSWERS),
                allowed: parse_sorted_unique(ALLOWED),
            };
            debug_assert!(lists.allowed.len() > lists.answers.len());
            #[cfg(debug_assertions)]
            debug_assert!(
                lists
                    .answers
                    .iter()
                    .all(|a| lists.allowed.binary_search(a).is_ok()),
                "every answer must also be a valid guess"
            );
            Box::new(lists)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(b: &[Word]) -> bool {
        b.windows(2).all(|w| w[0].0 < w[1].0)
    }

    #[test]
    fn embedded_lists_match_spec_counts() {
        let l = WordLists::embedded();
        assert_eq!(l.answers.len(), 2_315);
        assert_eq!(l.allowed.len(), 12_972);
        assert!(sorted(&l.answers));
        assert!(sorted(&l.allowed));
    }
}
