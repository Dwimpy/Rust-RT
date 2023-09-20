use crate::types::hit::HitRecord;
use crate::types::ray::Ray;
use crate::types::vec3::Color;

pub trait Scatter: Send + Sync {
	fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<(Color, Ray)>;
}
pub mod lambertian;
pub mod metal;
pub mod dielectric;