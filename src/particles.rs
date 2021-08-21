use amethyst::{
    assets::{DefaultLoader, Handle, Loader, ProcessingQueue},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, SpriteRender, SpriteSheet, Texture},
    utils::ortho_camera::{CameraNormalizeMode, CameraOrtho, CameraOrthoWorldCoordinates},
};

const MAP_HEIGHT: f32 = 100.0;
const MAP_WIDTH: f32 = 100.0;

const PARTICLE_RADIUS: f32 = 4.0;
pub struct ParticleState;

struct Particle;

impl Particle {
    fn new() -> Particle {
        Particle {}
    }
}

impl SimpleState for ParticleState {
    fn on_start(&mut self, data: StateData<'_, GameData>) {
        let world = data.world;

        // Load the spritesheet necessary to render the graphics.
        // `spritesheet` is the layout of the sprites on the image;
        // `texture` is the pixel data.
        let sprite_sheet_handle = load_sprite_sheet(data.resources);

        initialize_particles(world, sprite_sheet_handle);
        initialize_camera(world);
    }
}

/// initialize the camera.
fn initialize_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, MAP_HEIGHT, 1.0);
    let mut ortho = CameraOrtho::default();
    ortho.mode = CameraNormalizeMode::Contain;
    ortho.world_coordinates = CameraOrthoWorldCoordinates {
        left: 0.0,
        top: MAP_HEIGHT,
        right: MAP_WIDTH,
        bottom: 0.0,
        far: 2000.0,
        near: 1.0,
    };
    world.push((Camera::standard_2d(MAP_WIDTH, MAP_HEIGHT), transform, ortho));
}

fn initialize_particles(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // Correctly position the paddles.
    let y = MAP_HEIGHT / 2.0;
    left_transform.set_translation_xyz(PARTICLE_RADIUS, y, 0.0);
    right_transform.set_translation_xyz(MAP_WIDTH - PARTICLE_RADIUS, y, 0.0);

    // Assign the sprites for the paddles
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 1); // paddle is the first sprite in the sprite_sheet

    // Create a left plank entity.
    world.push((sprite_render.clone(), Particle::new(), left_transform));

    // Create right plank entity.
    world.push((sprite_render, Particle::new(), right_transform));
}

fn load_sprite_sheet(resources: &mut Resources) -> Handle<SpriteSheet> {
    let texture: Handle<Texture> = {
        let loader = resources.get::<DefaultLoader>().unwrap();
        loader.load("texture/pong_spritesheet.png")
    };
    let loader = resources.get::<DefaultLoader>().unwrap();
    let sprites = loader.load("texture/pong_spritesheet.ron");

    loader.load_from_data(
        SpriteSheet { texture, sprites },
        (),
        &resources.get::<ProcessingQueue<SpriteSheet>>().unwrap(),
    )
}
