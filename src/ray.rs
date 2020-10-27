#[derive(Clone, Debug, Copy, PartialEq)]
struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction
        }
    }
    fn origin(&self) -> &Vec3 {
        &self.origin
    }

    fn direction(&self) -> &Vec3 {
        &self.direction
    }

    fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }

}
