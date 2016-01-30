extern crate sdl2;

use math;
use drawable;
use sdl2::rect::{Point, Rect};
use sdl2::pixels;
use std::cmp::max;

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

impl <'a> drawable::Drawable for Actor<'a>
{
	fn draw(&self, _renderer : &mut sdl2::render::Renderer, _cam_pos : &Point) {
		// Draw a life bar
		let max_size = max(self.m_sprite.m_sprite_size.0/2, self.m_sprite.m_sprite_size.1/2) as i32;
		let len = (self.m_max_life / 2.0) as i32;
		let xpos = self.m_sprite.m_location.x as i32 - len/2 - _cam_pos.x();
		let ypos = self.m_sprite.m_location.y as i32 - _cam_pos.y() - max_size - 20;
		_renderer.set_draw_color(pixels::Color::RGB(10,10,10));
		_renderer.fill_rect(Rect::new(xpos, ypos, len as u32, 10).unwrap().unwrap());
		let len = (self.m_life / 2.0) as i32;
		_renderer.set_draw_color(pixels::Color::RGB(100,255,100));
		_renderer.fill_rect(Rect::new(xpos, ypos, len as u32, 10).unwrap().unwrap());
		
		(&self.m_sprite as &drawable::Drawable).draw(_renderer, _cam_pos);
	}
}