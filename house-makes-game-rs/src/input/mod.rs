mod command;

pub use command::Command;

#[derive(Clone, Debug)]
pub enum Input<'a> {
    Command(Command),
    Action(&'a str),
}

impl<'a> Input<'a> {
    pub fn parse<S: AsRef<str> + 'a>(string: &'a S) -> Self {
        let s = string.as_ref();
        match s.chars().next() {
            Some('\\') => Input::Command(Command::parse(&s[1..])),
            _ => Input::Action(&s)
        }
    }
}
