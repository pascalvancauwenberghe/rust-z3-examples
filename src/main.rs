use rust_z3_examples::games;
use rust_z3_examples::games::{add_cage_constraints, add_thermo_constraints};
use rust_z3_examples::sudoku::Game;
use std::time::Instant;
use z3::{Config, Context};
use rust_z3_examples::word_game::SendMoreMoney;

fn main() {
    solve("easiest 1", games::easy_sudoku());
    solve("easiest 2", games::easy_sudoku2());
    solve("intermediate", games::intermediate_sudoku1());
    solve("difficult", games::difficult_sudoku1());
    solve("not fun", games::not_fun_sudoku1());
    solve_caged("cages", games::sudoku_without_numbers());
    solve_thermos("thermos", games::sudoku_without_numbers());
    solve_word_game();
}

fn solve(name: &str, initial_values: &str) {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let mut game = Game::new(&ctx, name, initial_values);
    let now = Instant::now();
    let solved = game.solve();
    let elapsed = now.elapsed().as_millis();
    if solved {
        println!(
            "Solution for game '{}' in {} ms:{}",
            game.name, elapsed, game
        );
    } else {
        println!(
            "No solution for game '{}' in {} ms: {}",
            game.name, elapsed, game
        );
    }
}

// No initial values are given
// Extra constraint: "cages"
// A cage defines 2 constraints:
// - the sum of the values in the cage must be the given sum
// - the same value can't appear twice in a cage
// This is the "Miracle Killer Sudoku" of "Cracking the Cryptic" (https://www.youtube.com/watch?v=ejhtYYvUs5M)
fn solve_caged(name: &str, initial_values: &str) {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let mut game = Game::new(&ctx, name, initial_values);

    add_cage_constraints(&mut game);

    let now = Instant::now();
    let solved = game.solve();
    let elapsed = now.elapsed().as_millis();
    if solved {
        println!(
            "Solution for game '{}' in {} ms:{}",
            game.name, elapsed, game
        );
    } else {
        println!(
            "No solution for game '{}' in {} ms: {}",
            game.name, elapsed, game
        );
    }
}

fn solve_thermos(name: &str, initial_values: &str) {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let mut game = Game::new(&ctx, name, initial_values);

    add_thermo_constraints(&mut game);

    let now = Instant::now();
    let solved = game.solve();
    let elapsed = now.elapsed().as_millis();
    if solved {
        println!(
            "Solution for game '{}' in {} ms:{}",
            game.name, elapsed, game
        );
    } else {
        println!(
            "No solution for game '{}' in {} ms: {}",
            game.name, elapsed, game
        );
    }
}

fn solve_word_game() {
    let mut cfg = Config::new();
    cfg.set_model_generation(true);
    cfg.set_proof_generation(true);
    let ctx = Context::new(&cfg);

    let puzzle = SendMoreMoney::new(&ctx);

    let now = Instant::now();
    let solved = puzzle.solve();
    let elapsed = now.elapsed().as_millis();
    println!("Solved word puzzle in {} ms: {}", elapsed, solved);

    let send_value =
        puzzle.value_of(&puzzle.s) * 1000 +
            puzzle.value_of(&puzzle.e) * 100 +
            puzzle.value_of(&puzzle.n) * 10 +
            puzzle.value_of(&puzzle.d) * 1;
    println!("Send =   {}", send_value);
    let more_value =
        puzzle.value_of(&puzzle.m) * 1000 +
            puzzle.value_of(&puzzle.o) * 100 +
            puzzle.value_of(&puzzle.r) * 10 +
            puzzle.value_of(&puzzle.e) * 1;
    println!("More =   {}", more_value);
    let money_value =
        puzzle.value_of(&puzzle.m) * 10000 +
            puzzle.value_of(&puzzle.o) * 1000 +
            puzzle.value_of(&puzzle.n) * 100 +
            puzzle.value_of(&puzzle.e) * 10 +
            puzzle.value_of(&puzzle.y) * 1;
    println!("Money = {}", money_value);
}

