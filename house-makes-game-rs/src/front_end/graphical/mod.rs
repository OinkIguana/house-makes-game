use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::{WindowBuilder, Window},
};

use crate::message::Message;
use crate::input::Input;

pub struct Graphical;

impl Graphical {
    pub fn new() -> Self { Graphical }
}

struct Renderer {
    event_loop: EventLoop<Message>,
    window: Window,
}

struct Sender {
    proxy: EventLoopProxy<Message>,
}

struct Receiver;

impl super::FrontEnd for Graphical {
    fn split(self) -> (Box<dyn super::Renderer>, super::Channel) {
        let event_loop = EventLoop::with_user_event();
        let window = WindowBuilder::new()
            .with_maximized(true)
            .with_visible(true)
            .with_title("House Makes Game")
            .build(&event_loop)
            .expect("Could not create window");
        let proxy = event_loop.create_proxy();

        (Box::new(Renderer { event_loop, window }), (Box::new(Sender { proxy }), Box::new(Receiver)))
    }
}

impl super::Renderer for Renderer {
    fn game_loop(&self) { unimplemented!() }
}

impl super::channel::Sender<Message> for Sender {
    fn send(&self, msg: Message) -> Result<(), ()> {
        self.proxy.send_event(msg)
            .map_err(|_| ())
    }
}

impl super::channel::Receiver<Input> for Receiver {
    fn recv(&self) -> Result<Input, ()> {
        unimplemented!()
    }
}
