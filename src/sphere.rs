use std::ops::Range;
use std::sync::Arc;

use crate::hit::Hit;
use crate::material::*;
use crate::vec3::Point3;

#[derive(Clone)]
pub struct Sphere{
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>
}

impl Sphere {
    pub fn new(x: f64, y: f64, z: f64, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center: Point3::new(x, y, z),
            radius,
            material
        }
    }
}

impl Hit for Sphere {
    fn try_hit(&self, ray: &Ray, interval: &Range<f64>) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.len_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.len_squared() - (self.radius * self.radius);

        let discriminant = (half_b * half_b) - (a * c);
        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sqrt = discriminant.sqrt();

        // Try to get both the +Δ and -Δ roots
        let mut root = (-half_b - discriminant_sqrt) / a;
        if !interval.contains(&root) {
            root = (-half_b + discriminant_sqrt) / a;
            if !interval.contains(&root) {
                return None;
            }
        }

        let hit_point = ray.at(root);
        let outward_normal = (hit_point - self.center) / self.radius;
        let hit = HitRecord::new(
            root,
            hit_point,
            outward_normal,
            &ray,
            Arc::clone(&self.material)
        );
        
        Some(hit)
    }
}
