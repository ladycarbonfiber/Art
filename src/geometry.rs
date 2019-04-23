use std::ops;
#[derive(Debug, Copy, Clone)]
pub struct CartPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl ops::Add<CartPoint> for CartPoint {
    type Output = CartPoint;
    fn add(self, rhs: CartPoint) -> CartPoint {
        CartPoint { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl ops::Mul<f64> for CartPoint {
    type Output = CartPoint;
    fn mul(self, rhs: f64) -> CartPoint {
        CartPoint { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl CartPoint {
    pub fn normalize(self, x_bound: (f64, f64), y_bound: (f64, f64), z_bound: (f64, f64)) -> CartPoint {
        let x = if x_bound.0 - x_bound.1 == 0.0 {
            0.0
        } else {
            (self.x - x_bound.0) / (x_bound.1 - x_bound.0)
        };
        let y = if x_bound.0 - x_bound.1 == 0.0 {
            0.0
        } else {
            (self.y - y_bound.0) / (y_bound.1 - y_bound.0)
        };
        let z = if z_bound.0 - z_bound.1 == 0.0{
            0.0
        } else {
            (self.z - z_bound.0) / (z_bound.1 - z_bound.0)
        };

        CartPoint { x, y, z }
    }
}