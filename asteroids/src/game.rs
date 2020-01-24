// Once again, Naming the file something generic so that changing the game title is easier.
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

// Constants
// VIEW_WIDTH/VIEW_HEIGHT: Amount of the screen to show. Not the window size.
pub const VIEW_WIDTH: f32 = 100.0;
pub const VIEW_HEIGHT: f32 = 100.0;
// SHIP_WIDTH/SHIP_HEIGHT: Size of the ship sprite
pub const SHIP_WIDTH: f32 = 32.0;
pub const SHIP_HEIGHT: f32 = 32.0;

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
impl SimpleState for BaseState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Ship>();

        init_camera(world);
        init_ship(world);
    }
}

// Create the camera
fn init_camera(world: &mut World) {
    // Make the view scale to the window size.
    let mut transform = Transform::default();

    // this positions the camera so that 0,0 is the bottom left
    transform.set_translation_xyz(VIEW_WIDTH * 0.5, VIEW_HEIGHT * 0.5, 1.0);

    // Build the camera
    world
        .create_entity()
        .with(Camera::standard_2d(VIEW_WIDTH, VIEW_HEIGHT))
        .with(transform)
        .build();
}

// Create the ship
pub struct Ship {
    pub width: f32,
    pub height: f32,
    pub lives: i32,
}

impl Ship {
    fn new(lives: i32) -> Ship {
        Ship {
            width: SHIP_WIDTH,
            height: SHIP_HEIGHT,
            lives,
        }
    }
}

impl Component for Ship {
    type Storage = DenseVecStorage<Self>;
}

// Makes a new ship with 3 lives
fn init_ship(world: &mut World) {
    let mut ship_transform = Transform::default();

    // Position the ship in the center of the screen
    let x = VIEW_WIDTH / 2.0;
    let y = VIEW_HEIGHT / 2.0;
    ship_transform.set_translation_xyz(x, y, 0.0);

    // Build the ship
    let lives: i32 = 3;
    world
        .create_entity()
        .with(Ship::new(lives))
        .with(ship_transform)
        .build();
}
