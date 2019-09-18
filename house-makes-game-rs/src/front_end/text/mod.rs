use std::sync::mpsc;

use crate::message::Message;
use crate::input::Input;

pub struct Text;

impl Text {
    pub fn new() -> Self { Text }
}

struct Renderer {
    inputs: mpsc::Sender<Input>,
    messages: mpsc::Receiver<Message>,
}

impl super::FrontEnd for Text {
    fn split(self) -> (Box<dyn super::Renderer>, super::Channel) {
        let (sender, messages) = mpsc::channel();
        let (inputs, receiver) = mpsc::channel();
        (Box::new(Renderer { inputs, messages }), (Box::new(sender), Box::new(receiver)))
    }
}

impl super::Renderer for Renderer {
    fn game_loop(&self) { unimplemented!() }
}
