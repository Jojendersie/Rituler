extern crate sdl2;

use math;
use drawable;

pub struct Actor<'a> {
	pub m_sprite : drawable::Sprite<'a>,
	pub m_life : f32,
	pub m_max_life : f32,
}

impl<'a> Actor<'a> {
	pub fn new(_vec: math::Vector, _texture: &sdl2::render::Texture, _max_life: f32) -> Actor {
		Actor {
			m_sprite: drawable::Sprite::new(_vec, _texture),
			m_life: _max_life,
			m_max_life: _max_life
		}
	}
}