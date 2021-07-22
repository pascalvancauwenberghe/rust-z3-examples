// Examples from https://sandiway.arizona.edu/sudoku/examples.html

use crate::sudoku::Game;

pub fn easy_sudoku() -> &'static str {
    r#"
...26.7.1
68..7..9.
19...45..
82.1...4.
..46.29..
.5...3.28
..93...74
.4..5..36
7.3.18...
"#
}

pub fn easy_sudoku_solution() -> &'static str {
    r#"
435269781
682571493
197834562
826195347
374682915
951743628
519326874
248957136
763418259
"#
}

pub fn easy_sudoku2() -> &'static str {
    r#"
1..489..6
73.....4.
.....1295
..712.6..
5..7.3..8
..6.957..
9146.....
.2.....37
8..512..4
"#
}

pub fn easy_sudoku2_solution() -> &'static str {
    r#"
152489376
739256841
468371295
387124659
591763428
246895713
914637582
625948137
873512964
"#
}

pub fn intermediate_sudoku1() -> &'static str {
    r#"
.2.6.8...
58...97..
....4....
37....5..
6.......4
..8....13
....2....
..98...36
...3.6.9.
"#
}

pub fn intermediate_sudoku1_solution() -> &'static str {
    r#"
123678945
584239761
967145328
372461589
691583274
458792613
836924157
219857436
745316892
"#
}

pub fn difficult_sudoku1() -> &'static str {
    r#"
...6..4..
7....36..
....91.8.
.........
.5.18...3
...3.6.45
.4.2...6.
9.3......
.2....1..
"#
}

pub fn difficult_sudoku1_solution() -> &'static str {
    r#"
581672439
792843651
364591782
438957216
256184973
179326845
845219367
913768524
627435198
"#
}

pub fn not_fun_sudoku1() -> &'static str {
    r#"
.2.......
...6....3
.74.8....
.....3..2
.8..4..1.
6..5.....
....1.78.
5....9...
.......4.
"#
}

pub fn not_fun_sudoku1_solution() -> &'static str {
    r#"
126437958
895621473
374985126
457193862
983246517
612578394
269314785
548769231
731852649
"#
}

pub fn sudoku_without_numbers() -> &'static str {
    r#"
.........
.........
.........
.........
.........
.........
.........
.........
"#
}

// Miracle Killer sudoku with cages https://www.youtube.com/watch?v=ejhtYYvUs5M
pub fn sudoku_with_cages_solution() -> &'static str {
    r#"
693154872
815672394
742398516
927485163
381267945
456913728
139826457
278549631
564731289
"#
}

pub fn add_cage_constraints(game: &mut Game) {
    game.cage_of_3(
        Game::position_of(2, 2),
        Game::position_of(3, 2),
        Game::position_of(4, 2),
        7,
    );
    game.cage_of_2(Game::position_of(2, 6), Game::position_of(2, 7), 5);
    game.cage_of_2(Game::position_of(3, 7), Game::position_of(3, 8), 6);
    game.cage_of_2(Game::position_of(4, 6), Game::position_of(4, 7), 6);
    game.cage_of_3(
        Game::position_of(6, 3),
        Game::position_of(6, 4),
        Game::position_of(7, 4),
        23,
    );
    game.cage_of_2(Game::position_of(7, 3), Game::position_of(8, 3), 17);
    game.cage_of_2(Game::position_of(6, 9), Game::position_of(7, 9), 15);
    game.cage_of_2(Game::position_of(9, 6), Game::position_of(9, 7), 3);
}

// Gravity sudoku with Thermos https://www.youtube.com/watch?v=-kym5UAVA7I
// A 'thermo(meter)' is a group of cells that must have strictly increasing digits, starting at the "bulb". For example:
// 0 = = = =>
// 2 3 4 5 6
// 1 5 7 8 9
pub fn add_thermo_constraints(game: &mut Game) {
    game.thermo_3(
        Game::position_of(1, 4),
        Game::position_of(2, 3),
        Game::position_of(2, 2),
    );
    game.thermo_7(
        Game::position_of(1, 5),
        Game::position_of(1, 6),
        Game::position_of(2, 7),
        Game::position_of(3, 7),
        Game::position_of(3, 8),
        Game::position_of(4, 9),
        Game::position_of(5, 9),
    );
    game.thermo_4(
        Game::position_of(3, 4),
        Game::position_of(4, 5),
        Game::position_of(5, 6),
        Game::position_of(5, 7),
    );
    game.thermo_4(
        Game::position_of(4, 3),
        Game::position_of(5, 4),
        Game::position_of(6, 5),
        Game::position_of(7, 5),
    );
    game.thermo_4(
        Game::position_of(4, 8),
        Game::position_of(3, 9),
        Game::position_of(2, 9),
        Game::position_of(1, 9),
    );
    game.thermo_5(
        Game::position_of(6, 1),
        Game::position_of(7, 2),
        Game::position_of(7, 3),
        Game::position_of(8, 3),
        Game::position_of(9, 4),
    );
    game.thermo_5(
        Game::position_of(8, 5),
        Game::position_of(8, 4),
        Game::position_of(9, 3),
        Game::position_of(9, 2),
        Game::position_of(9, 1),
    );
}

pub fn sudoku_with_thermos_solution() -> &'static str {
    r#"
645723189
298165473
731489562
382954617
457316928
169278354
523697841
814532796
976841235
"#
}
