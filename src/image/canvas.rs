extern crate sdl2;

use std::time::Duration;
use rand::Rng;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use crate::types::hit::{Hit, World};
use crate::types::ray::Ray;
use crate::types::scene::Scene;
use crate::types::vec3::{Color, Vec3};
extern crate rayon;

use rayon::prelude::*;
pub struct Canvas {
	canvas: WindowCanvas,
	event_pump: EventPump,
	aspect_ratio: f64,
	width: u64,
	height: u64,
}

impl Canvas {
	pub fn new(title: &str, width: u32, height: u32) -> Self {
		let sdl_context = sdl2::init().unwrap();
		let video_subsystem = sdl_context.video().unwrap();
		let window = video_subsystem
			.window(title, width, height)
			.position_centered()
			.allow_highdpi()
			.build()
			.unwrap();

		let mut canvas = window.into_canvas().build().unwrap();
		let mut event_pump = sdl_context.event_pump().unwrap();
		let size = canvas.output_size().unwrap();
		let width = size.0 as u64;
		let height = size.1 as u64;
		let aspect_ratio = width as f64 / height as f64;
		canvas.clear();
		Canvas {
			canvas,
			event_pump,
			aspect_ratio,
			width,
			height
		}
	}

	pub fn draw_color(&mut self, rgb: sdl2::pixels::Color) {
		self.canvas.set_draw_color(rgb)
	}

	pub fn draw_pixel(&mut self, x: i32, y: i32, color: Option<Vec3>) {
		if let Some(color) = color {
			let r = (color.x().powf(1.0 / 1.8).clamp(0.0, 0.999) * 255.999) as u8;
			let b = (color.z().powf(1.0 / 1.8).clamp(0.0, 0.999) * 255.999) as u8;
			let g = (color.y().powf(1.0 / 1.8).clamp(0.0, 0.999) * 255.999) as u8;
			self.canvas.set_draw_color(sdl2::pixels::Color::RGB(r, g, b));

		}
		self.canvas.draw_rect(Rect::new(x, y, 1, 1)).unwrap()
	}

	pub fn render(&mut self, scene: &Scene, width: u32, height: u32, depth: u64) {
		let mut rng = rand::thread_rng();
		let camera = scene.camera().as_ref().unwrap();
		for j in 0..height
		{
		let scanline: Vec<Color> = (0..width).into_par_iter().map(|i| {
			let mut pixel_color = Color::new(0.0, 0.0, 0.0);
			for _ in 0..*scene.spp() {
				let mut rng = rand::thread_rng();
				let random_u: f64 = rng.gen();
				let random_v: f64 = rng.gen();
				let u = ((i as f64) + random_u) / ((width - 1) as f64);
				let v = ((j as f64) + random_v) / ((height - 1) as f64);

				let r = camera.get_ray(u, v);
				pixel_color += ray_color(&r, &scene.world(), depth);
			}
			pixel_color
			}).collect();
			for (i, pixel) in scanline.iter().enumerate() {
				self.draw_pixel(i as i32, (height - j) as i32, Some(*pixel / *scene.spp() as f64))
			}
		}
	}

	pub fn aspect_ratio(&self) -> f64 {
		self.aspect_ratio
	}
	pub fn event_loop(&mut self) {
		'eventloop: loop {
			for event in self.event_pump.poll_iter() {
				match event {
					Event::Quit { .. } |
					Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
						break 'eventloop;
					}
					_ => {}
				}
			}
		}
	}

	pub fn size(&self) -> (u32, u32) {
		self.canvas.output_size().unwrap()
	}
	pub fn width(&self) -> u64 {
		self.width
	}

	pub fn height(&self) -> u64 {
		self.height
	}

	pub fn show(&mut self) {
		self.canvas.present();
	}
}

fn ray_color(ray: &Ray, world: &World, depth: u64) -> Color {
	if depth == 0 {
		return Color::new(0.0, 0.0, 0.0)
	}
	if let Some(hit_rec) = world.hit(ray, 0.001, f64::INFINITY)
	{
		if let Some((attenuation, scattered)) = hit_rec.mat.scatter(ray, &hit_rec) {
			attenuation * ray_color(&scattered, world, depth -1)
		}
		else {
			Color::new(0.0, 0.0, 0.0)
		}
	}
	else
	{
		let unit_dir = ray.direction().normalized();
		let t = 0.5 * (unit_dir.y() + 1.0);
		(1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
	}
}
