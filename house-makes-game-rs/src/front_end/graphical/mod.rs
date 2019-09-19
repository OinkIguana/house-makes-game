use std::sync::mpsc;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::{WindowBuilder, Window, Fullscreen},
};

use crate::message::Message;
use crate::input::Input;

/// A graphical front end, which provides the full, intended game experience. This is the front end
/// with which most (all?) users will play.
pub struct Graphical;

impl Graphical {
    pub fn new() -> Self { Graphical }
}

struct Renderer {
    event_loop: EventLoop<Message>,
    inputs: mpsc::Sender<Input>,
    _window: Window,
}

#[derive(Clone)]
struct Sender {
    proxy: EventLoopProxy<Message>,
}

impl super::FrontEnd for Graphical {
    fn split(self) -> (Box<dyn super::Renderer>, super::Channel) {
        let event_loop = EventLoop::with_user_event();
        let monitor = event_loop.primary_monitor();
        let window = WindowBuilder::new()
            .with_maximized(true)
            .with_fullscreen(Some(Fullscreen::Borderless(monitor)))
            .with_visible(true)
            .with_title("House Makes Game")
            .build(&event_loop)
            .expect("Could not create window");
        let proxy = event_loop.create_proxy();
        let (sx, tx) = mpsc::channel();

        (Box::new(Renderer { event_loop, inputs: sx, _window: window }), (Box::new(Sender { proxy }), Box::new(tx)))
    }
}

impl super::Renderer for Renderer {
    fn game_loop(self: Box<Self>) {
        let mut input_buffer = String::new();
        let Renderer { event_loop, inputs, .. } = *self;
        event_loop.run(move |event, _window_target, control_flow| {
            match event {
                // quit when they click the close button or enter the `\quit` command in game)
                | Event::UserEvent(Message::Quit)
                | Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => *control_flow = ControlFlow::Exit,

                // upon receiving \r (carriage return/enter key), process the input and send it out
                // to the game engine
                Event::WindowEvent { event: WindowEvent::ReceivedCharacter('\r'), .. } => {
                    let input = Input::parse(&input_buffer);
                    input_buffer.clear();
                    if inputs.send(input).is_err() {
                        *control_flow = ControlFlow::Exit;
                    }
                }
                // receive letters and add them to the buffer
                Event::WindowEvent { event: WindowEvent::ReceivedCharacter(ch), .. } => input_buffer.push(ch),

                // draw the screen when redraw is requested (I guess this is what VSync is?)
                Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => { /* TODO: render visible UI */ }
                _ => (),
            }
        });
    }
}

impl super::channel::Sender<Message> for Sender {
    fn send(&self, msg: Message) -> Result<(), ()> {
        self.proxy.send_event(msg)
            .map_err(|_| ())
    }
}
