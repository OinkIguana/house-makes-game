//! Graphical front end state management and event handling

use std::sync::mpsc;

use ggez::nalgebra as na;
use ggez::{self, graphics};

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

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, graphics::WHITE);

        let (w, h) = graphics::size(ctx);
        let margin = 40.0;

        let circle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            [0.0, 0.0, w / 3.0, h].into(),
            graphics::BLACK,
        )?;
        graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;

        let narrative = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            [0.0, 0.0, 2.0 * w / 3.0, 4.0 * h / 6.0].into(),
            graphics::Color::new(0.0, 1.0, 0.0, 1.0),
        )?;
        graphics::draw(ctx, &narrative, (na::Point2::new(w / 3.0, 0.0),))?;

        let input = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            [0.0, 0.0, 2.0 * w / 3.0 - margin, h / 6.0 - margin].into(),
            graphics::Color::new(1.0, 0.0, 1.0, 1.0),
        )?;
        graphics::draw(ctx, &input, (na::Point2::new(w / 3.0 + margin / 2.0, 4.0 * h / 6.0 + margin / 2.0),))?;

        let mut w_btn = w / 3.0;

        let dictionary = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            [0.0, 0.0, (2.0 * w / 3.0) / 4.0  - margin, h / 6.0 - margin].into(),
            graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        )?;
        graphics::draw(ctx, &dictionary, (na::Point2::new(w_btn + margin / 2.0, 5.0 * h / 6.0),))?;

        w_btn += (2.0 * w / 3.0) / 4.0;

        let inventory = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            [0.0, 0.0, (2.0 * w / 3.0) / 4.0  - margin, h / 6.0 - margin].into(),
            graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        )?;
        graphics::draw(ctx, &inventory, (na::Point2::new(w_btn + margin / 2.0, 5.0 * h / 6.0),))?;

        w_btn += (2.0 * w / 3.0) / 4.0;

        let map = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            [0.0, 0.0, (2.0 * w / 3.0) / 4.0  - margin, h / 6.0 - margin].into(),
            graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        )?;
        graphics::draw(ctx, &map, (na::Point2::new(w_btn + margin / 2.0, 5.0 * h / 6.0),))?;

        w_btn += (2.0 * w / 3.0) / 4.0;

        let log = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            [0.0, 0.0, (2.0 * w / 3.0) / 4.0  - margin, h / 6.0 - margin].into(),
            graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        )?;
        graphics::draw(ctx, &log, (na::Point2::new(w_btn + margin / 2.0, 5.0 * h / 6.0),))?;

        // must be last to be drawn (bottom up aka painter's algorithm)
        let input_prompt = graphics::Text::new(graphics::TextFragment::new(self.input_buffer.as_str()).color(graphics::WHITE));
        graphics::draw(ctx, &input_prompt, graphics::DrawParam::new().dest([32., 32.]))?;

        graphics::present(ctx)
    }

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
