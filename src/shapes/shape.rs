#![allow(dead_code)]
use raylib::drawing::RaylibDrawHandle;

pub trait DrawShape {
    fn draw(&self, draw_handle: &mut RaylibDrawHandle);
}