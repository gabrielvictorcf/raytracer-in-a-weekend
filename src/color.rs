use crate::vec3::{Vec3};
use crate::ray::Ray;

use image::Rgb;

pub const WHITE: Color = Color{ x: 1.0, y: 1.0, z: 1.0 };
pub const BLACK: Color = Color{ x: 0.0, y: 0.0, z: 0.0 };
pub const LIGHT_BLUE: Color = Color{ x: 0.5, y: 0.7, z: 1.0 };

pub type Color = Vec3;

impl Color {
    /// Turn a color into an Rgb<[u8; 3]> while also applying gamma correction.
    pub fn to_rgb(&self, samples: f64) -> Rgb<u8> {
        let r = (256.0 * (self.x / samples).sqrt().clamp(0.0, 0.999)) as u8;
        let g = (256.0 * (self.y / samples).sqrt().clamp(0.0, 0.999)) as u8;
        let b = (256.0 * (self.z / samples).sqrt().clamp(0.0, 0.999)) as u8;

        Rgb([r, g, b])
    }
}

impl From<&Ray> for Color {
    /// Turn a ray into a color by lerp'ing white -> blue
    fn from(r: &Ray) -> Self {
        let unit_direction = r.direction.unit_vec();
        let t = 0.5 * (unit_direction.y + 1.0);

        (1.0 - t) * WHITE + t * LIGHT_BLUE
    }
}