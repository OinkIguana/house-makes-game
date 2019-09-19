use std::sync::mpsc;
use std::thread::spawn;

use crate::message::Message;
use crate::input::Input;

/// A text based front end for the game, which operates on stdin/stdout. This is mostly useful in
/// development to allow writing scripted tests, for working on the game through SSH, or for
/// testing and prototyping things quickly without having to deal with a UI.
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
        // two channels are created, one to send messages in
        let (sender, messages) = mpsc::channel();
        // and one to send inputs out
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
        // and another thread that passes Messages received from the game engine to the internal event queue
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
                // input events get sent to the outside game engine
                Internal::Input(input) => self.inputs.send(input).unwrap(),

                // message events (received from outside) are processed here

                // when the `\quit` message is received, exit. The channels will be dropped and
                // then the other two threads above will terminate on their own eventually!
                Internal::Message(Message::Quit) => break,
            }
        }
    }
}
