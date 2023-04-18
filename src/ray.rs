use std::ops;

#[derive(Debug)]
pub struct Ray {
    direction: [f64; 3],
}

impl Ray {
    pub fn new(x: f64, y: f64, z: f64) -> Ray {
        Ray {
            direction: [x, y, z]
        }
    }
}

impl ops::Add for Ray {
    type Output = Self;

    fn add(self, rhs: Ray) -> Ray {
        Ray {
            direction: [
                self.direction[0] + rhs.direction[0],
                self.direction[1] + rhs.direction[1],
                self.direction[2] + rhs.direction[2]]
        }
    }
}
