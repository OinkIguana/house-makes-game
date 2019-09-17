#[derive(Copy, Clone, Debug)]
pub enum Command {
    Quit,
    Invalid,
}

impl Command {
    pub fn parse<S: AsRef<str>>(s: S) -> Self {
        match s.as_ref().to_lowercase().as_str() {
            "quit" => Command::Quit,
            _ => Command::Invalid,
        }
    }
}
