extern crate sdl2;

use math;
use drawable;
use actor;
use sdl2::rect::{Point};

pub struct Player<'a> {
	pub m_actor : actor::Actor<'a>,
	pub m_construction_progress : f32,
}

impl<'a> Player<'a> {
	pub fn new(_vec: math::Vector, _texture: &sdl2::render::Texture) -> Player {
		Player {
			m_actor: actor::Actor::new(_vec, _texture, 200.0),
			m_construction_progress: 0.0,
		}
	}
}

impl <'a> drawable::Drawable for Player<'a>
{
	fn draw(&self, _renderer : &mut sdl2::render::Renderer, _cam_pos : &Point) {
		// Draw a life bar
/*		let max_size = max(self.m_sprite.m_sprite_size.0/2, self.m_sprite.m_sprite_size.1/2) as i32;
		let len = (self.m_max_life / 2.0) as i32;
		let xpos = self.m_sprite.m_location.x as i32 - len/2 - _cam_pos.x();
		let ypos = self.m_sprite.m_location.y as i32 - _cam_pos.y() - max_size - 20;
		_renderer.set_draw_color(pixels::Color::RGB(10,10,10));
		_renderer.fill_rect(Rect::new(xpos, ypos, len as u32, 10).unwrap().unwrap());
		let len = (self.m_life / 2.0) as i32;
		_renderer.set_draw_color(pixels::Color::RGB(100,255,100));
		_renderer.fill_rect(Rect::new(xpos, ypos, len as u32, 10).unwrap().unwrap());*/
		
		(&self.m_actor as &drawable::Drawable).draw(_renderer, _cam_pos);
	}
}