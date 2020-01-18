use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

// Initial game state.
// The tutorial names this the same as the game name,
// but I don't like that since working titles are common.
pub struct BaseState;

// Simpletate is a simplified State trait
// SimpleState: https://docs-src.amethyst.rs/stable/amethyst/prelude/trait.SimpleState.html
// State: https://docs-src.amethyst.rs/stable/amethyst/prelude/trait.State.html
// "State" in the book: https://book.amethyst.rs/stable/concepts/state.html
//
// To clarify, State uses generics for all its implementation, whereas
// SimpleState fills in these generics with what you would commonly use.
impl SimpleState for BaseState {}

fn main() -> amethyst::Result<()> {
    // Set up a logger for game testing/debugging
    // This spits out all your logging messages into the terminal.
    amethyst::start_logger(Default::default());

    // Set up the game window
    // By setting it up to load the config from a file, you don't have to
    // recompile every time you change the graphics settings.
    // see: config/display.ron
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    // Basic application setup
    let game_data = GameDataBuilder::default();

    let assets_dir = app_root.join("assets");
    let mut world = World::new();
    let mut game = Application::new(assets_dir, BaseState, game_data)?;

    // This starts the game loop. The game will continue until a state returns
    // Trans::Quit or all states are popped off the stack.
    game.run();

    Ok(())
}
