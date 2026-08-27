//! Interactive CLI: solve a live Wordle game with the user.
//!
//! Suggests a guess, you type the real Board feedback (5 chars of B/G/Y for
//! black/yellow/green), repeat until all green.
//! Run: `cargo run --release --bin play`

use std::io::{self, BufRead, Write};

use wordle_solver::Solver;

fn main() {
    let mut s = Solver::new();
    println!("Wordle CLI. Guess and enter feedback as 5 chars of B/G/Y (black/yellow/green).");
    println!("Ctrl-C to quit.\n");

    let stdin = io::stdin();
    let mut out = io::stdout();
    for turn in 1..=6u32 {
        let guess = match s.suggest_guess() {
            Some(g) => g,
            None => {
                println!("No candidates left — feedback may have been entered wrong.");
                return;
            }
        };
        println!("[{turn}/6] guess: {guess}");
        print!("  feedback (B/Y/G): ");
        out.flush().ok();

        let input = match stdin.lock().lines().next() {
            Some(Ok(l)) => l.trim().to_string(),
            _ => return,
        };

        // Empty input = it was the answer.
        if input.is_empty() {
            println!("Solved in {turn}!");
            return;
        }

        match s.apply_feedback(&guess, &input) {
            Ok(()) => {}
            Err(e) => {
                println!("  error: {e}");
                return;
            }
        }
        if input == "GGGGG" {
            println!("Solved in {turn}!");
            return;
        }
    }
    println!("Out of guesses.");
}
