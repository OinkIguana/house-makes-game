use std::sync::mpsc;
use std::thread::spawn;

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

enum Internal {
    Input(Input),
    Message(Message),
}

impl super::Renderer for Renderer {
    fn game_loop(self: Box<Self>) {
        let (sx, tx) = mpsc::channel();
        let send_input = sx.clone();
        // spawn one thread for reading user input
        spawn(move || {
            loop {
                let line: String = read!("{}\n");
                let input = Input::parse(&line);
                if send_input.send(Internal::Input(input)).is_err() {
                    break;
                }
            }
        });
        // another thread that passes messages from self.messages to internal event queue
        let messages = self.messages;
        spawn(move || {
            for msg in messages.iter() {
                if sx.send(Internal::Message(msg)).is_err() {
                    break;
                }
            }
        });

        // then process all the received messages here
        for internal in tx.iter() {
            match internal {
                Internal::Input(input) => self.inputs.send(input).unwrap(),
                Internal::Message(Message::Quit) => break,
            }
        }
    }
}
