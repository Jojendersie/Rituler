extern crate sdl2;
extern crate sdl2_image;

use sdl2_image::LoadTexture;
use std::path::Path;

use math;

pub trait Drawable {
    fn draw(&self, _renderer : &mut sdl2::render::Renderer );
}

pub struct Actor
{
	pub m_location : math::Vector,
	pub m_texture : sdl2::render::Texture,
}

impl Drawable for Actor
{
	fn draw(&self, _renderer : &mut sdl2::render::Renderer){
		_renderer.copy(&self.m_texture, None, None);
	}
}

