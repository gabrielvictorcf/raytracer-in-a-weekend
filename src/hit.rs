use std::ops::Range;
use std::sync::Arc;

use crate::color;
use crate::color::Color;
use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use crate::material::{Material, Scatter};

pub trait Hit: Send + Sync {
    fn try_hit(&self, ray: &Ray, interval: &Range<f64>) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub t: f64,
    pub p: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Arc<dyn Material>
}

impl HitRecord {
    /// p is point hit when ray travelled t time
    /// Calculates the front face internally
    pub fn new(t: f64, p: Point3, outward_normal: Vec3, ray: &Ray, material: Arc<dyn Material>) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = match front_face {
            true => outward_normal,
            false => -outward_normal
        };

        Self { t, p, normal, front_face, material }
    }

    // pub fn scatter(&self, hit: &HitRecord, ray: &Ray) -> Option<Scatter> {
    //     self.material.scatter(hit, ray)
    // }

    /// Calls the hit material's underlying scatter function
    pub fn scatter(&self, ray: &Ray) -> Option<Scatter> {
        self.material.scatter(self, ray)
    }
}

#[derive(Default)]
pub struct HittableList {
    hittables: Vec<Box<dyn Hit>>
}

impl HittableList {
    /// The lifetime 'static here means that geometry owns all it's data
    pub fn add(&mut self, geometry: impl Hit + 'static) {
        self.hittables.push(Box::new(geometry));
    }

    /// Shoot ray into world and return the closest element it hits
    pub fn shoot_ray(&self, ray: &Ray, mut interval: Range<f64>) -> Option<HitRecord> {
        // Shoot the ray at every "Hit" object in the list.
        // To get the object which is hit first (has the lowest 't') the interval.end
        // decreases every time a hit is sucessfull.
        let mut hit_record = None;
        for hittable in &self.hittables {
            if let Some(hit) = hittable.try_hit(&ray, &interval) {
                interval.end = hit.t;
                hit_record = Some(hit);
            }
        }

        hit_record
    }

    /// Shoot ray into world and simulate bouncing and scattering for a max of
    /// `bounces` child rays.
    pub fn find_ray_color(&self, mut ray: Ray, mut bounces: u32) -> Color {
        // Ray starts with full energy, which is white {1.0, 1.0, 1.0} and
        // gets attenuated each hit (how much depends on the hittable albedo)
        let mut ray_color = color::WHITE;
        while bounces > 0 {
            match self.shoot_ray(&ray, 0.001..f64::INFINITY) {
                Some(hit) => {
                    // match hit.scatter(&hit, &ray) {
                    match hit.scatter(&ray) {
                        Some((scattered, attenuation)) => {
                            // If ray hit something and bounced, shoot the scattered ray
                            bounces = bounces - 1;
                            ray = scattered;
                            ray_color *= attenuation;    // Attenuate ray color
                        },
                        // Otherwise ray was absorbed and lost all energy
                        None => return color::BLACK,
                    };
                },
                // Ray returned to camera - we found it's color.
                None => return ray_color * Color::from(&ray),
            };
        }

        color::BLACK    // If ray exhausts it's bounces, it lost all energy
    }
}
