pub struct RegularGame;

impl RegularGame {
    pub fn new() -> Self { RegularGame }
}

impl super::Game for RegularGame {
    fn read_line(&self) -> String { String::from("\\quit") }
}
