

// Acceptance tests with increasingly difficult Sudokus to solve

use rust_z3_examples::games::*;
use rust_z3_examples::sudoku::Game;

#[test]
fn test_can_solve_easy_sudoku() {
    let mut game = Game::new("easy", easy_sudoku());
    game.solve();
    assert_eq!(easy_sudoku_solution(), game.to_string());
}

#[test]
fn test_can_solve_second_easy_sudoku() {
    let mut game = Game::new("easy2", easy_sudoku2());
    game.solve();
    assert_eq!(easy_sudoku2_solution(), game.to_string());
}

#[test]
fn test_can_solve_intermediate_sudoku() {
    let mut game = Game::new("intermediate", intermediate_sudoku1());
    game.solve();
    assert_eq!(intermediate_sudoku1_solution(), game.to_string());
}

#[test]
fn test_can_solve_difficult_sudoku() {
    let mut game = Game::new("difficult", difficult_sudoku1());
    game.solve();
    assert_eq!(difficult_sudoku1_solution(), game.to_string());
}

#[test]
fn test_can_solve_notfun_sudoku() {
    let mut game = Game::new("not fun", not_fun_sudoku1());
    game.solve();
    assert_eq!(not_fun_sudoku1_solution(), game.to_string());
}
