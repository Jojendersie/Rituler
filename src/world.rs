extern crate sdl2;
extern crate sdl2_image;

use actor;
use math;
use sdl2::rect::{Point};

pub struct World<'a>
{
	pub m_groundTiles : Vec< actor::Actor<'a> >,
	pub m_gameObjects : Vec< actor::Actor<'a> >,
}

impl <'a> actor::Drawable for World<'a>
{
	fn draw(&self, _renderer : &mut sdl2::render::Renderer, _cam_pos : &Point){
		for act in &self.m_groundTiles{
			(act as &actor::Drawable).draw(_renderer, &_cam_pos);
		}
		
		for act in &self.m_gameObjects{
			(act as &actor::Drawable).draw(_renderer, &_cam_pos);
		}
	}
}

impl<'a> World<'a>{
	pub fn addActor(&mut self, _actor : actor::Actor<'a>){
		self.m_gameObjects.push(_actor);
	}
	//constructs a world with the given ground textures
	pub fn new(_groundTextures : Vec < &sdl2::render::Texture >) -> World {
		let mut groundTiles = Vec::new();
		
//		let rng = rand::XorShiftRng::new_unseeded();
		for x in 0..8{
			for y in 0..8{
		//		println!("{}", (&rng as &rand::Rng).next_u32() % 2 == 0);
		//		if(rng.next_u32() % 2 == 0){
					groundTiles.push(actor::Actor::new( math::Vector{x : (x as f32) * 350.0, y : (y as f32) * 350.0}, _groundTextures[0]));
		//		}else{
		//			groundTiles.push(actor::Actor::new( math::Vector{x : (x as f32) * 350.0, y : (x as f32) * 350.0}, _groundTextures[1]));
		//		}
			}
		}
		
		World {
			m_groundTiles: groundTiles,
			m_gameObjects: Vec::new(),
		}
	}
}