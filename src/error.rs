use thiserror::Error;
use std::io;


#[derive(Error, Debug)]
pub enum PluginError {
    #[error("'PLUGIN_NAME' global variable doesn't exist on '{0}'!")]
    PluginNameMissing(String),
    #[error("plugin with the name '{0}' already exists!")]
    PluginExists(String),
    #[error("error loading plugins")]
    IOError(#[from] io::Error),
    #[error("error executing lua file")]
    LuaError(#[from] mlua::Error)
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("a plugin error ocurred")]
    PluginError(#[from] PluginError)
}