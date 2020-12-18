mod camera;
mod hit_record;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hit_record::HitRecord;
use hittable::Hittable;
use hittable_list::HittableList;
use rand::*;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

const IMAGE_WIDTH: i32 = 500;
const IMAGE_HEIGHT: i32 = 500;
const SAMPLES_PER_PIXEL: i32 = 500;
const MAX_DEPTH: i32 = 50;

/// TODO
/// Handle image rendering with the image crate
/// Support parallel operations with Rayon

pub fn hit_sphere(center: Vec3, radius: f32, r: &Ray) -> f32 {
    let oc: Vec3 = *r.origin() - center;
    let a = r.direction().length().sqrt(); // Works?
    let half_b = Vec3::dot(&oc, r.direction());
    let c = oc.length().sqrt() - radius * radius;
    let discriminant = (half_b * half_b) - (a * c);

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / (a)
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();

    loop {
        let p = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
            - Vec3::new(1.0, 1.0, 1.0);

        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

fn depth_exceeded(depth: i32) -> bool {
    depth <= 0
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    let mut rec = HitRecord::default();

    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    } else if world.hit(r, 0.0, std::f32::MAX, &mut rec) {
        let target: Vec3 = rec.p() + rec.normal + random_in_unit_sphere();
        let ray: Ray = Ray::new(rec.p(), target - rec.p());
        let ray_color = 0.5 * ray_color(&ray, world, depth - 1);

        ray_color
    } else {
        let unit_direction: Vec3 = Vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.get_y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

// Utility
fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    } else if x > max {
        return max;
    }

    x
}

// generates a float between 0 and 1
fn _random_double() -> f32 {
    let mut rng = rand::thread_rng();
    let y: f32 = rng.gen();

    y
}

fn write_color(pixel_color: Vec3, samples_per_pixel: i32) {
    let mut r = pixel_color.get_x();
    let mut g = pixel_color.get_y();
    let mut b = pixel_color.get_z();

    // We divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f32;

    r *= scale;
    g *= scale;
    b *= scale;

    let ir = (clamp(r, 0.0, 0.999) * 255.99) as i32;
    let ig = (clamp(g, 0.0, 0.999) * 255.99) as i32;
    let ib = (clamp(b, 0.0, 0.999) * 255.99) as i32;

    println!("{} {} {}", ir, ig, ib);
}

fn main() {
    // IMAGE
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let _image_height = (image_width as f32 / aspect_ratio) as f32;

    // WORLD
    let mut world: HittableList = HittableList::new();
    world.objects.push(Box::new(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    }));
    world.objects.push(Box::new(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    }));

    // CAMERA
    let camera = Camera::new();

    let viewport_height: f32 = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length: f32 = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);
    let _lower_left_corner: Vec3 =
        origin - (horizontal / 2.0) - (vertical / 2.0) - (Vec3::new(0.0, 0.0, focal_length));

    // RENDER
    println!("P3\n{} {} \n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut rng = rand::thread_rng();

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color: Vec3 = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u: f32 = (i as f32 + rng.gen::<f32>()) / (IMAGE_WIDTH - 1) as f32;
                let v: f32 = (j as f32 + rng.gen::<f32>()) / (IMAGE_HEIGHT - 1) as f32;

                let ray = camera.get_ray(u, v);
                let ray_color: Vec3 = ray_color(&ray, &world, MAX_DEPTH);
                pixel_color = pixel_color + ray_color;
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vec3_mul() {
        let vec3 = Vec3::new(2.0, 4.0, 3.0);

        let t = 3.0;
        let sum = vec3 * t;

        assert_eq!(sum, Vec3::new(6.0, 12.0, 9.0))
    }
    #[test]
    fn vec3_sum() {
        let vec3 = Vec3::new(2.0, 4.0, 3.0);

        let vec3_2 = Vec3::new(2.0, 4.0, 3.0);
        let sum = vec3 + vec3_2;
        assert_eq!(sum, Vec3::new(4.0, 8.0, 6.0))
    }
    #[test]
    fn vec3_div() {
        let vec3 = Vec3::new(8.0, 4.0, 2.0);

        let dv = vec3 / 2.0;
        assert_eq!(dv, Vec3::new(4.0, 2.0, 1.0))
    }
}
