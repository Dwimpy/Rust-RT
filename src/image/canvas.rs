extern crate sdl2;

use std::time::Duration;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use crate::types::vec3::{Color, Vec3};

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