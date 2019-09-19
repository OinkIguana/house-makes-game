//! Graphical front end state management and event handling

use std::sync::mpsc;

use ggez;

use crate::input::Input;
use crate::message::Message;

/// The current state of the UI. Very basic right now, but can be updated for everything we need.
pub struct State {
    /// channel to send inputs to the game engine
    inputs: mpsc::Sender<Input>,
    /// channel to receive messages from the game engine
    messages: mpsc::Receiver<Message>,
    /// what the user has currently typed in, but not yet sent
    input_buffer: String,
}

impl State {
    pub fn new(inputs: mpsc::Sender<Input>, messages: mpsc::Receiver<Message>) -> Self {
        State { inputs, messages, input_buffer: String::new() }
    }
}

// handles all the events of the game. See the [ggez documentation](https://ggez.rs/) for more info.
impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        loop {
            let message = match self.messages.try_recv() {
                Ok(msg) => msg,
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    ggez::event::quit(ctx); // game is over and somehow we missed it, so let's close up
                    break;
                }
            };

            self.message_received_event(ctx, message)?;
        }

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult { Ok(()) }

    fn text_input_event(&mut self, _ctx: &mut ggez::Context, ch: char) {
        match ch {
            '\r' => {
                let input = Input::parse(&self.input_buffer);
                self.input_buffer.clear();
                self.inputs.send(input).unwrap();
            },
            _ => self.input_buffer.push(ch),
        }
    }
}

// our own event handler for handling messages from the game engine
impl State {
    fn message_received_event(&mut self, ctx: &mut ggez::Context, message: Message) -> ggez::GameResult {
        match message {
            // Quit the game by calling quit
            Message::Quit => ggez::event::quit(ctx),
        }

        Ok(())
    }
}
