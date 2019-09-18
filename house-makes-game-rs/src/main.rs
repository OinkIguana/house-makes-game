//! This is the entry point for House Makes Game.

// This crate is not compatible with new macro system, so it must be imported the old way
#[macro_use] extern crate text_io;

mod front_end;
mod input;
mod message;

use front_end::FrontEnd;
use input::{Input, Command};

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

    // TODO: spawn a thread here and start processing inputs

    renderer.game_loop();
}
