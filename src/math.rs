use std::ops::Add;
use std::ops::Sub;
use std::f32;

#[derive(Copy, Clone)]
pub struct Vector{
    pub x: f32,
    pub y: f32,
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector { x: self.x - other.x, y: self.y - other.y }
    }
}

impl Vector {

	pub fn normalize(&mut self) {
		let len = f32::sqrt(self.x * self.x + self.y * self.y) + 1.0e-6;
		self.x /= len;
		self.y /= len;
	}

}