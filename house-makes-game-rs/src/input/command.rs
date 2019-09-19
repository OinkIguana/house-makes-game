/// A command received from the player, which the game engine should respond to without affecting
/// the story.
#[derive(Copy, Clone, Debug)]
pub enum Command {
    /// Quit the game
    Quit,
    /// The input was processed as a command, but the command was not recognized
    Invalid,
}

impl Command {
    /// Maps command names to their values. Commands are case-insensitive. The leading backslash
    /// should have been stripped off already.
    pub fn parse<S: AsRef<str>>(s: S) -> Self {
        match s.as_ref().to_lowercase().as_str() {
            "quit" => Command::Quit,
            _ => Command::Invalid,
        }
    }
}
