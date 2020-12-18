use crate::hittable::Hittable;
use crate::Vec3;
use crate::ray::Ray;
use crate::hit_record::HitRecord;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = *r.origin() - self.center;
        let a = Vec3::dot(&r.direction(), &r.direction());
        let b = Vec3::dot(&oc, &r.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;

        let discriminant = b*b - a * c;
        
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.set_t(temp);
                rec.set_p(r.at(rec.t()));
                rec.set_normal((rec.p() - self.center) / self.radius);
                return true;
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.set_t(temp);
                rec.set_p(r.at(rec.t()));
                rec.set_normal((rec.p() - self.center) / self.radius);
                return true;
            } 
        } 

        return false;

        //let sqrtd = discriminant.sqrt();

        //let mut root = (-half_b - sqrtd) / a;
        //if root < t_min || t_max < root {
        //    root = (-half_b + sqrtd) / a;
        //    if root < t_min || t_max < root {
        //        return false
        //    }
        //}

        //rec.t = root;
        //rec.p = r.at(rec.t);
        //rec.normal = (rec.p - self.center) / self.radius;
        //let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        //rec.set_face_normal(r, &outward_normal);
        //true

    }
}