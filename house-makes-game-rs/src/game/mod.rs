//! Defines the game frontend abstractions

mod regular;
mod text;

/// A frontend for the game. Provides an abstraction for methods related to getting user input and
/// returning feedback to the player.
pub trait Game {
    /// Reads a line of input from the player
    fn read_line(&self) -> String;
}

pub use regular::RegularGame;
pub use text::TextGame;
