// Node smoke test for the wasm-pack web build.
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import init, { createSolver } from "../pkg/wordle_solver.js";

const __dirname = fileURLToPath(new URL(".", import.meta.url));
const wasm = readFileSync(new URL("../pkg/wordle_solver_bg.wasm", import.meta.url));

await init(wasm);

const assert = (cond, msg) => { if (!cond) throw new Error("FAIL: " + msg); };

const s = createSolver();
assert(typeof s.suggestGuess === "function", "suggestGuess is a function");
assert(typeof s.remainingCount === "function", "remainingCount is a function");
assert(s.remainingCount() === 2315, `initial remainingCount === 2315 (got ${s.remainingCount()})`);

// First call returns the optimal opener.
const g1 = s.suggestGuess();
assert(typeof g1 === "string" && g1.length === 5, `first guess is a 5-letter string (got ${g1})`);
console.log("opener:", g1);

// Feed a real shape of feedback and check the set shrinks and stays consistent.
const before = s.remainingCount();
s.applyFeedback(g1, "BBBYB");
const after = s.remainingCount();
assert(after > 0 && after < before, `candidate set shrank (${before} -> ${after})`);
console.log("remaining:", after, "candidates:", s.remainingCandidates().slice(0, 3));

// Invalid input throws.
let threw = false;
try { s.applyFeedback("zzzzz", "BBBBB"); } catch (e) { threw = true; }
assert(threw, "illegal guess throws");

s.reset();
assert(s.remainingCount() === 2315, "reset returns to 2315");

console.log("OK ✅ all binding smoke checks passed");