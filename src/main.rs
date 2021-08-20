use amethyst::{
    assets::LoaderBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        rendy::hal::command::ClearColor,
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

pub struct Pong;

impl SimpleState for Pong {}

fn main() -> amethyst::Result<()> {
    println!("Hello, world!");
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let mut dispatcher = DispatcherBuilder::default();
    dispatcher.add_bundle(LoaderBundle).add_bundle(
        RenderingBundle::<DefaultBackend>::new()
        .with_plugin(RenderToWindow::from_config_path(display_config_path)?.with_clear(ClearColor{
            float32: [0.0, 0.0, 0.0, 1.0],
        }))
        .with_plugin(RenderFlat2D::default()),
    );
    let assets_dir = app_root.join("assets");
    let game = Application::new(assets_dir, Pong, dispatcher)?;

    game.run();

    Ok(())
}
