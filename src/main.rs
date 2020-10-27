use std::ops::{Add, Mul, Div, Sub};

const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT:i32 = 256;

fn ray_color(r: &Ray) -> Vec3 {
    let unit_direction: Vec3 = Vec3::unit_vector(*r.direction());
    let t: f32= 0.5 * (unit_direction.get_y() + 1.0);

    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7,1.0) * t
}



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