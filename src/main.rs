mod types;
mod image;
mod objects;
mod materials;

extern crate sdl2;
extern crate rand;

use std::rc::Rc;
use rand::Rng;
use crate::image::canvas::Canvas;
use crate::types::hit::{Hit};
use crate::types::scene::Scene;

fn main() {
	let mut image = Canvas::new("Ameri Ray Tracing", 1920, 1080);
	let mut scene = Scene::new(64);
	scene.test_scene(90.0, image.aspect_ratio());
	let camera = scene.camera().as_ref().unwrap();
	let height = image.height();
	let width = image.width();

	image.render(&scene, width as u32, height as u32, 25);
	image.show();
	image.event_loop();
}
