use z3::ast::{Int, Ast};
use z3::{Solver, Context, SatResult};

pub struct SendMoreMoney<'a> {
    pub s: Int<'a>,
    pub e: Int<'a>,
    pub n: Int<'a>,
    pub d: Int<'a>,

    pub m: Int<'a>,
    pub o: Int<'a>,
    pub r: Int<'a>,

    pub y: Int<'a>,
    solver: Solver<'a>,
}

// https://en.wikipedia.org/wiki/Verbal_arithmetic#Solving_cryptarithms
impl<'a> SendMoreMoney<'a> {
    pub fn new(ctx: &'a Context) -> Self {
        let result = Self {
            s: Int::new_const(&ctx, "S"),
            e: Int::new_const(&ctx, "E"),
            n: Int::new_const(&ctx, "N"),
            d: Int::new_const(&ctx, "D"),
            m: Int::new_const(&ctx, "M"),
            o: Int::new_const(&ctx, "O"),
            r: Int::new_const(&ctx, "R"),
            y: Int::new_const(&ctx, "Y"),

            solver: Solver::new(ctx),
        };
        result.establish_constraints();
        result
    }

    pub fn solve(self: &Self) -> bool {
        self.solver.check() == SatResult::Sat
    }

    pub fn value_of(self: &Self, parameter: &Int) -> i64 {
        let model = self.solver.get_model().unwrap();
        model.eval(parameter, true).unwrap().as_i64().unwrap()
    }

    fn establish_constraints(&self) {
        let ctx = self.solver.get_context() ;
        // Useful constants
        let zero = Int::from_i64(&ctx, 0);
        let nine = Int::from_i64(&ctx, 9);

        let ten = Int::from_i64(&ctx, 10);
        let hundred = Int::from_i64(&ctx, 100);
        let thousand = Int::from_i64(&ctx, 1000);
        let ten_thousand = Int::from_i64(&ctx, 10000);

        self.is_digit(&self.s, &zero, &nine);
        self.is_digit(&self.e, &zero, &nine);
        self.is_digit(&self.n, &zero, &nine);
        self.is_digit(&self.d, &zero, &nine);
        self.is_digit(&self.o, &zero, &nine);
        self.is_digit(&self.r, &zero, &nine);
        self.is_digit(&self.y, &zero, &nine);

        self.non_zero_digit(&self.m, &zero, &nine);

        self.solver.assert(&Ast::distinct(&ctx, &[&self.s, &self.e, &self.n, &self.d, &self.m, &self.o, &self.r, &self.y]));

        let send = &(&self.s * &thousand) + &(&self.e * &hundred) + &(&self.n * &ten) + &self.d;

        let more = &(&self.m * &thousand) + &(&self.o * &hundred) + &(&self.r * &ten) + &self.e;

        let money = &(&self.m * &ten_thousand) + &(&self.o * &thousand) + &(&self.n * &hundred) + &(&self.e * &ten) + &self.y;

        let send_more = &send + &more;

        self.solver.assert(&send_more._eq(&money));
    }

    fn is_digit(self: &Self, parameter: &Int, zero: &Int, nine: &Int) {
        self.solver.assert(&parameter.ge(&zero));
        self.solver.assert(&parameter.le(&nine));
    }

    fn non_zero_digit(self: &Self, parameter: &Int, zero: &Int, nine: &Int) {
        self.solver.assert(&parameter.gt(&zero));
        self.solver.assert(&parameter.le(&nine));
    }
}