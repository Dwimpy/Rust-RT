use std::rc::Rc;
use crate::materials::dielectric::Dielectric;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;
use crate::objects::sphere::Sphere;
use crate::types::camera::Camera;
use crate::types::hit::World;
use crate::types::vec3::{Color, Point3, Vec3};

pub struct Scene {
	world: World,
	camera: Option<Camera>,
}

impl Scene {
	pub fn new () -> Scene {
		let world = World::new();
		let camera = None;
		Self {
			world,
			camera,
		}
	}

	pub fn test_scene(&mut self, vfov: f64, aspect_ratio: f64) {
		self.camera = Some(Camera::new(
			Point3::new(-2.0, 2.0, 1.0),
			Point3::new(0.0, 0.0, -1.0),
			Vec3::new(0.0, 1.0, 0.0),
			vfov,
			aspect_ratio,
		));

		let ground_mat = Rc::new(Lambertian::new(Color::new(0.8,0.8,0.0)));
		let center_mat = Rc::new(Lambertian::new(Color::new(0.7,0.3,0.3)));
		let left_mat = Rc::new(Metal::new(Color::new(0.8,0.8,0.8), 0.1));
		let right_mat_inner = Rc::new(Dielectric::new(1.52));
		let right_mat = Rc::new(Dielectric::new(1.52));

		let ground = Sphere::new(Point3::new(0.0, -100.5, 0.0), 100.0, ground_mat);
		let center = Sphere::new(Point3::new(0.0, 0.0,0.0), 0.5, center_mat);
		let left = Sphere::new(Point3::new(1.5, 0.5,0.0), 1.0, left_mat);
		let right = Sphere::new(Point3::new(-1.5, 0.5,0.0), 0.5, right_mat);
		let right_inner = Sphere::new(Point3::new(-1.5, 0.5,0.0), -0.4, right_mat_inner);
		self.world.push(Box::new(ground));
		self.world.push(Box::new(center));
		self.world.push(Box::new(left));
		self.world.push(Box::new(right_inner));
		self.world.push(Box::new(right));
	}

	pub fn world(&self) -> &World {
		&self.world
	}

	pub fn camera(&self) -> &Option<Camera> {
		&self.camera
	}

	pub fn clear(&mut self) {
		self.world.clear();
		self.camera = None
	}
}