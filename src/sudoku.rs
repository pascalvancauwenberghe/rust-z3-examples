use z3::{Solver, Context, SatResult, ast};
use z3::ast::{Int, Ast};
use std::fmt;

pub struct Game<'a> {
    pub name: String,
    solver: Solver<'a>,
    variables: Vec<Int<'a>>,
}

impl<'a> Game<'a> {
    fn make_int_var(ctx: &Context, row: usize, col: usize) -> Int {
        Int::new_const(&ctx, "cell".to_string() + &row.to_string() + &col.to_string())
    }

    fn position_of(row: usize, col: usize) -> usize {
        (row - 1) * 9 + (col - 1)
    }

    pub fn new(ctx: &'a Context, game_name: &str, initial: &str) -> Self {
        let mut result = Self {
            name: game_name.to_string(),
            solver: Solver::new(ctx),
            variables: Vec::with_capacity(81),
        };
        let parsed_values = parse_initial_sudoku_values(initial);
        for row in 1..=9 {
            for col in 1..=9 {
                let predefined = parsed_values[(row - 1) * 9 + (col - 1)];
                if predefined > 0 {
                    result.variables.push(Int::from_i64(ctx, predefined));
                } else {
                    result.variables.push(Game::make_int_var(ctx, row, col));
                }
            }
        }
        result.basic_sudoku_rules();
        result
    }

    pub fn solve(&mut self) -> bool {
        let result = self.solver.check();
        result == SatResult::Sat
    }

    fn basic_sudoku_rules(&self) {
        let ctx = &self.solver.get_context();
        let one = ast::Int::from_i64(&ctx, 1);
        let nine = ast::Int::from_i64(&ctx, 9);

        self.all_variables_must_have_value_between(&one, &nine);
        Game::unique_values_in_rows(&&self.solver, &ctx, &&self.variables);
        Game::unique_values_in_columns(&&self.solver, &ctx, &&self.variables);
        Game::unique_values_in_subgrids(&&self.solver, &ctx, &&self.variables);
    }

    fn all_variables_must_have_value_between(&self, one: &Int, nine: &Int) {
        for variable in self.variables.iter() {
            self.solver.assert(&variable.ge(one));
            self.solver.assert(&variable.le(nine));
        }
    }

    fn unique_values_in_row(solver: &Solver, ctx: &Context, variables: &[Int], row: usize) {
        let mut cells = Vec::new();
        for col in 1..=9 {
            let cell = variables.get(Game::position_of(row, col)).unwrap();
            cells.push(cell);
        }
        solver.assert(&Ast::distinct(ctx, &cells));
    }

    fn unique_values_in_rows(solver: &Solver, ctx: &Context, variables: &[Int]) {
        for row in 1..=9 {
            Game::unique_values_in_row(solver, ctx, variables, row);
        }
    }

    fn unique_values_in_column(solver: &Solver, ctx: &Context, variables: &[Int], col: usize) {
        let mut cells = Vec::new();
        for row in 1..=9 {
            let cell = variables.get(Game::position_of(row, col)).unwrap();
            cells.push(cell);
        }
        solver.assert(&Ast::distinct(ctx, &cells));
    }

    fn unique_values_in_columns(solver: &Solver, ctx: &Context, variables: &[Int]) {
        for col in 1..=9 {
            Game::unique_values_in_column(solver, ctx, variables, col);
        }
    }

    fn unique_values_in_subgrid(solver: &Solver, ctx: &Context, variables: &[Int], rowgrid: usize, colgrid: usize) {
        let mut cells = Vec::new();

        for r in 1..=3 {
            for c in 1..=3 {
                let cell_row = rowgrid + r;
                let cell_col = colgrid + c;
                let cell = variables.get(Game::position_of(cell_row, cell_col)).unwrap();
                cells.push(cell);
            }
        }
        solver.assert(&Ast::distinct(ctx, &cells));
    }

    fn unique_values_in_subgrids(solver: &Solver, ctx: &Context, variables: &[Int]) {
        for rowgrid in 0..=2 {
            for colgrid in 0..=2 {
                Game::unique_values_in_subgrid(solver, ctx, variables, rowgrid * 3, colgrid * 3);
            }
        }
    }
}

// Parses a multi-line string with the starting values of the Sudoku
// Digit => value of the digit 1..9
// '.'   => 0
// values are arranged row per row
// Empty (less than 9 chars) are skipped
// The input doesn't have to contain 9 rows. If rows are missing, they are assumed to be empty (= 0 values)
fn parse_initial_sudoku_values(values: &str) -> [i64; 81] {
    let lines = values.lines();
    let mut result: [i64; 81] = [0; 81];
    let mut row = 0;
    for line in lines {
        if line.len() >= 9 {
            row += 1;
            for col in 1..=9 {
                let kar = line.chars().nth(col - 1).unwrap();
                if ('1'..='9').contains(&kar) {
                    result[Game::position_of(row, col)] = kar.to_digit(10).unwrap() as i64;
                }
            }
        }
    }

    result
}

// Default to_string
impl fmt::Display for Game<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let model = self.solver.get_model().unwrap();
        let mut output = String::new();
        output += "\n";
        for row in 1..=9 {
            for col in 1..=9 {
                let cell = &self.variables[Game::position_of(row, col)];
                let xv = model.eval(cell, true).unwrap().as_i64().unwrap();
                output.push_str(&xv.to_string());
            }
            output += "\n";
        }
        write!(f, "{}", output)
    }
}