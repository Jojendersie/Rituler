extern crate sdl2;

use sdl2::rect::{Point};

use math;
use drawable;
use actor;

//structure that holds general projectile information to spawn projectiles of the same type
pub struct ProjectileBuilder<'a>{
	pub m_texture : &'a sdl2::render::Texture,
	pub m_speed : f32,
	pub m_damage : i32,
}

impl <'a> ProjectileBuilder<'a>{
	pub fn create_projectile(&self, _loc : math::Vector, mut _dir : math::Vector) -> Projectile{
		_dir.normalize();
		Projectile::new(_loc, self.m_texture, _dir * self.m_speed)
	}
}


//the actual projectiles
pub struct Projectile<'a> {
	pub m_sprite : drawable::Sprite<'a>,
	pub m_velocity : math::Vector,
	pub m_is_finished : bool,
}

impl <'a> drawable::Drawable for Projectile<'a>
{
	fn draw(&self, _renderer : &mut sdl2::render::Renderer, _cam_pos : &Point) {
		// Draw a life bar
		
		(&self.m_sprite as &drawable::Drawable).draw(_renderer, _cam_pos);
	}
}

//constructor
impl<'a> Projectile<'a> {
	pub fn new(_vec: math::Vector, _texture: &sdl2::render::Texture, _vel : math::Vector) -> Projectile {
		Projectile{
			m_sprite: drawable::Sprite::new(_vec, _texture),
			m_velocity: _vel,
			m_is_finished : false,
		}
	}
}

impl <'a> actor::Dynamic for Projectile<'a>
{
	fn process(&mut self)
	{
		self.m_sprite.m_location = self.m_sprite.m_location + self.m_velocity;
	}
}