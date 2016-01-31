use math;
use actor;
use player;


pub struct Controller
{
//	pub m_actor : &'a actor::Actor<'a>,
//	pub m_player : &'a player::Player<'a>,
	pub m_speed : f32, // movement speed
}

impl Controller{
	pub fn think(&mut self, _actor: &mut actor::Actor, _player : & player::Player){
		let mut dir = _actor.m_sprite.m_location - _player.m_actor.m_sprite.m_location;
		dir.normalize();
		
		_actor.m_sprite.m_location = _actor.m_sprite.m_location - dir * self.m_speed;
		
	//	let vec = self.m_actor.m_sprite.m_location + dir * self.m_speed;
	//	self.m_actor.m_sprite.m_location.x = vec.x;
	//	self.m_actor.m_sprite.m_location.x = vec.y;
	}
}

