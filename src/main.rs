mod types;
mod image;
mod objects;
mod materials;

extern crate sdl2;
extern crate rand;

use std::rc::Rc;
use rand::Rng;
use crate::image::canvas::Canvas;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;
use crate::objects::sphere::Sphere;
use crate::types::camera::Camera;
use crate::types::hit::{Hit, World};
use crate::types::ray::Ray;
use crate::types::scene::Scene;
use crate::types::vec3::{Color, Point3, Vec3};

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

fn main() {
	const SPP: u64 = 64;
	const MAX_DEPTH: u64 = 25;
    let mut image = Canvas::new("Ameri Ray Tracing", 800, 600);
	let mut scene = Scene::new();
	scene.test_scene(90.0, image.aspect_ratio());
	let camera = scene.camera().as_ref().unwrap();
	let mut rng = rand::thread_rng();


    for j in 0..image.height(){
        for i in 0..image.width() {
			let mut pixel_color = Color::new(0.0, 0.0, 0.0);
			for sample in 0..SPP {
				let random_u: f64 = rng.gen();
				let random_v: f64 = rng.gen();
				let u = (i as f64 + random_u) / (image.width() - 1) as f64;
				let v =  (j as f64 + random_v) / (image.height() - 1) as f64;
				let ray = camera.get_ray(u, v);
				pixel_color += ray_color(&ray, scene.world(), MAX_DEPTH);
			}
			image.draw_pixel(i as i32, (image.height() - j) as i32, Some(pixel_color / SPP as f64));
        }
    }
    image.show();
    image.event_loop();
}
