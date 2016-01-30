extern crate sdl2;
extern crate sdl2_image;

use actor;

pub struct World
{
	pub m_groundTiles : Vec< actor::Actor >,
	pub m_gameObjects : Vec< actor::Actor >,
}

impl actor::Drawable for World
{
	fn draw(&self, _renderer : &mut sdl2::render::Renderer){
		for act in &self.m_groundTiles{
			(act as &actor::Drawable).draw(_renderer);
		}
		
		for act in &self.m_gameObjects{
			(act as &actor::Drawable).draw(_renderer);
		}
	}
}

impl World{
	pub fn addActor(&mut self, _actor : actor::Actor){
		self.m_gameObjects.push(_actor);
	}
}