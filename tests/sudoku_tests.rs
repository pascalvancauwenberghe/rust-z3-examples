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
    let mut game = Game::new(&ctx, "sudoku with cages", sudoku_with_cages());

    game.cage_of_3( Game::position_of(2, 2), Game::position_of(3, 2), Game::position_of(4, 2), 7);
    game.cage_of_2( Game::position_of(2, 6), Game::position_of(2, 7), 5);
    game.cage_of_2( Game::position_of(3, 7), Game::position_of(3, 8), 6);
    game.cage_of_2( Game::position_of(4, 6), Game::position_of(4, 7), 6);
    game.cage_of_3( Game::position_of(6, 3), Game::position_of(6, 4), Game::position_of(7, 4), 23);
    game.cage_of_2( Game::position_of(7, 3), Game::position_of(8, 3), 17);
    game.cage_of_2( Game::position_of(6, 9), Game::position_of(7, 9), 15);
    game.cage_of_2( Game::position_of(9, 6), Game::position_of(9, 7), 3);
    
    game.solve();
    assert_eq!(sudoku_with_cages_solution(), game.to_string());
}
