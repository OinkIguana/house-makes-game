#[macro_use] extern crate text_io;

mod game;
mod input;

use game::Game;
use input::{Input, Command};

#[derive(Debug, structopt::StructOpt)]
struct Opts {
    #[structopt(long)]
    no_gfx: bool,
}

#[paw::main]
fn main(args: Opts) {
    let game: Box<dyn Game> = if args.no_gfx {
        Box::new(game::TextGame::new())
    } else {
        Box::new(game::RegularGame::new())
    };

    loop {
        let line = game.read_line();
        let input = Input::parse(&line);

        eprintln!("{:?}", input);

        match input {
            Input::Command(Command::Quit) => break,
            _ => {},
        }
    }
}
