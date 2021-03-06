use criterion::{criterion_group, criterion_main, Criterion};
use rust_z3_examples::games::{add_cage_constraints, difficult_sudoku1, not_fun_sudoku1};
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

    add_cage_constraints(&mut game);

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

criterion_group!(
    benches,
    notfun_benchmark,
    difficult_benchmark,
    caged_benchmark
);
criterion_main!(benches);
