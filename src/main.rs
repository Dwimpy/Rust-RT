mod types;
mod image;

extern crate sdl2;

use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use crate::image::canvas::Canvas;
use crate::types::ray::Ray;
use crate::types::vec3::{Color, Point3, Vec3};

fn main() {

    let mut image = Canvas::new("Ameri Ray Tracing", 800, 600);
    let aspect_ratio = (image.width() / image.height()) as f32;
    for j in 0..image.height(){
        for i in 0..image.width() {
            let pixel_color = Color::new(i as f64 / ((image.width() - 1) as f64),
                                         j as f64 / ((image.height() - 1) as f64),
                                         0.25);
            image.draw_pixel(i as i32, (image.height() - j) as i32, Some(pixel_color));
        }
    }
    image.show();
    image.event_loop();

}
