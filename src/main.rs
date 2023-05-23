#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(missing_docs)]

//!Flappy Dragon - a flappy bird clone

/// Use e/verything from bracket-lib
use bracket_lib::prelude::*;

/// The game's current state. Everything you need to preserve between frames is in your game’s state.
/// The state represents a snapshot of the current game.
struct State {
    mode: GameMode,
}

/// The game state.
impl State {
    /// Initliaze the game state.
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
        }
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn restart(&mut self) {
        // TODO fill in later
        self.mode = GameMode::Playing;
    }

    fn play(&mut self, ctx: &mut BTerm) {
        //TODO fill in later
        self.mode = GameMode::End;
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }

        //TOD fill in later
    }
}

/// Let the engine know what to call on each tick.
impl GameState for State {
    /// Called at every tick. The bridge between the current Game State and the Game Engine.
    ///
    /// # Arguments
    ///
    /// * `self` Allows the `tick` function to change your `State` instance.
    /// * `ctx`  A window into the currently running `bracket-terminal`.
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

// The game mode specifying what the game should do on the current tick.
enum GameMode {
    /// The player is waiting at the main menu.
    Menu,
    /// Game play is in progress.
    Playing,
    /// The game is over.
    End,
}

/// This function runs the game.
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}
