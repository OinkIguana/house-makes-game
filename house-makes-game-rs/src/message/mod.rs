/// Messages are sent from the internals of the game to the front end. The front end needs to be
/// able to handle each of these messages correctly
#[derive(Clone, Debug)]
pub enum Message {
    /// The game loop should end
    Quit,
}
