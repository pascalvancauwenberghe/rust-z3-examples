use criterion::{criterion_group, criterion_main, Criterion};
use rust_z3_examples::games::{difficult_sudoku1, not_fun_sudoku1};
use rust_z3_examples::sudoku::Game;
use z3::{Config, Context};

fn solve_notfun_sudoku() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let mut game = Game::new(&ctx, "not fun", not_fun_sudoku1());
    game.solve();
}

fn solve_difficult_sudoku() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let mut game = Game::new(&ctx, "difficult", difficult_sudoku1());
    game.solve();
}

fn solve_cagedsudoku() {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let mut game = Game::new(&ctx, "difficult", difficult_sudoku1());

    game.cage_of_3(Game::position_of(2, 2), Game::position_of(3, 2), Game::position_of(4, 2), 7);
    game.cage_of_2(Game::position_of(2, 6), Game::position_of(2, 7), 5);
    game.cage_of_2(Game::position_of(3, 7), Game::position_of(3, 8), 6);
    game.cage_of_2(Game::position_of(4, 6), Game::position_of(4, 7), 6);
    game.cage_of_3(Game::position_of(6, 3), Game::position_of(6, 4), Game::position_of(7, 4), 23);
    game.cage_of_2(Game::position_of(7, 3), Game::position_of(8, 3), 17);
    game.cage_of_2(Game::position_of(6, 9), Game::position_of(7, 9), 15);
    game.cage_of_2(Game::position_of(9, 6), Game::position_of(9, 7), 3);

    game.solve();
}

fn notfun_benchmark(c: &mut Criterion) {
    c.bench_function("not fun", |b| b.iter(|| solve_notfun_sudoku()));
}

fn difficult_benchmark(c: &mut Criterion) {
    c.bench_function("difficult", |b| b.iter(|| solve_difficult_sudoku()));
}

fn caged_benchmark(c: &mut Criterion) {
    c.bench_function("caged", |b| b.iter(|| solve_cagedsudoku()));
}

criterion_group!(benches, notfun_benchmark, difficult_benchmark,caged_benchmark);
criterion_main!(benches);
