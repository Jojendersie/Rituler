extern crate sdl2;
extern crate sdl2_image;

use actor;
use math;

pub struct World<'a>
{
	pub m_ground_tiles : Vec< actor::Actor<'a> >,
	pub m_game_objects : Vec< actor::Actor<'a> >,
}

impl <'a> actor::Drawable for World<'a>
{
	fn draw(&self, _renderer : &mut sdl2::render::Renderer){
		for act in &self.m_ground_tiles{
			(act as &actor::Drawable).draw(_renderer);
		}
		
		for act in &self.m_game_objects{
			(act as &actor::Drawable).draw(_renderer);
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
		//		println!("{}", (&rng as &rand::Rng).next_u32() % 2 == 0);
		//		if(rng.next_u32() % 2 == 0){
					ground_tiles.push(actor::Actor::new( math::Vector{x : (x as f32) * 350.0, y : (y as f32) * 350.0}, _ground_textures[0]));
		//		}else{
		//			groundTiles.push(actor::Actor::new( math::Vector{x : (x as f32) * 350.0, y : (x as f32) * 350.0}, _groundTextures[1]));
		//		}
			}
		}
		
		World {
			m_ground_tiles: ground_tiles,
			m_game_objects: Vec::new(),
		}
	}
}