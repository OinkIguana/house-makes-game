//! Defines the game front end abstractions

mod graphical;
mod text;

mod channel;

use crate::input::Input;
use crate::message::Message;

/// A front end for the game. Provides an abstraction for methods related to getting user input and
/// returning feedback to the player.
pub trait FrontEnd {
    /// Splits the Game in two
    fn split(self) -> (Box<dyn Renderer>, Channel);
}

pub trait Renderer {
    /// Passes control to the game loop. This method should not return until the player quits the
    /// game.
    fn game_loop(&self);
}

pub type Channel = (Box<dyn channel::Sender<Message>>, Box<dyn channel::Receiver<Input>>);

pub use graphical::Graphical;
pub use text::Text;
