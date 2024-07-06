use mlua::UserData;
use raylib::drawing::RaylibDrawHandle;
use std::collections::VecDeque;

use super::{Circle, DrawShape};

#[derive(Default)]
pub struct Screen {
    window_width: i32,
    window_height: i32,
    shapes: VecDeque<Box<dyn DrawShape>>
}

impl Screen {
    pub fn new(window_width: i32, window_height: i32) -> Self {
        Self { window_width, window_height, shapes: VecDeque::new() }
    }

    pub fn draw(&mut self, draw_handle: &mut RaylibDrawHandle) {
        while let Some(shape) = self.shapes.pop_front() {
            shape.draw(draw_handle);
        }
    }
}

impl UserData for Screen {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("size", |lua, screen| {
            let tbl = lua.create_table()?;
            tbl.set("x", screen.window_width)?;
            tbl.set("y", screen.window_height)?;
            Ok(tbl)
        });
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("draw_circle", |_, screen, (x, y, radius)| {
            Ok(
                screen.shapes.push_back(
                    Box::new(
                        Circle::new(x, y, radius)
                    )
                )
            )
        });
    }
}
