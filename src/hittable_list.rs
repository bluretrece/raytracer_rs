use crate::hittable::Hittable;
use crate::hit_record::HitRecord;
use crate::ray::Ray;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new()
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                hit_anything = Some(rec);
            }
        }

        hit_anything
    }
}
