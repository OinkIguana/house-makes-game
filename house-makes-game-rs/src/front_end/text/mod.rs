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
    fn game_loop(self: Box<Self>) {
        loop {
            // TODO: need to either read a line OR receive a Message here.
            let line: String = read!("{}\n");
            let input = Input::parse(&line);
            self.inputs.send(input).unwrap();
        }
    }
}
