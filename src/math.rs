use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
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

impl Mul<f32> for Vector {
	type Output = Vector;
	
	fn mul(self, _rhs: f32) -> Vector {
		Vector { x : self.x * _rhs, y : self.y * _rhs }
	}
}

impl Mul<Vector> for f32 {
	type Output = Vector;
	
	fn mul(self, _rhs: Vector) -> Vector {
		Vector { x : _rhs.x * self, y : _rhs.y * self }
	}
}

impl Vector {

	pub fn normalize(&mut self) {
		let len = f32::sqrt(self.x * self.x + self.y * self.y) + 1.0e-6;
		self.x /= len;
		self.y /= len;
	}
	
	//rotates the vector around the given angle
	pub fn rotate(&mut self, _angle: f32){
		let mut angle = (_angle-45.0);
		println!("{}",angle);
	//	if _angle < 0.0 {angle = 360.0 + _angle;}
		let angleRad = angle * f32::consts::PI / 180.0;
		println!("{}",angleRad);
        let cosA = f32::cos(angleRad);
        let sinA = f32::sin(angleRad);
		
		let oldX = self.x;
        self.x = self.x * cosA - self.y * sinA;
        self.y = self.y * cosA + oldX * sinA;
		println!("{},{}",self.x, self.y);
    }
}


static mut xsNum: u32 = 12345871;

//simple procedual random numbers!
fn xor_shift() -> u32
{
unsafe{
	xsNum ^= xsNum << 13;
	xsNum ^= xsNum >> 17;
	xsNum ^= xsNum << 5;
	xsNum
	}
}

pub fn get_rand(_max : u32) -> i32
{
	(xor_shift() % (_max + 1)) as i32
}

