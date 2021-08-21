use amethyst::{core::{Axis2, transform::Transform}, ecs::World, renderer::Camera, utils::ortho_camera::{CameraNormalizeMode, CameraOrtho, CameraOrthoWorldCoordinates}};

pub fn initialize_camera(world: &mut World, width: f32, height: f32) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, height * 1.0, 1.0);
    let mut ortho = CameraOrtho::default();
    ortho.mode = CameraNormalizeMode::Contain;
    ortho.world_coordinates = CameraOrthoWorldCoordinates {
        left: 0.0,
        top: height,
        right: width,
        bottom: 0.0,
        far: 2000.0,
        near: 1.0,
    };
    world.push((Camera::standard_2d(width, height), transform, ortho));
}
