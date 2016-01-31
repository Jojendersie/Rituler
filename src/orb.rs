extern crate sdl2;

use sdl2::rect::{Point};

use math;
use drawable;
use actor;
use std::f32;

pub struct Orb<'a> {
	pub m_sprite : drawable::Sprite<'a>,
	pub m_quality : i32,
	pub m_phase : f32, // Orbs are moving slowly up and down
}

impl <'a> drawable::Drawable for Orb<'a>
{
	fn draw(&self, _renderer : &mut sdl2::render::Renderer, _cam_pos : &Point) {
		(&self.m_sprite as &drawable::Drawable).draw(_renderer, _cam_pos);
	}
}

// Constructor
impl<'a> Orb<'a> {
	pub fn new(_vec: math::Vector, _texture: &sdl2::render::Texture, _quality: i32) -> Orb {
		Orb {
			m_sprite: drawable::Sprite::new(_vec, _texture),
			m_quality: _quality,
			m_phase: 0.0
		}
	}
}

impl <'a> actor::Dynamic for Orb<'a>
{
	fn process(&mut self)
	{
		self.m_phase += 0.1;
		self.m_sprite.m_location.y += f32::sin(self.m_phase);
	}
}