extern crate sdl2;
extern crate sdl2_image;

use sdl2::rect::{Rect, Point};

use math;

pub trait Drawable {
    fn draw(&self, _renderer : &mut sdl2::render::Renderer, _cam_pos : &Point);
}

pub struct Actor<'a>
{
	pub m_location : math::Vector,
	pub m_texture : &'a sdl2::render::Texture,
	pub m_angle : f32,
	pub m_sprite_size : (u32, u32),
}

impl <'a> Drawable for Actor<'a>
{
	fn draw(&self, _renderer : &mut sdl2::render::Renderer, _cam_pos : &Point) {
		let hsize_x = (self.m_sprite_size.0/2) as i32;
		let hsize_y = (self.m_sprite_size.1/2) as i32;
		_renderer.copy_ex(&self.m_texture, None, Rect::new(self.m_location.x as i32 - _cam_pos.x() - hsize_x, self.m_location.y as i32 - _cam_pos.y() - hsize_y, self.m_sprite_size.0, self.m_sprite_size.1).unwrap(),
						  self.m_angle as f64, Some(Point::new(hsize_x, hsize_y)), (false,false))
	}
}

impl<'a> Actor<'a> {
	pub fn new(_vec: math::Vector, _texture: &sdl2::render::Texture) -> Actor {
		let tex_query = _texture.query();
		Actor {
			m_location: _vec,
			m_texture: _texture,
			m_angle: 0.0,
			m_sprite_size: (tex_query.width, tex_query.height)
		}
	}
}