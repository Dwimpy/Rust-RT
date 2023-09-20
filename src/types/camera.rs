use std::ops::Sub;
use crate::types::ray::Ray;
use crate::types::vec3::{Point3, Vec3};

pub struct Camera {
	origin: Point3,
	horizontal: Vec3,
	vertical: Vec3,
	viewport_width: f64,
	viewport_height: f64,
	lower_left_corner: Point3,
}

impl Camera {
	pub fn new (from: Point3, to: Point3, world_up: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
		let theta = std::f64::consts::PI / 180.0 * vfov;
		let viewport_height = 2.0 * (theta / 2.0).tan();
		let viewport_width = viewport_height * aspect_ratio;

		let mut forward = (from - to).normalized();
		if forward.y().is_nan() && forward.x().is_nan()  && forward.z().is_nan() {
			// Handle this case by setting a fixed up direction (e.g., Y axis)
			forward = Vec3::new(0.0, 1.0, 0.0);
		}
		let mut up = world_up.cross(forward).normalized();
		if up.y().is_nan() && up.x().is_nan()  && up.z().is_nan() {
			// Handle this case by setting a fixed up direction (e.g., Y axis)
			up = Vec3::new(1.0, 0.0, 0.0);
		}

		let right = forward.cross(up);
		let h = viewport_width * up;
		let v = viewport_height * right;
		let lower_left_corner = from - h / 2.0 - v / 2.0 - forward;
		Self {
			origin: from,
			horizontal: h,
			vertical: v,
			viewport_width,
			viewport_height,
			lower_left_corner,
		}
	}

	pub fn get_ray(&self, u: f64, v: f64) -> Ray {
		Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
	}
}