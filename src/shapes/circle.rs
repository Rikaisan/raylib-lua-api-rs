use raylib::prelude::*;
use super::DrawShape;


#[derive(Debug, Default)]
pub struct Circle {
    x: i32,
    y: i32,
    radius: f32
}

impl Circle {
    pub fn new(x: i32, y: i32, radius: f32) -> Self {
        Self { x, y, radius }
    }
}

impl DrawShape for Circle {
    fn draw(&self, draw_handle: &mut raylib::prelude::RaylibDrawHandle) {
        draw_handle.draw_circle(self.x, self.y, self.radius, Color::BLUE)
    }
}