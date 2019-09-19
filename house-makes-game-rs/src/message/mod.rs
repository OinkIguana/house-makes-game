//! Messages sent from the game engine to the front end to update the display

/// Messages are sent from the internals of the game to the front end. The front end needs to be
/// able to handle each of these messages correctly, relaying their effects to the user.
#[derive(Clone, Debug)]
pub enum Message {
    /// The game loop should end
    Quit,
}
