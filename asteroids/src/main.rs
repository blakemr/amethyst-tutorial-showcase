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
    // also sets up root directory path, which will be used to reference
    // all the other files.
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    // Basic application setup
    let game_data = GameDataBuilder::default().with_bundle(
        RenderingBundle::<DefaultBackend>::new()
            // The RenderToWindow plugin sets up everything you need to open a window and draw on it.
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)
                    .with_clear([0.0, 0.0, 0.0, 1.0]),
            )
            // RenderFlat2D plugin is used to render entities with a 'SpriteRender' component.
            .with_plugin(RenderFlat2D::default()),
    )?;

    let assets_dir = app_root.join("assets");
    let mut world = World::new();

    let mut game = Application::new(assets_dir, BaseState, game_data)?;

    // This starts the game loop. The game will continue until a state returns
    // Trans::Quit or all states are popped off the stack.
    game.run();

    Ok(())
}
