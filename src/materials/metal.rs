use crate::materials::Scatter;
use crate::types::hit::HitRecord;
use crate::types::ray::Ray;
use crate::types::vec3::{Color, Vec3};

pub struct Metal {
	albedo: Color,
	fuzz: f64,
}

impl Metal {
	pub fn new(albedo: Color, fuzz: f64) -> Self {
		Metal {
			albedo,
			fuzz,
		}
	}
}

impl Scatter for Metal {
	fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<(Color, Ray)> {
		let reflected_ray = ray.direction().reflect(hit_rec.normal).normalized();
		let scattered = Ray::new(hit_rec.isec_point, reflected_ray + self.fuzz * Vec3::random_unit_sphere());

		if scattered.direction().dot(hit_rec.normal) > 0.0 {
			Some((self.albedo, scattered))
		}else {
			None
		}
	}
}