use mlua::UserData;
use raylib::drawing::RaylibDrawHandle;
use std::collections::VecDeque;

use crate::shapes::{DrawShape, Circle};

#[derive(Default)]
pub struct WindowSize(pub i32, pub i32);

#[derive(Default)]
pub struct Surface {
    window_size: WindowSize,
    shapes: VecDeque<Box<dyn DrawShape>>
}

impl Surface {
    pub fn new(window_size: WindowSize) -> Self {
        Self { window_size, shapes: VecDeque::new() }
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
        methods.add_method_mut("draw_circle", |_, surface, (x, y, radius)| {
            Ok(
                surface.shapes.push_back(
                    Box::new(
                        Circle::new(x, y, radius)
                    )
                )
            )
        });
    }
}
