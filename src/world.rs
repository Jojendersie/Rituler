extern crate sdl2;
extern crate sdl2_image;

use drawable;
use actor;
use math;
use projectile;
use sdl2::rect::{Point};
use constants::*;
use player;

pub struct World<'a>
{
	pub m_player : player::Player<'a>,
	pub m_ground_tiles : Vec< drawable::Sprite<'a> >,
	pub m_ground_tile_ids : Vec< i32 >,
	pub m_game_objects : Vec< actor::Actor<'a> >,
	pub m_ground_textures : Vec < &'a sdl2::render::Texture >,
	pub m_projectiles : Vec< projectile::Projectile<'a> >,
}

impl <'a> drawable::Drawable for World<'a>
{
	fn draw(&self, _renderer : &mut sdl2::render::Renderer, _cam_pos : &Point){
		for act in &self.m_ground_tiles{
			(act as &drawable::Drawable).draw(_renderer, &_cam_pos);
		}
		
		for proj in &self.m_projectiles{
			(proj as &drawable::Drawable).draw(_renderer, &_cam_pos);
		}
		
		for act in &self.m_game_objects{
			(act as &drawable::Drawable).draw(_renderer, &_cam_pos);
		}
		
		(&self.m_player as &drawable::Drawable).draw(_renderer, &_cam_pos);
	}
}

impl <'a> actor::Dynamic for World<'a>
{
	fn process(&mut self)
	{
		for act in &mut self.m_game_objects{
			act.m_coolDown -= 0.016667;
			
			if(act.m_wantsToAttack && act.m_coolDown <= 0.0){
				act.m_coolDown = act.m_coolDownMax;
				self.m_projectiles.push(act.m_projectileBuilder.create_projectile(act.m_sprite.m_location, math::Vector{x: 10.0, y: 10.0}));
			}
		}
		
		
		//player is handled here
		let playerAct = &mut self.m_player.m_actor;
		playerAct.m_coolDown -= 0.016667;
			
		if(playerAct.m_wantsToAttack && playerAct.m_coolDown <= 0.0){
			playerAct.m_coolDown = playerAct.m_coolDownMax;
			let mut dir = math::Vector{x:1.0, y: 0.0};
			dir.rotate(playerAct.m_sprite.m_angle);
			self.m_projectiles.push(playerAct.m_projectileBuilder.create_projectile(playerAct.m_sprite.m_location, dir));
		}
	
		for proj in &mut self.m_projectiles{
			(proj as &mut actor::Dynamic).process();
		}
	}
}

impl<'a> World<'a>{
	/*pub fn set_player(&mut self, _player : player::Player<'a>){
		self.m_player = _player;
	}*/
	
	pub fn add_actor(&mut self, _actor : actor::Actor<'a>){
		self.m_game_objects.push(_actor);
	}
	
	pub fn spawn_projectile(&mut self, _projectile: projectile::Projectile<'a>){
		self.m_projectiles.push(_projectile);
	}
	
	
	//constructs a world with the given ground textures
	pub fn new(mut _ground_textures : Vec < &'a sdl2::render::Texture >, _player : player::Player<'a>) -> World<'a> {
		let mut ground_tiles = Vec::new();
		let mut ground_tile_ids = Vec::new();
		for x in 0..MAP_NUM_TILES_X {
			for y in 0..MAP_NUM_TILES_Y {
				let id: i32 = math::get_rand(1);
				ground_tiles.push(drawable::Sprite::new( math::Vector{x : (x as f32) * 350.0, y : (y as f32) * 350.0}, _ground_textures[id as usize]));
				ground_tile_ids.push(id);
			}
		}
		
		World {
			m_player: _player,
			m_ground_tiles: ground_tiles,
			m_ground_tile_ids: ground_tile_ids,
			m_game_objects: Vec::new(),
			m_ground_textures: _ground_textures,
			m_projectiles: Vec::new(),
		}
	}
	
	pub fn get_tile(&self, _pos : math::Vector) -> i32 {
		let x = f32::floor((_pos.x + 175.0) / 350.0) as i32;
		let y = f32::floor((_pos.y + 175.0) / 350.0) as i32;
		if x >= 0 && y >= 0 && x < MAP_NUM_TILES_X && y < MAP_NUM_TILES_Y {
			self.m_ground_tile_ids[(x * MAP_NUM_TILES_Y + y) as usize]
		} else {
			-1
		}
	}
	
	pub fn set_tile(&mut self, _pos : math::Vector, _tile : i32) {
		let x = f32::floor((_pos.x + 175.0) / 350.0) as i32;
		let y = f32::floor((_pos.y + 175.0) / 350.0) as i32;
		if x >= 0 && y >= 0 && x < MAP_NUM_TILES_X && y < MAP_NUM_TILES_Y {
			let index = (x * MAP_NUM_TILES_Y + y) as usize;
			self.m_ground_tile_ids[index] = _tile;
			self.m_ground_tiles[index] = drawable::Sprite::new( math::Vector{x : (x as f32) * 350.0, y : (y as f32) * 350.0}, self.m_ground_textures[_tile as usize]);
		}
	}
}