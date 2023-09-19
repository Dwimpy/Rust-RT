mod types;
mod image;

extern crate sdl2;

use crate::image::canvas::Canvas;
use crate::types::vec3::{Color, Point3, Vec3};

fn main() {

    let mut image = Canvas::new("Ameri Ray Tracing", 1920, 1080);
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
