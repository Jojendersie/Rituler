use std::ops::Add;
use std::ops::Sub;

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

/*impl Clone for Vector {
	fn clone(&self) -> Self {
		Vector { x: self.x, y: self.y }
	}
}*/
