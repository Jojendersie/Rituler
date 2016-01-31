extern crate sdl2;

use sdl2::rect::{Point, Rect};
use sdl2::pixels;

use math;
use drawable;
use actor;

//structure that holds general projectile information to spawn projectiles of the same type
pub struct ProjectileBuilder<'a>{
	pub m_texture : &'a sdl2::render::Texture,
	pub m_speed : f32,
	pub m_damage : f32,
}

impl <'a> ProjectileBuilder<'a>{
	pub fn create_projectile(&self, _loc : math::Vector, mut _dir : math::Vector) -> Projectile{
		_dir.normalize();
		Projectile::new(_loc, self.m_texture, _dir * self.m_speed, self.m_damage)
	}
}


//the actual projectiles
pub struct Projectile<'a> {
	pub m_sprite : drawable::Sprite<'a>,
	pub m_velocity : math::Vector,
	pub m_is_finished : bool,
	pub m_damage : f32,
	pub m_life_time : i32,
}

impl <'a> drawable::Drawable for Projectile<'a>
{
	fn draw(&self, _renderer : &mut sdl2::render::Renderer, _cam_pos : &Point) {
		// Draw a life bar
		
		let offset = Point::new(self.m_velocity.x as i32, self.m_velocity.y as i32);
		let baseLoc = Point::new((self.m_sprite.m_location.x as i32) - _cam_pos.x() - offset.x()*5, (self.m_sprite.m_location.y as i32) - _cam_pos.y() - offset.y()*5 );
		
		_renderer.set_draw_color(pixels::Color::RGB(95,54,228));
		
		let max = if self.m_life_time < 48 {
			self.m_life_time / 2
		} else{
			24
		};
		for i in 0..max{
			let randOff = i + 40;
			let i_2 = randOff / 2; 
			let point = Point::new(baseLoc.x() - i * offset.x() + math::get_rand(randOff as u32) - (i_2) - 2, baseLoc.y() - i * offset.y() + math::get_rand(randOff as u32) - i_2 - 2);
		//	_renderer.draw_point();
			_renderer.fill_rect(Rect::new(point.x(), point.y(), 4, 4).unwrap().unwrap());
		}
		
		//draw the sprite last
		(&self.m_sprite as &drawable::Drawable).draw(_renderer, _cam_pos);
	}
}

//constructor
impl<'a> Projectile<'a> {
	pub fn new(_vec: math::Vector, _texture: &sdl2::render::Texture, _vel : math::Vector, _damage : f32) -> Projectile {
		Projectile{
			m_sprite: drawable::Sprite::new(_vec, _texture),
			m_velocity: _vel,
			m_is_finished : false,
			m_damage : _damage,
			m_life_time : 0,
		}
	}
}

impl <'a> actor::Dynamic for Projectile<'a>
{
	fn process(&mut self)
	{
		self.m_sprite.m_location = self.m_sprite.m_location + self.m_velocity;
		self.m_life_time = self.m_life_time + 1;
	}
}