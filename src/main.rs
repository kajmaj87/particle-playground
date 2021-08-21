use amethyst::{assets::LoaderBundle, core::transform::TransformBundle, prelude::*, renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        rendy::hal::command::ClearColor,
        types::DefaultBackend,
        RenderingBundle,
    }, utils::{application_root_dir, ortho_camera::build_camera_normalize_system}};

mod particles;

use crate::particles::ParticleState;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let mut dispatcher = DispatcherBuilder::default();
    dispatcher
        .add_bundle(LoaderBundle)
        .add_bundle(TransformBundle)
        .add_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?.with_clear(ClearColor {
                        float32: [0.0, 0.0, 0.0, 1.0],
                    }),
                )
                .with_plugin(RenderFlat2D::default()),
        )
        .add_system(|| { build_camera_normalize_system() });
    let assets_dir = app_root.join("assets");
    let game = Application::new(assets_dir, ParticleState, dispatcher)?;

    game.run();

    Ok(())
}
