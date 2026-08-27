//! Benchmarks: evaluator throughput and full suggest_guess per turn depth.
//! Run with `cargo bench`.

use criterion::{criterion_group, criterion_main, Criterion};
use wordle_solver::bitset::Survivors;
use wordle_solver::entropy::Scorer;
use wordle_solver::evaluator::evaluate;
use wordle_solver::solver::Solver;
use wordle_solver::word::Word;
use wordle_solver::wordlist::WordLists;

fn bench(c: &mut Criterion) {
    let lists = WordLists::embedded();
    let a = lists.answers[0];
    let b = lists.answers[1];

    c.bench_function("evaluate (pairs)", |bench| {
        bench.iter(|| {
            for &w in lists.answers.iter() {
                criterion::black_box(evaluate(a, w));
                let _ = b;
            }
        })
    });

    let mut solver = Solver::new();
    // Turn ~2: ~200 survivors.
    solver.apply_feedback("roate", "BBBYB").unwrap();
    c.bench_function("suggest_guess turn~2", |bench| {
        bench.iter(|| solver.suggest_guess())
    });

    let survivors = Survivors::all(lists.answers.len());
    let mut scorer = Scorer::new();
    c.bench_function("score one guess (full set)", |bench| {
        bench.iter(|| scorer.score(lists.allowed[0], &survivors, &lists.answers))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);