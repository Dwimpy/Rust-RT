use std::fmt::{Display, Formatter};
use crate::types::vec3::{Point3, Vec3};

pub struct Ray {
	origin: Point3,
	direction: Vec3,
}

impl Ray {
	pub fn new (origin: Point3, direction: Vec3) -> Ray {
		Ray {
			origin,
			direction
		}
	}

	pub fn origin(&self) -> Point3 {
		self.origin
	}

	pub fn direction(&self) -> Vec3 {
		self.direction
	}

	pub fn at(&self, t: f64) -> Point3 {
		self.origin + t * self.direction
	}
}

impl Display for Ray {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		println!("------  Ray  -------");
		println!("Ray Origin: {}", self.origin);
		println!("Ray Position: {}", self.origin);
		println!("------ End -------");
		Ok(())
	}
}