use std::{fs::File, io::Read, path::PathBuf};

use mlua::prelude::*;

#[derive(Debug, Default)]
struct Plugin {
    name: String,
    state: Lua
}

#[derive(Debug, Default)]
struct Manager {
    plugins: Vec<Plugin>
}

impl Manager {
    fn new(plugin_dir: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let mut manager = Self {
            ..Default::default()
        };

        match std::fs::read_dir(plugin_dir.into()) {
            Ok(dir) => {
                for entry in dir.into_iter().filter( |entry| entry.as_ref().unwrap().file_type().unwrap().is_dir() ) {
                if let Ok(mut file) = File::open(entry.unwrap().path()) {
                    let mut file_buffer = String::new();
                    file.read_to_string(&mut file_buffer)?;
                    let lua = Lua::new();
                    lua.load(file_buffer).exec()?;
                    let name = lua.globals().get::<_, String>("PLUGIN_NAME").unwrap_or("No Name".into());
                    manager.plugins.push(Plugin {
                        name,
                        state: lua
                    });
                }
            }
        },
            Err(_) => { println!("Plugin dir doesn't exist!") },
        }

        // manager.plugins.push();

        Ok(manager)
    }
}