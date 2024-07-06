use raylib::prelude::*;
use super::DrawShape;


#[derive(Debug, Default)]
pub struct Circle {
    x: i32,
    y: i32,
    radius: f32,
    color: Color
}

impl Circle {
    pub fn new(x: i32, y: i32, radius: f32, color: Color) -> Self {
        Self { x, y, radius, color }
    }
}

impl DrawShape for Circle {
    fn draw(&self, draw_handle: &mut raylib::prelude::RaylibDrawHandle) {
        draw_handle.draw_circle(self.x, self.y, self.radius, self.color)
    }
}