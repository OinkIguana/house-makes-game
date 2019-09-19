//! This is the entry point for House Makes Game.

// This crate is not compatible with new macro system, so it must be imported the old way
#[macro_use] extern crate text_io;

use std::thread::spawn;

mod front_end;
mod input;
mod message;

use front_end::FrontEnd;
use input::{Input, Command};
use message::Message;

/// House Makes Game. Play by running this command.
#[derive(Debug, structopt::StructOpt)]
struct Opts {
    /// Runs the game without opening a graphical window, allowing it to be played (in a basic
    /// form) on a terminal
    #[structopt(long)]
    no_gfx: bool,
}

#[paw::main]
fn main(args: Opts) {
    let (renderer, channel) = if args.no_gfx {
        front_end::Text::new().split()
    } else {
        front_end::Graphical::new().split()
    };

    let handle = spawn(move || {
        let (sx, tx) = channel;
        loop {
            let input = match tx.recv() {
                Ok(input) => input,
                Err(()) => break,
            };
            match input {
                Input::Command(Command::Quit) => sx.send(Message::Quit).unwrap(),
                _ => (),
            }
        }
    });

    renderer.game_loop();

    handle.join().unwrap();
}
