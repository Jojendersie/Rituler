extern crate sdl2;

use actor;
use math;
use projectile;

// a actor factory
pub struct ActorBuilder<'a>{
	pub m_texture : &'a sdl2::render::Texture,
	pub m_max_life : f32,
	pub m_cool_down_max : f32,
	pub m_projectile_builder : &'a projectile::ProjectileBuilder<'a>,
}

impl<'a> ActorBuilder<'a> {
	pub fn new(_texture: &'a sdl2::render::Texture, _max_life: f32, _proj_builder: &'a projectile::ProjectileBuilder, _cool_down: f32) -> ActorBuilder<'a>{
		ActorBuilder{
			m_texture: _texture,
			m_max_life: _max_life,
			m_cool_down_max: _cool_down,
			m_projectile_builder: _proj_builder,
		}
	}
	
	pub fn create_actor(&self, _vec: math::Vector) -> actor::Actor{
		actor::Actor::new(_vec, self.m_texture, self.m_max_life, self.m_projectile_builder, self.m_cool_down_max)
	}
}

// a object in the world that spawns one type of monsters
pub struct Spawner<'a>
{
	pub m_actor_builder : ActorBuilder<'a>,
	pub m_location : math::Vector,
	pub m_cool_down : i32,
	pub m_cool_down_max : i32,
	pub m_wants_to_spawn : bool,
}

impl<'a> Spawner<'a> {
/*	pub fn spawn (&mut self, _player : &player::Player) -> (actor::Actor, controller::Controller){
		self.m_wants_to_spawn = false;
		let act = self.m_actor_builder.create_actor(self.m_location);
		(act, controller::Controller{m_actor : &act, m_player : _player, m_speed : 2.0})
	}*/
}

impl <'a> actor::Dynamic for Spawner<'a>
{
	fn process(&mut self)
	{
		self.m_cool_down = self.m_cool_down - 1;
		
		if self.m_cool_down <= 0{
			self.m_wants_to_spawn = true;
			self.m_cool_down = self.m_cool_down_max;
		}
	}
}