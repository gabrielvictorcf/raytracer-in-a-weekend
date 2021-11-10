use crate::color::Color;
pub use crate::hit::HitRecord;
pub use crate::ray::Ray;
use crate::vec3::Vec3;

pub type Scatter = (Ray, Color);

pub trait Material: Send + Sync {
    fn scatter(&self, hit: &HitRecord, ray: &Ray) -> Option<Scatter>;
}

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, hit: &HitRecord, ray: &Ray) -> Option<Scatter> {
        let mut scatter_direction = hit.normal + Vec3::rand_unit_vec();

        // Catch degenerate scatter directions (infinity, NaN, ...)
        if scatter_direction.is_near_zero() {
            scatter_direction = hit.normal;
        }

        let scattered = Ray::new(hit.p, scatter_direction);
        let attenuation = self.albedo;
        
        Some((scattered, attenuation))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = fuzz.min(1.0);
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, hit: &HitRecord, ray: &Ray) -> Option<Scatter> {
        let reflected = ray.direction.unit_vec().reflect(&hit.normal);
        let scattered = Ray::new(hit.p, reflected + self.fuzz * Vec3::rand_in_unit_sphere());
        let attenuation = self.albedo;

        match scattered.direction.dot(&hit.normal) > 0.0 {
            true => Some((scattered, attenuation)),
            false => None,
        }
    }
}

pub struct Dielectric {
    refraction: f64
}

impl Dielectric {
    pub fn new(refraction: f64) -> Self {
        Self { refraction }
    }

    /// Schlick's approximation for reflectance
    pub fn reflectance(cos: f64, refraction: f64) -> f64 {
        let r0 = (1.0 - refraction) / (1.0 + refraction);
        let r0 = r0*r0;
        
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, hit: &HitRecord, ray: &Ray) -> Option<Scatter> {
        let attenuation = crate::color::WHITE;
        let refrac_ratio = match hit.front_face {
            true => 1.0 / self.refraction,
            false => self.refraction,
        };

        let unit_direction = ray.direction.unit_vec();
        let cos_theta = -unit_direction.dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        
        // If our ray is *inside* the object, there are no real solutions
        // to Snell's law -> so we reflect instead!
        let mut cannot_refract = refrac_ratio * sin_theta > 1.0;
        cannot_refract |= Dielectric::reflectance(cos_theta, refrac_ratio) > fastrand::f64();
        
        let direction = match cannot_refract {
            true => unit_direction.reflect(&hit.normal),
            false => unit_direction.refract(&hit.normal, refrac_ratio),
        };
        
        let scattered = Ray::new(hit.p, direction);
        Some((scattered, attenuation))
    }
}