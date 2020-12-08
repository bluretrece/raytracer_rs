use std::ops::{Add, Mul, Div, Sub};
mod vec3;
mod ray;
use vec3::Vec3;
use ray::Ray;
use image::*;


/// TODO
/// Handle image rendering with the image crate
/// Support parallel operations with Rayon

const IMAGE_WIDTH: i32 = 1000;
const IMAGE_HEIGHT:i32 = 1000;

pub fn hit_sphere(center: Vec3, radius:f32, r: &Ray) -> f32 {
    let oc: Vec3 = *r.origin() - center;
    let a = r.direction().length().sqrt(); // Works?
    let half_b = Vec3::dot(&oc, r.direction());
    let c = oc.length().sqrt() - radius*radius;
    let discriminant = (half_b * half_b) - (a *c);
    

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / (a)
    }
}


fn ray_color(r: &Ray) -> Vec3 {
    let mut t: f32 = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, &r);
    match t {
        t if t > 0.0 => {
            let n: Vec3 = Vec3::unit_vector(r.at(t) - Vec3::new(0.0,0.0,-1.0));
            0.5 * Vec3::new(n.get_x() + 1.0, n.get_y() + 1.0, n.get_z() + 1.0)
        }
        _ => { 
            let unit_direction: Vec3 = Vec3::unit_vector(*r.direction());
            t = 0.5 * (unit_direction.get_y() + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7,1.0) * t
        }
    }
}
fn main() {
    /// IMAGE
    let aspect_ratio = 16.0/9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as f32;

    /// CAMERA

    let viewport_height:f32 = 2.0;
    let viewport_width  = aspect_ratio * viewport_height;
    let focal_length:f32 = 1.0;

    let origin = Vec3::new(0.0,0.0,0.0);
    let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner:Vec3 = origin - (horizontal/2.0) - (vertical/2.0) - (Vec3::new(0.0,0.0, focal_length));

    /// RENDER
    println!("P3\n{} {} \n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let v:f32  = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            
            let direction: Vec3 = lower_left_corner + (u * horizontal) + (v*vertical) - origin;
            let ray = Ray::new(origin, direction);

            let pixel_color = ray_color(&ray);

            let ir = ( pixel_color.get_x() * 255.999) as i32;
            let ig = ( pixel_color.get_y()* 255.999) as i32;
            let ib = ( pixel_color.get_z()* 255.999) as i32;


            println!("{} {} {}", ir, ig, ib);
            
        }    
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vec3_mul() {
        let vec3 = Vec3::new(2.0,4.0,3.0);
        
        let t = 3.0;
        let sum = vec3 * t;

        assert_eq!(sum, Vec3::new(6.0,12.0,9.0)) 
    }
    #[test]
    fn vec3_sum() {
        let vec3 = Vec3::new(2.0,4.0,3.0);
        
        let vec3_2= Vec3::new(2.0,4.0,3.0);
        let sum = vec3 + vec3_2;
        assert_eq!(sum, Vec3::new(4.0, 8.0, 6.0)) 
    }
    #[test]
    fn vec3_div() {
        let vec3 = Vec3::new(8.0,4.0,2.0);

        let dv = vec3 / 2.0;
        assert_eq!(dv, Vec3::new(4.0, 2.0, 1.0)) 
    }
} 