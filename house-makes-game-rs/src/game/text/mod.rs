pub struct TextGame;

impl TextGame {
    pub fn new() -> Self { TextGame }
}

impl super::Game for TextGame {
    fn read_line(&self) -> String { read!("{}\n") }
}
