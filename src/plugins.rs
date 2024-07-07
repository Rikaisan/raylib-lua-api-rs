#![allow(dead_code)]
use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};
use crate::error::PluginError;
use mlua::prelude::*;


#[derive(Debug, Default)]
pub struct Plugin {
    pub name: String,
    pub state: Lua
}

#[derive(Debug, Default)]
pub struct PluginManager {
    plugins: HashMap<String, Plugin>
}

impl PluginManager {
    pub fn new(plugin_dir: impl Into<PathBuf>) -> Result<Self, PluginError> {
        let mut manager = Self {
            ..Default::default()
        };

        let dir = std::fs::read_dir(plugin_dir.into())?;
        let files = dir.into_iter().filter( |entry| entry.as_ref().unwrap().file_type().unwrap().is_file() && entry.as_ref().unwrap().file_name().to_str().unwrap().ends_with(".lua") );
        for entry in files {
            if let Ok(mut file) = File::open(entry.as_ref().unwrap().path()) {
                let mut file_buffer = String::new();
                file.read_to_string(&mut file_buffer)?;

                let lua = Lua::new();
                lua.load(file_buffer).exec()?;

                let file_name = entry.as_ref().unwrap().file_name().into_string().unwrap();
                let name = lua.globals().get::<_, String>("PLUGIN_NAME").map_err(|_| {
                    log::warn!("Plugin {} doesn't define PLUGIN_NAME, skipping...", &file_name);
                    PluginError::PluginNameMissing(file_name.clone())
                })?;

                if manager.plugins.contains_key(&name) {
                    log::warn!("Plugin {} defines PLUGIN_NAME={} which already exists, skipping...", file_name, name);
                    return Err(PluginError::PluginExists(name))
                }
                manager.plugins.insert(name.clone(), Plugin { name: name.clone(), state: lua });
                log::info!("Plugin {}[{}] loaded successfully!", name, file_name);
            }
        }
        Ok(manager)
    }

    pub fn tick(&self) -> Result<(), PluginError> {
        for plugin in self.plugins.values() {
            if let Ok(update) = plugin.state.globals().get::<_, LuaFunction>("Update") {
                let result = update.call(());
                if result.is_err() { log::error!("Error while calling Update on {}", plugin.name) }
                result?
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