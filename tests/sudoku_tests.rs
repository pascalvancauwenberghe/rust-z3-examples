// Acceptance tests with increasingly difficult Sudokus to solve

use rust_z3_examples::games::*;
use rust_z3_examples::sudoku::Game;
use z3::{Config, Context};

#[test]
fn test_can_solve_easy_sudoku() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let mut game = Game::new(&ctx, "easy", easy_sudoku());
    game.solve();
    assert_eq!(easy_sudoku_solution(), game.to_string());
}

#[test]
fn test_can_solve_second_easy_sudoku() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let mut game = Game::new(&ctx, "easy2", easy_sudoku2());
    game.solve();
    assert_eq!(easy_sudoku2_solution(), game.to_string());
}

#[test]
fn test_can_solve_intermediate_sudoku() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let mut game = Game::new(&ctx, "intermediate", intermediate_sudoku1());
    game.solve();
    assert_eq!(intermediate_sudoku1_solution(), game.to_string());
}

#[test]
fn test_can_solve_difficult_sudoku() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let mut game = Game::new(&ctx, "difficult", difficult_sudoku1());
    game.solve();
    assert_eq!(difficult_sudoku1_solution(), game.to_string());
}

#[test]
fn test_can_solve_notfun_sudoku() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let mut game = Game::new(&ctx, "not fun", not_fun_sudoku1());
    game.solve();
    assert_eq!(not_fun_sudoku1_solution(), game.to_string());
}

#[test]
fn test_can_solve_sudoku_with_cages() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let mut game = Game::new(&ctx, "sudoku with cages", sudoku_without_numbers());

    add_cage_constraints(&mut game);

    game.solve();
    assert_eq!(sudoku_with_cages_solution(), game.to_string());
}

#[test]
fn test_can_solve_sudoku_with_thermos() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let mut game = Game::new(&ctx, "sudoku with thermos", sudoku_without_numbers());

    add_thermo_constraints(&mut game);

    game.solve();
    assert_eq!(sudoku_with_thermos_solution(), game.to_string());
}
