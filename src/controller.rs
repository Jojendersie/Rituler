use math;
use actor;
use player;
use constants::*;
use std::f32;

pub struct Controller
{
	pub m_speed : f32, // movement speed
	pub m_prefered_dist : f32,
}

impl Controller{
	pub fn think(&mut self, _actor: &mut actor::Actor, _target : & actor::Actor){
		let mut dir = _actor.m_sprite.m_location - _target.m_sprite.m_location;
		let len = dir.len();
		dir.normalize();
		
		if len > self.m_prefered_dist {
			_actor.m_sprite.m_location = _actor.m_sprite.m_location - dir * self.m_speed;
		}
		_actor.m_sprite.m_angle = f32::atan2(dir.y, dir.x) * 180.0 / f32::consts::PI + 90.0;
		
		_actor.m_wants_to_attack = true;
		
	//	let vec = self.m_actor.m_sprite.m_location + dir * self.m_speed;
	//	self.m_actor.m_sprite.m_location.x = vec.x;
	//	self.m_actor.m_sprite.m_location.x = vec.y;
	}
}

