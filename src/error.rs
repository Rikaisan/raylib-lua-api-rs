use thiserror::Error;
use std::io;


#[derive(Error, Debug)]
pub enum PluginError {
    #[error("'PLUGIN_NAME' global variable doesn't exist on '{0}'!")]
    PluginNameMissing(String),
    #[error("plugin with the name '{0}' already exists!")]
    PluginExists(String),
    #[error(transparent)]
    IOError(#[from] io::Error),
    #[error(transparent)]
    LuaError(#[from] mlua::Error)
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    PluginError(#[from] PluginError)
}