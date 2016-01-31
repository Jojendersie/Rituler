extern crate sdl2;

use sdl2::rect::{Point};

use math;
use drawable;

pub struct Building<'a> {
	pub m_sprite : drawable::Sprite<'a>,
	pub m_orb_sprites : Vec< drawable::Sprite<'a> >,
	pub m_req_orb_sprites : Vec< drawable::Sprite<'a> >,
	pub m_resources : [i32; 3],
	pub m_req_resources : [i32; 3],
}

impl <'a> drawable::Drawable for Building<'a>
{
	fn draw(&self, _renderer : &mut sdl2::render::Renderer, _cam_pos : &Point) {
		self.m_sprite.draw(_renderer, _cam_pos);
		let mut n = 0;
		for o in 0..3 {
			for _ in 0..self.m_resources[o] {
				self.m_orb_sprites[n].draw(_renderer, _cam_pos);
				n += 1;
			}
			for _ in self.m_resources[o]..self.m_req_resources[o] {
				self.m_req_orb_sprites[n].draw(_renderer, _cam_pos);
				n += 1;
			}
		}
	}
}

// Constructor
impl<'a> Building<'a> {
	pub fn new(_vec: math::Vector, _texture: &'a sdl2::render::Texture, _req_resources: [i32; 3],
				_orb_textures : &Vec< &'a sdl2::render::Texture >,
				_req_orb_textures : &Vec< &'a sdl2::render::Texture >) -> Building<'a> {
		let mut orb_sprites = Vec::new();
		let mut req_orb_sprites = Vec::new();
		for o in 0..3 {
			for i in 0.._req_resources[o] {
				let mut sprite = drawable::Sprite::new( math::Vector{x:_vec.x + o as f32 * 45.0 - 145.0, y:_vec.y - (i as f32)*50.0 + 140.0}, _orb_textures[o]);
				sprite.m_sprite_size.0 /= 3;
				sprite.m_sprite_size.1 /= 3;
				orb_sprites.push(sprite);
				req_orb_sprites.push( drawable::Sprite::new( math::Vector{x:_vec.x + o as f32 * 45.0 - 145.0, y:_vec.y - (i as f32)*50.0 + 140.0}, _req_orb_textures[o]) );
			}
		}
		
		Building {
			m_sprite : drawable::Sprite::new(_vec, _texture),
			m_orb_sprites : orb_sprites,
			m_req_orb_sprites : req_orb_sprites,
			m_resources : [2,1,3],
			m_req_resources : _req_resources,
		}
	}
}