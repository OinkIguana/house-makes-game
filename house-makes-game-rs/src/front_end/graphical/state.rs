use std::sync::mpsc;

use ggez;

use crate::input::Input;
use crate::message::Message;

pub struct State {
    inputs: mpsc::Sender<Input>,
    messages: mpsc::Receiver<Message>,
    input_buffer: String,
}

impl State {
    pub fn new(inputs: mpsc::Sender<Input>, messages: mpsc::Receiver<Message>) -> Self {
        State { inputs, messages, input_buffer: String::new() }
    }
}

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

impl State {
    fn message_received_event(&mut self, ctx: &mut ggez::Context, message: Message) -> ggez::GameResult {
        match message {
            Message::Quit => ggez::event::quit(ctx),
        }

        Ok(())
    }
}
