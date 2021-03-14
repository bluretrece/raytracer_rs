use crate::Vec3;
use crate::Ray;
use super::PI;


pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(vfov: f32, aspect_ratio: f32) -> Self {
        let theta = (vfov).to_radians();
        let h = (theta/2.0).tan();

        let viewport_height:f32 = 2.0 * h;
        let viewport_width  = aspect_ratio * viewport_height;

        let focal_length:f32 = 1.0;

        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let origin = Vec3::new(0.0,0.0,0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin- horizontal/2.0 - vertical/2.0 - Vec3::new(0.0,0.0, focal_length),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }
}
