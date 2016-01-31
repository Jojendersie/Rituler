extern crate sdl2;

use math;
use drawable;
use actor;
use sdl2::rect::{Point, Rect};
use constants::*;
use sdl2::pixels;
use projectile;

pub struct Player<'a> {
	pub m_actor : actor::Actor<'a>,
	pub m_orb_sprites : Vec< drawable::Sprite<'a> >,
	pub m_construction_progress : f32,
	pub m_inventory: [i32; 3], // A counter for every collectable: currently only orbs.
}

impl<'a> Player<'a> {
	pub fn new(_vec: math::Vector, _texture: &'a sdl2::render::Texture, _proj_builder : &'a projectile::ProjectileBuilder<'a>, mut _orb_textures : Vec < &'a sdl2::render::Texture >) -> Player<'a> {
		let mut orb_sprites = Vec::new();
		for id in 0.._orb_textures.len() {
			let mut sprite = drawable::Sprite::new( math::Vector{x:0.0, y:0.0}, _orb_textures[id as usize]);
			sprite.m_sprite_size.0 /= 3;
			sprite.m_sprite_size.1 /= 3;
			orb_sprites.push(sprite);
		}
		
		Player {
			m_actor: actor::Actor::new(_vec, _texture, 200.0, _proj_builder, 0.75),
			m_orb_sprites: orb_sprites,
			m_construction_progress: 0.0,
			m_inventory: [0; 3],
		}
	}
}

impl <'a> drawable::Drawable for Player<'a>
{
	fn draw(&self, _renderer : &mut sdl2::render::Renderer, _cam_pos : &Point) {
		// Draw inventory
		for o in 0..3 {
			for i in 1..(self.m_inventory[o]+1) {
				let cam = Point::new(-30 - (o as i32) * 45, -WIN_HEIGHT + i * 50);
				self.m_orb_sprites[o].draw(_renderer, &cam);
			}
		}
		
		(&self.m_actor as &drawable::Drawable).draw(_renderer, _cam_pos);
		
		// While constructing stuff show a bar
		if self.m_construction_progress > 0.0 {
			_renderer.set_draw_color(pixels::Color::RGB(10,10,10));
			_renderer.draw_rect(Rect::new(50, WIN_HEIGHT - 75, WIN_WIDTH as u32 - 100, 25).unwrap().unwrap());
			let len = ((WIN_WIDTH - 102) as f32 * self.m_construction_progress) as u32;
			_renderer.set_draw_color(pixels::Color::RGB(50,50,200));
			_renderer.fill_rect(Rect::new(51, WIN_HEIGHT - 74, len, 23).unwrap().unwrap());
		}
	}
}