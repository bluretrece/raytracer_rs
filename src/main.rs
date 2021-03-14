mod camera;
mod hit_record;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hit_record::HitRecord;
use hittable::Hittable;
use hittable_list::HittableList;
use material::{scatter, Material};
use rand::*;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

pub const PI: f32 = std::f32::consts::PI;

const IMAGE_WIDTH: i32 = 715;
const IMAGE_HEIGHT: i32 = 400;
const SAMPLES_PER_PIXEL: i32 = 500;
const MAX_DEPTH: i32 = 50;

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

fn ray_color(r: &Ray, world: &dyn Hittable, depth: f32) -> Vec3 {
    let mut rec = HitRecord::default();

    if let Some(rec) = world.hit(r, 0.001, std::f32::MAX) {
        let mut scattered = Ray::new(Vec3::default(), Vec3::default());
        let mut attentuation = Vec3::default();

        if depth < 50.0 && scatter(&rec.mat_ptr, r, &rec, &mut attentuation, &mut scattered) {
            return attentuation * ray_color(&scattered, world, depth + 1.0);
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    } else {
        let unit_direction = Vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.get_y() + 1.0);

        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }

    // let mut rec = HitRecord::default();

    // if depth < 0.0 {
    //     return Vec3::new(0.0, 0.0, 0.0);
    // } else if world.hit(r, 0.001, std::f32::INFINITY, &mut rec) {
    //     let mut scattered: Ray = Ray::new(Vec3::default(), Vec3::default());
    //     let mut attenuation = Vec3::default();

    //     if rec
    //         .mat_ptr
    //         .scatter(&r, &rec, &mut attenuation, &mut scattered)
    //     {
    //         return attenuation * ray_color(&scattered, world, depth as f32 - 1.0);
    //     } else {
    //         Vec3::new(0.0, 0.0, 0.0)
    //     }
    // } else {
    //     let unit_direction: Vec3 = Vec3::unit_vector(&r.direction());
    //     let t = 0.5 * (unit_direction.get_y() + 1.0);
    //     (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    // }
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
pub fn _random_double() -> f32 {
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
    let pixel_color = pixel_color / samples_per_pixel as f32;

    let col = Vec3::new(
        pixel_color.get_x().sqrt(),
        pixel_color.get_y().sqrt(),
        pixel_color.get_z().sqrt(),
    );

    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let ir = (255.99 * col.get_x()) as i32;
    let ig = (255.99 * col.get_y()) as i32;
    let ib = (255.99 * col.get_z()) as i32;
    // let ir = (255.99 * clamp(r, 0.0, 0.999)) as i32;
    // let ig = (255.99 * clamp(g, 0.0, 0.999)) as i32;
    // let ib = (255.99 * clamp(b, 0.0, 0.999)) as i32;

    println!("{} {} {}", ir, ig, ib);
}

fn main() {
    // IMAGE
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let _image_height = (image_width as f32 / aspect_ratio) as f32;

    // WORLD
    //
    let R = (PI / 4.0).cos();

    let material_ground = Material::Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    };
    let material_center = Material::Lambertian {
        albedo: Vec3::new(0.1, 0.2, 0.5),
    };
    let material_left = Material::Dielectric { ir: 1.5 };

    let material_right = Material::Metal {
        albedo: Vec3::new(0.8, 0.6, 0.2),
        fuzz: 0.0,
    };

    let mut world: HittableList = HittableList::new();

    world.objects.push(Box::new(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        mat_ptr: material_ground,
    }));
    world.objects.push(Box::new(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        mat_ptr: material_center,
    }));
    world.objects.push(Box::new(Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: -0.4,
        mat_ptr: material_left,
    }));
    world.objects.push(Box::new(Sphere {
        center: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        mat_ptr: material_right,
    }));

    // CAMERA
    let camera = Camera::new(90.0, aspect_ratio);

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
                let u: f32 = (i as f32 + rng.gen::<f32>()) / (IMAGE_WIDTH) as f32;
                let v: f32 = (j as f32 + rng.gen::<f32>()) / (IMAGE_HEIGHT) as f32;

                let ray = camera.get_ray(u, v);
                let ray_color: Vec3 = ray_color(&ray, &world, 0 as f32);
                pixel_color = pixel_color + ray_color;
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn vec3_mul() {
//         let vec3 = Vec3::new(2.0, 4.0, 3.0);

//         let t = 3.0;
//         let sum = vec3 * t;

//         assert_eq!(sum, Vec3::new(6.0, 12.0, 9.0))
//     }
//     #[test]
//     fn vec3_sum() {
//         let vec3 = Vec3::new(2.0, 4.0, 3.0);

//         let vec3_2 = Vec3::new(2.0, 4.0, 3.0);
//         let sum = vec3 + vec3_2;
//         assert_eq!(sum, Vec3::new(4.0, 8.0, 6.0))
//     }
//     #[test]
//     fn vec3_div() {
//         let vec3 = Vec3::new(8.0, 4.0, 2.0);

//         let dv = vec3 / 2.0;
//         assert_eq!(dv, Vec3::new(4.0, 2.0, 1.0))
//     }
// }
