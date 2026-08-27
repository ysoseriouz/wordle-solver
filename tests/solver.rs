//! End-to-end: play full games against actual answer words using the public API.

use wordle_solver::evaluator::evaluate;
use wordle_solver::pattern::Pattern;
use wordle_solver::wordlist::WordLists;
use wordle_solver::{Solver, Word};

/// Play one game against `answer`, returning guesses used (or u32::MAX on loss).
fn play(answer: &Word) -> u32 {
    let mut s = Solver::new();
    for used in 1..=6 {
        let guess = s.suggest_guess().unwrap();
        let p = evaluate(Word::parse(&guess).unwrap(), *answer);
        if p == Pattern::from_tiles([2; 5]) {
            return used;
        }
        s.apply_feedback(&guess, &p.to_feedback_str()).unwrap();
    }
    u32::MAX
}

/// Filtering invariant: after every feedback, each surviving candidate must
/// reproduce exactly the applied pattern.
#[test]
fn filtering_invariant() {
    let mut s = Solver::new();
    for guess in ["roate", "snare", "crane"] {
        let sample = s.remaining_words();
        if sample.is_empty() {
            break;
        }
        let ans = Word::parse(&sample[0]).unwrap();
        let g = Word::parse(guess).unwrap();
        // Simulate an "answer" drawn from the current candidates.
        let p = evaluate(g, ans);
        s.apply_feedback(guess, &p.to_feedback_str()).unwrap();
        for cand in s.remaining_words() {
            let c = Word::parse(&cand).unwrap();
            assert_eq!(
                evaluate(g, c),
                p,
                "survivor {cand} must match pattern of {guess}"
            );
        }
    }
}

/// Full-game completion over a 200-word sample: must always win in ≤ 6.
#[test]
fn solves_sample_of_answers() {
    let answers = sample_answers(WordLists::embedded(), 200);
    let (solved, max, avg) = run_all(&answers);
    assert_eq!(solved, answers.len() as i64);
    assert!(max <= 6, "some answer needed >6 guesses");
    assert!(avg < 4.0, "avg guesses too high: {avg}");
}

/// Exhaustive: every one of the 2,315 answers must be solved in ≤ 6, avg < 4.
/// Slow (minutes in debug); run explicitly: `cargo test --release -- --ignored`.
#[test]
#[ignore]
fn solves_all_answers() {
    let lists = WordLists::embedded();
    let answers: Vec<&Word> = lists.answers.iter().collect();
    let (solved, max, avg) = run_all(&answers);
    assert_eq!(solved, answers.len() as i64, "not all answers solved");
    assert!(max <= 6, "max guesses exceeded Wordle limit: {max}");
    assert!(avg < 4.0, "avg guesses too high: {avg}");
    println!("solved={solved} max={max} avg={avg:.3}");
}

/// Sample `count` answers spread evenly across the answer list.
fn sample_answers(lists: &WordLists, count: usize) -> Vec<&Word> {
    let step = (lists.answers.len() / count).max(1);
    lists.answers.iter().step_by(step).collect()
}

fn run_all(answers: &[&Word]) -> (i64, u32, f64) {
    let mut solved = 0i64;
    let mut max_guesses = 0u32;
    let mut total = 0i64;
    for a in answers {
        let g = play(a);
        solved += 1;
        max_guesses = max_guesses.max(g);
        total += g as i64;
    }
    let avg = total as f64 / solved as f64;
    (solved, max_guesses, avg)
}
