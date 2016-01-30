extern crate sdl2;
extern crate sdl2_image;

use sdl2::rect::{Rect, Point};

use math;

pub trait Drawable {
    fn draw(&self, _renderer : &mut sdl2::render::Renderer );
}

pub struct Actor
{
	pub m_location : math::Vector,
	pub m_texture : sdl2::render::Texture,
	pub m_angle : f32,
	pub m_sprite_size : (u32, u32),
}

impl Drawable for Actor
{
	fn draw(&self, _renderer : &mut sdl2::render::Renderer) {
		_renderer.copy_ex(&self.m_texture, None, Rect::new(100, 100, self.m_sprite_size.0, self.m_sprite_size.1).unwrap(),
						  self.m_angle as f64, Some(Point::new((self.m_sprite_size.0/2) as i32, (self.m_sprite_size.1/2) as i32)), (false,false))
	}
}

impl Actor {
	pub fn new(_vec: math::Vector, _texture: sdl2::render::Texture) -> Actor {
		let tex_query = _texture.query();
		Actor {
			m_location: _vec,
			m_texture: _texture,
			m_angle: 0.0,
			m_sprite_size: (tex_query.width, tex_query.height)
		}
	}
}