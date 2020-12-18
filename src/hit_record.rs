use crate::Vec3;
use crate::ray::Ray;

#[derive(Default)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}


impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = (Vec3::dot(r.direction(), outward_normal)) < 0.0;
        self.normal = match self.front_face {
            true =>  *outward_normal,
            false => *outward_normal * 1.0 
        }
    }

    pub fn p(&self) -> Vec3 {
        self.p
    }
    pub fn t(&self) -> f32 {
        self.t
    }
    pub fn normal(&self) -> Vec3 {
        self.normal
    }
    pub fn set_p(&mut self, val: Vec3) {
        self.p = val
    }

    pub fn set_t(&mut self, val: f32){
        self.t = val
    }

    pub fn set_normal(&mut self, val: Vec3) {
        self.normal = val
    }
}