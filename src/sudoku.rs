use std::fmt;

pub struct Game {
    pub name: String,
     values: String
}

impl Game {
    pub fn new(game_name: &str, initial: &'static str) -> Self {
        let result = Self {
            name: game_name.to_string(),
            values : initial.to_string()
        };
        result
    }

    pub fn solve(&mut self) -> bool {
        false
    }
}

// Default to_string
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.values)
    }
}