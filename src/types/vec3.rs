use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Range, Sub, SubAssign};
use rand::Rng;

#[derive(Copy, Clone)]

pub struct Vec3 {
    e: [f64; 3],
}

pub type Color = Vec3;
pub type Point3 = Vec3;

impl Vec3 {
    pub fn new (x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            e: [x, y, z]
        }
    }

    pub fn x(self) -> f64 {
        self.e[0]
    }

    pub fn y(self) -> f64 {
        self.e[1]
    }

    pub fn z(self) -> f64 {
        self.e[2]
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.e[0] * rhs.e[0] + self.e[1] * rhs.e[1] + self.e[2] * rhs.e[2]
    }

    pub fn cross(self, rhs: Self) -> Vec3 {
        Vec3 {
            e: [self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1], self.e[2] * rhs.e[0] - self.e[0] * rhs.e[2], self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0]]
        }
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }
    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn format_color(self) -> String {
        format!("{} {} {}",
            (255.999 * self.e[0]) as u64,
            (255.999 * self.e[1]) as u64,
            (255.999 * self.e[2]) as u64
        )
    }
    pub fn random(r: Range<f64>) -> Vec3 {
        let mut rng = rand::thread_rng();

        Vec3 {
            e: [rng.gen_range(r.clone()), rng.gen_range(r.clone()), rng.gen_range(r.clone())]
        }
    }
    pub fn random_unit_sphere() -> Vec3 {
        loop {
            let v = Vec3::random(-1.0..1.0);
            if v.length() < 1.0{
                 return v
            }
        }
    }

	pub fn reflect(self, n: Vec3) -> Vec3 {
		self - 2.0 * self.dot(n) * n
	}

	pub fn refract(self, n: Vec3, etai_over_etat: f64) -> Vec3 {
		let cos = ((-1.0) * self).dot(n).min(1.0);
		let r_perp = etai_over_etat * (self + cos * n);
		let r_parallel = -(1.0 - r_perp.length().powi(2)).abs().sqrt() * n;
		r_perp + r_parallel
	}
	pub fn near_zero(self) -> bool {
		self.e[0].abs() < 1e-8 && self.e[1].abs() < 1e-8 && self.e[2].abs() < 1e-8
	}
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2]]
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Vec3 {
            e: [self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2]]
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [self.e[0] - rhs.e[0], self.e[1] - rhs.e[1], self.e[2] - rhs.e[2]]
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Vec3 {
            e: [self.e[0] - rhs.e[0], self.e[1] - rhs.e[1], self.e[2] - rhs.e[2]]
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) -> () {
        *self = Vec3 {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs]
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [self * rhs.e[0], self * rhs.e[1], self * rhs.e[2]]
        }
    }
}

impl Mul<Vec3> for Vec3 {
	type Output =  Vec3;

	fn mul(self, rhs: Vec3) -> Vec3 {
		Vec3 {
		e: [self.e[0] * rhs.e[0], self.e[1] * rhs.e[1], self.e[2] * rhs.e[2]]
		}
	}
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) -> () {
        *self = Vec3 {
            e: [self.e[0] * rhs.e[0], self.e[1] * rhs.e[1], self.e[2] * rhs.e[2]]
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs]
        }
    }
}

impl Div<Vec3> for Vec3 {
	type Output = Vec3;

	fn div(self, rhs: Vec3) -> Vec3 {
		Vec3 {
			e: [self.e[0] / rhs.e[0], self.e[1] / rhs.e[1], self.e[2] / rhs.e[2]]
		}
	}
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Vec3 {
            e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs]
        }
    }
}
impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        *self = Vec3 {
            e: [self.e[0] / rhs.e[0], self.e[1] / rhs.e[1], self.e[2] / rhs.e[2]]
            }
        }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        &mut self.e[index]
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}