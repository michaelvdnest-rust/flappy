#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(missing_docs)]

//!Flappy Dragon - a flappy bird clone

/// Use everything from bracket-lib
use bracket_lib::prelude::*;

/// The game's current state. Everything you need to preserve between frames is in your game’s state.
/// The state represents a snapshot of the current game.
struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello, Bracket Terminal!");
    }
}

/// This function runs the game.
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State {})
}
