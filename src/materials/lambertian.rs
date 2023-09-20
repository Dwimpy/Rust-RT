use crate::materials::Scatter;
use crate::types::hit::HitRecord;
use crate::types::ray::Ray;
use crate::types::vec3::{Color, Vec3};

pub struct Lambertian {
	albedo: Color,
}

impl Lambertian {
	pub fn new (color: Color) -> Self {
		Lambertian {
			albedo: color,
		}
	}
}

impl Scatter for Lambertian {
	fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<(Color, Ray)>{
		let mut scatter_dir = hit_rec.normal + Vec3::random_unit_sphere();
		if scatter_dir.near_zero() {
			scatter_dir = hit_rec.normal;
		}
		let scattered = Ray::new(hit_rec.isec_point, scatter_dir);
		Some((self.albedo, scattered))
	}
}