use rust_z3_examples::games;
use rust_z3_examples::sudoku::Game;
use std::time::Instant;
use z3::{Config, Context};

fn main() {
    solve("easiest 1", games::easy_sudoku());
    solve("easiest 2", games::easy_sudoku2());
    solve("intermediate", games::intermediate_sudoku1());
    solve("difficult", games::difficult_sudoku1());
    solve("not fun", games::not_fun_sudoku1());
    solve_caged("cages", games::sudoku_with_cages());
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

    game.cage_of_3( Game::position_of(2, 2), Game::position_of(3, 2), Game::position_of(4, 2), 7);
    game.cage_of_2( Game::position_of(2, 6), Game::position_of(2, 7), 5);
    game.cage_of_2( Game::position_of(3, 7), Game::position_of(3, 8), 6);
    game.cage_of_2( Game::position_of(4, 6), Game::position_of(4, 7), 6);
    game.cage_of_3( Game::position_of(6, 3), Game::position_of(6, 4), Game::position_of(7, 4), 23);
    game.cage_of_2( Game::position_of(7, 3), Game::position_of(8, 3), 17);
    game.cage_of_2( Game::position_of(6, 9), Game::position_of(7, 9), 15);
    game.cage_of_2( Game::position_of(9, 6), Game::position_of(9, 7), 3);

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
