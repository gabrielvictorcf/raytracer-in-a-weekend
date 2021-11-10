use crate::{
    ray::Ray,
    vec3::{Point3, Vec3}
};

pub struct Camera {
    origin: Point3,
    x_axis: Vec3,
    y_axis: Vec3,
    lower_left_corner: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        fov_vertical: f64, 
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64
    ) -> Self {
        let theta = fov_vertical.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vec();
        let u = vup.cross(&w).unit_vec();
        let v = w.cross(&u);

        let origin = lookfrom;
        let x_axis = focus_dist * viewport_width * u;
        let y_axis = focus_dist * viewport_height * v;
        let lower_left_corner = origin - (x_axis/2.0) - (y_axis/2.0) - focus_dist * w;

        let lens_radius = aperture/2.0;
        
        Self {
            origin, x_axis, y_axis,
            lower_left_corner,
            u, v, w,
            lens_radius
        }
    }

    pub fn gen_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::rand_in_unit_disk();
        let off = self.u * rd.x + self.v * rd.y;

        let sx = s * self.x_axis;
        let ty = t * self.y_axis;

        let origin = self.origin + off;
        let direction = self.lower_left_corner + sx + ty - self.origin - off;
        Ray::new(origin, direction)
    }
}
