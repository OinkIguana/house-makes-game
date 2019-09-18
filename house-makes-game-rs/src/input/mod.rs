mod command;

pub use command::Command;

#[derive(Clone, Debug)]
pub enum Input {
    Command(Command),
    Action(String)
}

impl Input {
    pub fn parse<S: AsRef<str>>(string: S) -> Self {
        let s = string.as_ref();
        match s.chars().next() {
            Some('\\') => Input::Command(Command::parse(s[1..].to_string())),
            _ => Input::Action(s.to_string())
        }
    }
}
