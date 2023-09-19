mod types;
mod image;
mod objects;

extern crate sdl2;
extern crate rand;

use rand::Rng;
use crate::image::canvas::Canvas;
use crate::objects::sphere::Sphere;
use crate::types::camera::Camera;
use crate::types::hit::{Hit, World};
use crate::types::ray::Ray;
use crate::types::vec3::{Color, Point3, Vec3};

fn ray_color(ray: &Ray, world: &World, depth: u64) -> Color {
	if depth <= 0 {
		return Color::new(0.0, 0.0, 0.0)
	}
	if let Some(hit_rec) = world.hit(ray, 0.001, f64::INFINITY)
	{
		let target = hit_rec.isec_point + hit_rec.normal + Vec3::random_unit_sphere().normalized();
		let ray = Ray::new(hit_rec.isec_point, target - hit_rec.isec_point);
		0.5 * ray_color(&ray, world, depth - 1)
	}
	else
	{
		let unit_dir = ray.direction().normalized();
		let t = 0.5 * (unit_dir.y() + 1.0);
		(1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
	}
}

fn main() {
	const SPP: u64 = 16;
	const MAX_DEPTH: u64 = 5;

    let mut image = Canvas::new("Ameri Ray Tracing", 1920, 1080);
    let camera = Camera::new(
        Point3::new(0.1, 0.5, -3.0),
        Point3::new(0.0,0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
		70.0,
		image.aspect_ratio()
    );
	let mut world = World::new();
	world.push(Box::new(Sphere::new(
		Point3::new(0.0, 0.0, -1.0),
		0.5,
	)));
	world.push(Box::new(Sphere::new(
		Point3::new(0.0, -100.5, -1.0),
		100.0,
	)));
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
				pixel_color += ray_color(&ray, &world, MAX_DEPTH);
			}
			image.draw_pixel(i as i32, (image.height() - j) as i32, Some(pixel_color / SPP as f64));
        }
    }
    image.show();
    image.event_loop();
}
