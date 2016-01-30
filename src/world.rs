extern crate sdl2;
extern crate sdl2_image;

use drawable;
use actor;
use math;
use sdl2::rect::{Point};

pub struct World<'a>
{
	pub m_ground_tiles : Vec< drawable::Sprite<'a> >,
	pub m_game_objects : Vec< actor::Actor<'a> >,
}

impl <'a> drawable::Drawable for World<'a>
{
	fn draw(&self, _renderer : &mut sdl2::render::Renderer, _cam_pos : &Point){
		for act in &self.m_ground_tiles{
			(act as &drawable::Drawable).draw(_renderer, &_cam_pos);
		}
		
		for act in &self.m_game_objects{
			(&act.m_sprite as &drawable::Drawable).draw(_renderer, &_cam_pos);
		}
	}
}

impl<'a> World<'a>{
	pub fn add_actor(&mut self, _actor : actor::Actor<'a>){
		self.m_game_objects.push(_actor);
	}
	//constructs a world with the given ground textures
	pub fn new(_ground_textures : Vec < &sdl2::render::Texture >) -> World {
		let mut ground_tiles = Vec::new();
		
//		let rng = rand::XorShiftRng::new_unseeded();
		for x in 0..8{
			for y in 0..8{
				if math::get_rand(1) == 1 {
					ground_tiles.push(drawable::Sprite::new( math::Vector{x : (x as f32) * 350.0, y : (y as f32) * 350.0}, _ground_textures[0]));
				}else{
					ground_tiles.push(drawable::Sprite::new( math::Vector{x : (x as f32) * 350.0, y : (y as f32) * 350.0}, _ground_textures[1]));
				}
			}
		}
		
		World {
			m_ground_tiles: ground_tiles,
			m_game_objects: Vec::new(),
		}
	}
}