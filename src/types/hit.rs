use std::rc::Rc;
use crate::materials::Scatter;
use crate::types::ray::Ray;
use crate::types::vec3::{Point3, Vec3};

pub type World = Vec<Box<dyn Hit>>;

pub struct HitRecord {
	pub isec_point: Point3,
	pub normal: Vec3,
	pub t: f64,
	pub mat: Rc<dyn Scatter>,
	pub front_face: bool,
}

impl HitRecord {
	pub fn set_face_normal(&mut self, ray: &Ray, out_normal: Vec3) -> () {
		self.front_face = ray.direction().dot(out_normal) < 0.0;
		self.normal = if self.front_face {
			out_normal
		}else {
			(-1.0) * out_normal
		}
	}
}

pub trait Hit {
	fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl Hit for World {
	fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
		let mut tmp_rec = None;
		let mut closest = t_max;

		for obj in self {
			if let Some(hit_rec) = obj.hit(ray, t_min, closest) {
				closest = hit_rec.t;
				tmp_rec = Some(hit_rec);
			}
		}
		tmp_rec
	}
}