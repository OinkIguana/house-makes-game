//! Handles parsing of inputs from the user

mod command;

pub use command::Command;

/// An input from the user in a very slightly parsed, but raw form.
#[derive(Clone, Debug)]
pub enum Input {
    /// A command (starts with \) which the player has issued. These do not relate to the story,
    /// but rather to the actual game engine.
    Command(Command),
    /// A regular input from the player, which has not been processed yet, but will be fed to the
    /// story after some processing.
    Action(String)
}

impl Input {
    /// Parses an input string, identifying system commands.
    pub fn parse<S: AsRef<str>>(string: S) -> Self {
        let s = string.as_ref();
        match s.chars().next() {
            // if the first character of the input is \, then it should be handled as a Command.
            Some('\\') => Input::Command(Command::parse(s[1..].to_string())),
            // otherwise, it is an action and should be handled by the story engine.
            _ => Input::Action(s.to_string())
        }
    }
}
