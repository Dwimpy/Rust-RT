extern crate sdl2;

use std::time::Duration;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use crate::types::vec3::{Color, Vec3};

pub struct Canvas {
	canvas: WindowCanvas,
	event_pump: EventPump,
	width: u32,
	height: u32,
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
		Canvas {
			canvas,
			event_pump,
			width,
			height,
		}
	}

	pub fn draw_color(&mut self, rgb: sdl2::pixels::Color) {
		self.canvas.set_draw_color(rgb)
	}

	pub fn draw_pixel(&mut self, x: i32, y: i32, color: Option<Vec3>) {
		if let Some(color) = color {
			let r = (color.x() * 255.999) as u8;
			let g = (color.y() * 255.999) as u8;
			let b = (color.z() * 255.999) as u8;
			self.canvas.set_draw_color(sdl2::pixels::Color::RGB(r, g, b));
		}
		self.canvas.draw_rect(Rect::new(x, y, 1, 1)).unwrap()
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
			::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
		}
	}

	pub fn width(&self) -> u32 {
		self.width
	}

	pub fn height(&self) -> u32 {
		self.height
	}

	pub fn show(&mut self) {
		self.canvas.present();
	}
}