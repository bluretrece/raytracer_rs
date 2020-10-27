#[derive(Clone, Debug, Copy, PartialEq)]
struct Vec3 {
    e: [f32; 3]
}

impl Vec3 {
    fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Self { e: [e0, e1, e2]}
    }

    fn get_x(&self) -> f32 {
        self.e[0]
    }

    fn get_y(&self) -> f32 {
        self.e[1]
    }

    fn get_z(&self) -> f32 {
        self.e[2]
    }

    fn length(&self) -> f32 {
        (
            self.e[0] * self.e[0] 
            + self.e[1] * self.e[1] 
            + self.e[2] * self.e[2]
        )
        .sqrt()
    }

    fn unit_vector(v: Vec3) -> Vec3 {
        return v / v.length()
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