use rust_z3_examples::word_game::SendMoreMoney;
use z3::{Context, Config};

#[test]
fn test_can_solve_send_more_money() {
    let mut cfg = Config::new();
    cfg.set_model_generation(true) ;
    cfg.set_proof_generation(true) ;
    let ctx = Context::new(&cfg);

    let puzzle = SendMoreMoney::new(&ctx);

    let solved = puzzle.solve();

    assert!(solved);

    let send_value =
        puzzle.value_of(&puzzle.s) * 1000 +
            puzzle.value_of(&puzzle.e) * 100 +
            puzzle.value_of(&puzzle.n) * 10 +
            puzzle.value_of(&puzzle.d) * 1;
    let more_value =
        puzzle.value_of(&puzzle.m) * 1000 +
            puzzle.value_of(&puzzle.o) * 100 +
            puzzle.value_of(&puzzle.r) * 10 +
            puzzle.value_of(&puzzle.e) * 1;
    let money_value =
        puzzle.value_of(&puzzle.m) * 10000 +
            puzzle.value_of(&puzzle.o) * 1000 +
            puzzle.value_of(&puzzle.n) * 100 +
            puzzle.value_of(&puzzle.e) * 10 +
            puzzle.value_of(&puzzle.y) * 1;

    assert_eq!(9567, send_value);
    assert_eq!(1085, more_value);
    assert_eq!(10652, money_value);
    assert_eq!(send_value + more_value, money_value);
}

