use std::ops::{Add, Mul, Div, Sub, Neg};
use rand::Rng;

#[derive(Clone, Debug, Copy, PartialEq, Default)]
pub struct Vec3 {
    pub e: [f32; 3]
}

impl Vec3 {
    pub fn squared_length(self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        Self {
            e: [rng.gen_range(-1.0, 1.0),rng.gen_range(-1.0, 1.0),rng.gen_range(-1.0, 1.0)]
        }
    }

    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Self { e: [e0, e1, e2]}
    }

    pub fn dot(v: &Vec3, u: &Vec3) -> f32 {
        let dot_product = u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2];
        return  dot_product as f32
    }

    pub fn get_x(&self) -> f32 {
        self.e[0]
    }

    pub fn get_y(&self) -> f32 {
        self.e[1]
    }

    pub fn get_z(&self) -> f32 {
        self.e[2]
    }

    pub fn length(&self) -> f32 {
        (
            self.e[0] * self.e[0] 
            + self.e[1] * self.e[1] 
            + self.e[2] * self.e[2]
        )
        .sqrt()
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
        return *v / v.length()
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]]
        }
    }
}

impl Sub for Vec3 {
    type Output =  Self;
    fn sub(self, v: Self) -> Self {
        Self {
            e: [self.e[0] - v.e[0], self.e[1] - v.e[1], self.e[2] - v.e[2]]
        }
    }
}

impl Add for Vec3 {
    type Output =  Self;

    fn add(self, v: Self) -> Self {
        Self {
            e: [self.e[0] + v.e[0], self.e[1] + v.e[1], self.e[2] + v.e[2]]
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, t: f32) -> Vec3 {
        let k: f32 = 1.0 / t;

        Vec3 {
            e: [self.e[0] * k, self.e[1] * k, self.e[2] * k]
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        Vec3 {
            e: [v.e[0] * self, v.e[1] * self, v.e[2] * self],
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output =  Self;

    fn mul(self, t: f32) -> Self {
        Self {
            e: [self.e[0] * t, self.e[1] * t, self.e[2] * t]
        }
    }
}
