use std::rc::Rc;
use std::sync::Arc;
use crate::materials::Scatter;
use crate::types::hit::{Hit, HitRecord};
use crate::types::ray::Ray;
use crate::types::vec3::{Point3, Vec3};

pub struct Sphere {
	origin: Point3,
	radius: f64,
	mat: Arc<dyn Scatter>
}

impl Sphere {
	pub fn new(origin: Point3, radius: f64, mat: Arc<dyn Scatter>) -> Self {
		Self {
			origin,
			radius,
			mat,
		}
	}
}

impl Hit for Sphere {
	fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
		let oc = ray.origin() - self.origin;
		let a = ray.direction().length().powi(2);
		let half_b = oc.dot(ray.direction());
		let c = oc.length().powi(2) - self.radius.powi(2);
		let disc =  half_b.powi(2) - a * c;

		if disc < 0.0 {
			return None
		}

		let sqrt_disc = disc.sqrt();
		let mut root = (-half_b - sqrt_disc) / a;
		if root < t_min || t_max < root {
			root = (-half_b + sqrt_disc) / a;
			if root < t_min || t_max < root {
				return None
			}
		}
		let mut hit_rec = HitRecord {
			t: root,
			isec_point: ray.at(root),
			normal: Vec3::new(0.0, 0.0, 0.0),
			mat: self.mat.clone(),
			front_face: false
		};
		let out_normal = ((hit_rec.isec_point - self.origin) / self.radius);
		hit_rec.set_face_normal(&ray, out_normal);
		Some(hit_rec)
	}
}