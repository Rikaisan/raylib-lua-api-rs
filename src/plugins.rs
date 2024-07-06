use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};

use mlua::prelude::*;

use crate::error::PluginError;


#[derive(Debug, Default)]
pub struct Plugin {
    pub name: String,
    pub state: Lua
}

#[derive(Debug, Default)]
pub struct PluginManager {
    app_name: String,
    plugins: HashMap<String, Plugin>
}

impl PluginManager {
    pub fn new(plugin_dir: impl Into<PathBuf>, app_name: impl Into<String>) -> Result<Self, PluginError> {
        let mut manager = Self {
            app_name: app_name.into(),
            ..Default::default()
        };

        let dir = std::fs::read_dir(plugin_dir.into())?;
        let files = dir.into_iter().filter( |entry| entry.as_ref().unwrap().file_type().unwrap().is_file() && entry.as_ref().unwrap().file_name().to_str().unwrap().ends_with(".lua") );
        for entry in files {
            if let Ok(mut file) = File::open(entry.as_ref().unwrap().path()) {
                let mut file_buffer = String::new();
                file.read_to_string(&mut file_buffer)?;

                let lua = Lua::new();
                lua.globals().set("APP_NAME", manager.app_name.as_str())?;
                lua.load(file_buffer).exec()?;

                let name = lua.globals().get::<_, String>("PLUGIN_NAME").map_err(|_|PluginError::PluginNameMissing(entry.as_ref().unwrap().file_name().into_string().unwrap()))?;
                if manager.plugins.contains_key(&name) { return Err(PluginError::PluginExists(name)) }
                manager.plugins.insert(name.clone(), Plugin { name, state: lua });
            }
        }
        Ok(manager)
    }

    pub fn tick(&self) -> Result<(), PluginError> {
        for plugin in self.plugins.values() {
            if let Ok(update) = plugin.state.globals().get::<_, LuaFunction>("Update") {
                update.call(())?
            }
        }
        Ok(())
    }

    pub fn get_plugin(&self, name: impl Into<String>) -> Option<&Plugin> {
        self.plugins.get(&name.into())
    }

    pub fn get_plugins(&self) -> Vec<&Plugin> {
        self.plugins.values().collect()
    }
}