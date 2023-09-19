use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

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
        Vec3 {
            e: [self.e[0] / self.length(), self.e[1] / self.length(), self.e[2] / self.length()]
        }
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

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [self.e[0] * rhs.e[0], self.e[1] * rhs.e[1], self.e[2] * rhs.e[2]]
        }
    }
}

impl Mul<Vec3> for f64 {
	type Output = Vec3;

	fn mul(self, rhs: Self::Output) -> Self::Output {
		Vec3 {
			e: [self * rhs.e[0], self * rhs.e[1], self * rhs.e[1]]
		}
	}
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = Vec3 {
            e: [self.e[0] * rhs.e[0], self.e[1] * rhs.e[1], self.e[2] * rhs.e[2]]
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Vec3 {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs]
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