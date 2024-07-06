use raylib::prelude::*;
use super::DrawShape;


#[derive(Debug, Default)]
pub struct Rectangle {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    color: Color
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: i32, height: i32, color: Color) -> Self {
        Self { x, y, width, height, color }
    }
}

impl DrawShape for Rectangle {
    fn draw(&self, draw_handle: &mut raylib::prelude::RaylibDrawHandle) {
        draw_handle.draw_rectangle(self.x, self.y, self.width, self.height, self.color)
    }
}