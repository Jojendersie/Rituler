use math;
use actor;
use player;
use constants::*;
use std::f32;

pub struct Controller
{
//	pub m_actor : &'a actor::Actor<'a>,
//	pub m_player : &'a player::Player<'a>,
	pub m_speed : f32, // movement speed
}

impl Controller{
	pub fn think(&mut self, _actor: &mut actor::Actor, _player : & player::Player){
		let mut dir = _actor.m_sprite.m_location - _player.m_actor.m_sprite.m_location;
		let len = dir.len();
		dir.normalize();
		
		if(len > 144.0){
			_actor.m_sprite.m_location = _actor.m_sprite.m_location - dir * self.m_speed;
		}
		_actor.m_sprite.m_angle = f32::atan2(dir.y, dir.x) * 180.0 / f32::consts::PI + 90.0;
		
		_actor.m_wants_to_attack = true;
		
	//	let vec = self.m_actor.m_sprite.m_location + dir * self.m_speed;
	//	self.m_actor.m_sprite.m_location.x = vec.x;
	//	self.m_actor.m_sprite.m_location.x = vec.y;
	}
}

