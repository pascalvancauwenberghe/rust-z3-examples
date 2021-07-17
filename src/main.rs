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
}

fn solve(name: &str, initial_values: &str) {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let mut game = Game::new(&ctx, name, initial_values);
    let now = Instant::now();
    let solved = game.solve();
    let elapsed = now.elapsed().as_micros();
    if solved {
        println!(
            "Solution for game '{}' in {} µs:{}",
            game.name, elapsed, game
        );
    } else {
        println!(
            "No solution for game '{}' in {} µs: {}",
            game.name, elapsed, game
        );
    }
}
