use crate::geometry::*;
#[derive(Debug, Copy, Clone)]
pub struct LorentzAttractor {
   pub a: f64,
   pub b: f64,
   pub c: f64,
   pub step: f64,
   pub x: f64,
   pub y: f64,
   pub z: f64
}

impl Iterator for LorentzAttractor {
    //dx/dt = a(y-x)
    //dy/dt = x(b-z) -y
    //dz/dt = xy -cz
    type Item = CartPoint;
    fn next(&mut self) -> Option<CartPoint> {
        self.x = self.x + (self.step * self.a * (self.y - self.x));
        self.y = self.y + (self.step * (self.x * (self.b - self.z) - self.y));
        self.z = self.z + (self.step * (self.x * self.y - self.c * self.z));
        Some(CartPoint{x:self.x, y:self.y, z:self.z})
    }
}

