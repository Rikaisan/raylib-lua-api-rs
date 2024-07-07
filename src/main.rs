mod plugins;
mod surface;
mod shapes;
mod error;

use surface::{DrawSurface, Surface, WindowSize};
use plugins::PluginManager;
use raylib::prelude::*;

fn main() -> Result<(), error::AppError> {
    if std::env::var("RUST_LOG").is_err() { std::env::set_var("RUST_LOG", "info") }
    env_logger::init();

    let window_size = WindowSize(640, 480);

    let (mut rl, thread) = raylib::init()
        .size(window_size.0, window_size.1)
        .title("Hello, World!")
        .build();

    rl.set_target_fps(60);

    let plugin_manager = PluginManager::new("scripts").expect("Error loading scripts");
    let mut surface: Surface = Surface::new(window_size);

    let title = "Hey! :)";
    let text_size = rl.get_font_default().measure_text(title, 20.0, 2.0);

    while !rl.window_should_close() {
        plugin_manager.tick()?;
        if let Err(e) = plugin_manager.draw(&mut surface) { println!("Error populating frame: {e}") }
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text(
            title,
            (320.0 - text_size.x / 2.0) as i32,
            (240.0 - text_size.y / 2.0) as i32,
            20,
            Color::GRAY
        );

        surface.draw(&mut d);
    }

    Ok(())
}