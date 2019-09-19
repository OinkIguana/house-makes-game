use std::sync::mpsc;
use ggez::{self, event, graphics};

use crate::message::Message;
use crate::input::Input;

mod state;
use state::State;

/// A graphical front end, which provides the full, intended game experience. This is the front end
/// with which most (all?) users will play.
pub struct Graphical;

impl Graphical {
    pub fn new() -> Self { Graphical }
}

struct Renderer {
    event_loop: ggez::event::EventsLoop,
    ctx: ggez::Context,
    inputs: mpsc::Sender<Input>,
    messages: mpsc::Receiver<Message>,
}

impl super::FrontEnd for Graphical {
    fn split(self) -> (Box<dyn super::Renderer>, super::Channel) {
        let (ctx, event_loop) = ggez::ContextBuilder::new("House Makes Game", "House").build().unwrap();
        let (inputs, tx) = mpsc::channel();
        let (sx, messages) = mpsc::channel();
        (Box::new(Renderer { event_loop, ctx, inputs, messages }), (Box::new(sx), Box::new(tx)))
    }
}

impl super::Renderer for Renderer {
    fn game_loop(mut self: Box<Self>) {
        let mut state = State::new(self.inputs, self.messages);
        event::run(&mut self.ctx, &mut self.event_loop, &mut state).unwrap();
    }
}
