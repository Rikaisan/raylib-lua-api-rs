use crate::{error::PluginError, plugins::PluginManager, shapes::*};
use raylib::{color::Color, drawing::RaylibDrawHandle};
use std::collections::VecDeque;
use mlua::UserData;


#[derive(Default)]
pub struct WindowSize(pub i32, pub i32);

#[derive(Default)]
pub struct Surface {
    window_size: WindowSize,
    color_map: ColorMap,
    shapes: VecDeque<Box<dyn DrawShape>>
}

impl Surface {
    pub fn new(window_size: WindowSize) -> Self {
        Self { window_size, color_map: ColorMap::new(), shapes: VecDeque::new() }
    }

    pub fn draw(&mut self, draw_handle: &mut RaylibDrawHandle) {
        while let Some(shape) = self.shapes.pop_front() {
            shape.draw(draw_handle);
        }
    }
}


impl UserData for Surface {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("size", |lua, surface| {
            let tbl = lua.create_table()?;
            tbl.set("x", surface.window_size.0)?;
            tbl.set("y", surface.window_size.1)?;
            Ok(tbl)
        });
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("draw_circle", |_, surface, (x, y, radius, color)| {
            let color: String = color;
            Ok(
                surface.shapes.push_back(
                    Box::new(
                        Circle::new(x, y, radius, surface.color_map.from(color).unwrap_or(Color::BLACK))
                    )
                )
            )
        });

        methods.add_method_mut("draw_rectangle", |_, surface, (x, y, width, height, color)| {
            let color: String = color;
            Ok(
                surface.shapes.push_back(
                    Box::new(
                        Rectangle::new(x, y, width, height, surface.color_map.from(color).unwrap_or(Color::BLACK))
                    )
                )
            )
        });
    }
}

pub trait DrawSurface<E> {
    fn draw(&self, surface: &mut Surface) -> Result<(), E>;
}

impl DrawSurface<PluginError> for PluginManager {
    fn draw(&self, surface: &mut Surface) -> Result<(), PluginError> {
        for plugin in self.get_plugins().iter() {
            plugin.state.scope(|scope| {
                let surface = scope.create_userdata_ref_mut(surface)?;
                if let Ok(draw) = plugin.state.globals().get::<_, mlua::Function>("Draw") {
                    draw.call(surface)?
                }
                Ok(())
            })?
        }
        Ok(())
    }
}