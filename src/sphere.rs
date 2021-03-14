use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use crate::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub mat_ptr: Material,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vec3 = *r.origin() - self.center;
        let a = Vec3::dot(&r.direction(), &r.direction());
        let b = Vec3::dot(&oc, &r.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    p: r.at(temp),
                    normal: (r.at(temp) - self.center) / self.radius,
                    mat_ptr: self.mat_ptr,
                    front_face: true,
                });
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord {
                    t: temp,
                    p: r.at(temp),
                    normal: (r.at(temp) - self.center) / self.radius,
                    mat_ptr: self.mat_ptr,
                    front_face: true
                });
            }
        }
        None
    }
}
