use std::ops::{
    Add, AddAssign,
    Div, DivAssign,
    Mul, MulAssign,
    Neg, Range,
    Sub
};

pub type Point3 = Vec3;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Self {x, y, z} 
    }

    /// Return a random vec3
    pub fn rand() -> Vec3 {
        Self {
            x: fastrand::f64(), 
            y: fastrand::f64(), 
            z: fastrand::f64()
        }
    }

    /// Returns a random vec3 where all values are inside the given range
    pub fn rand_range(range: Range<f64>) -> Vec3 {
        let range_len = range.end - range.start;
        let x = range.start + (range_len * fastrand::f64());
        let y = range.start + (range_len * fastrand::f64());
        let z = range.start + (range_len * fastrand::f64());

        Self {x, y, z}
    }

    /// Generate a random vec3 inside the unit sphere using the rejection method
    pub fn rand_in_unit_sphere() -> Vec3 {
        loop {
            let vec = Vec3::rand_range(-1.0..1.0);
            if vec.len_squared() < 1.0 {
                return vec;
            }
        }
    }

    /// Generate a random vec3 inside the unit circle using the rejection method
    pub fn rand_in_unit_disk() -> Vec3 {
        loop {
            let vec = Vec3::new(
                -1.0 + (2.0 * fastrand::f64()),
                -1.0 + (2.0 * fastrand::f64()),
                0.0
            );
            if vec.len_squared() < 1.0 {
                return vec;
            }
        }
    }

    pub fn rand_unit_vec() -> Vec3 {
        Self::rand_in_unit_sphere().unit_vec()
    }

    pub fn rand_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::rand_in_unit_sphere();
        if in_unit_sphere.dot(&normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
    
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x),
        }
    }

    /// Calculate len²
    pub fn len_squared(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }
    
    /// Calculate vector length / magnitude
    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn unit_vec(&self) -> Vec3 {
        self / self.len()
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - self.dot(&normal) * normal * 2.0
    }

    pub fn refract(&self, normal: &Vec3, eta_ratio: f64) -> Vec3 {
        let cos_theta = -self.dot(&normal).min(1.0);
        let ray_perpendicular = eta_ratio * (*self + cos_theta * normal);
        let ray_parallel = -(1.0 - ray_perpendicular.len_squared()).abs().sqrt() * normal;

        ray_perpendicular + ray_parallel
    }

    pub fn is_near_zero(&self) -> bool {
        let almost_zero = 1e-8;
        [self.x, self.y, self.z].iter().any(|&val| val.abs() < almost_zero)
    }
}

// Impls of Vec3 unary operators
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {x: -self.x, y: -self.y, z: -self.z}
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {x: -self.x, y: -self.y, z: -self.z}
    }
}

// Impls of operations between Vec3 and Vec3
impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::add(self, -rhs)
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3::add(self, -rhs)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs);
    }
}

// Impls of operations between Vec3 and f64
// We need two Mul impls to make the order transitive

impl Mul<f64> for Vec3 {
    type Output = Self;
    
    fn mul(self, rhs: f64) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = self.mul(rhs);
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    
    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        self * (*rhs)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = self.div(rhs);
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::div(*self, rhs)
    }
}
