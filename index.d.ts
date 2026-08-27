//! TypeScript definitions for `wordle-solver` (hand-written; see src/wasm.rs).

export type Tile = "B" | "Y" | "G";

/** A 5-character feedback string: one tile per guess letter, left → right. */
export type Feedback = `${Tile}${Tile}${Tile}${Tile}${Tile}`;

/**
 * Wordle solver whose candidate set you drive with real guesses + feedback.
 *
 * ```ts
 * const s = createSolver();
 * console.log(s.suggestGuess());          // e.g. "roate"
 * s.applyFeedback("roate", "BYBBG");
 * s.remainingCount();
 * s.remainingCandidates();
 * s.reset();
 * ```
 */
export class Solver {
  /** Create a fresh solver over the full answer set. */
  constructor();

  /** Best next guess (`null` when no candidates remain). */
  suggestGuess(): string | null;

  /**
   * Record the result of a guess.
   * @throws on non-letter/length guess, unknown word, bad feedback, or a
   *         feedback string that contradicts all remaining candidates.
   */
  applyFeedback(guess: string, feedback: Feedback): void;

  /** Number of words still consistent with all recorded feedback. */
  remainingCount(): number;

  /** The remaining candidate words (not guaranteed sorted). */
  remainingCandidates(): string[];

  /** Start over from the full answer set. */
  reset(): void;
}

/** Construct a new solver (the exported factory function). */
export function createSolver(): Solver;

export default createSolver;