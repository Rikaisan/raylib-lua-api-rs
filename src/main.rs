mod shapes;
mod surface;

use std::{fs::File, io::Read, path::PathBuf};
use raylib::prelude::*;
use mlua::prelude::*;
use surface::{Surface, WindowSize};

fn get_lua_state(file_path: impl Into<PathBuf>, app_name: &str) -> Result<Lua, LuaError> {
    let mut file_content = String::new();
    File::open(file_path.into())?.read_to_string(&mut file_content)?;

    let lua = Lua::new();
    lua.globals().set("APP_NAME", app_name)?;
    lua.load(&file_content).exec()?;
    Ok(lua)
}

fn get_title(lua: &Lua) -> Result<String, LuaError> {
    lua.globals().get::<_, String>("TITLE")
}

fn populate_external_surface(lua: &Lua, surface: &mut Surface) -> mlua::Result<()> {
    lua.scope(|scope| {
        let surface = scope.create_userdata_ref_mut(surface)?;
        lua.globals().get::<_, LuaFunction>("Draw")?.call(surface)
    })
}

fn main() {
    let window_size = WindowSize(640, 480);

    let (mut rl, thread) = raylib::init()
        .size(window_size.0, window_size.1)
        .title("Hello, World!")
        .build();

    let test_file = get_lua_state("scripts/options.lua", "Raylib Lua API").unwrap_or(Lua::new());

    let title = get_title(&test_file).unwrap_or("LuaShapes".into());
    let text_size = rl.get_font_default().measure_text(title.as_str(), 20.0, 2.0);

    let mut surface: Surface = Surface::new(window_size);

    while !rl.window_should_close() {
        if let Err(e) = populate_external_surface(&test_file, &mut surface) { println!("Error populating frame: {e}") }
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        // d.draw_line(0, 240, 640, 240, Color::RED); // H
        // d.draw_line(320, 0, 320, 480, Color::RED); // V
        d.draw_text(
            title.as_str(),
            (320.0 - text_size.x / 2.0) as i32,
            (240.0 - text_size.y / 2.0) as i32,
            20,
            Color::GRAY
        );

        surface.draw(&mut d);
    }
}