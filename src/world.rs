extern crate sdl2;
extern crate sdl2_image;

use drawable;
use actor;
use math;
use projectile;
use sdl2::rect::{Point};
use constants::*;
use player;
use orb;
use controller;
use spawner;

pub struct World<'a>
{
	pub m_player : player::Player<'a>,
	pub m_ground_tiles : Vec< drawable::Sprite<'a> >,
	pub m_ground_tile_ids : Vec< i32 >,
	pub m_game_objects : Vec< actor::Actor<'a> >,
	pub m_ground_textures : Vec < &'a sdl2::render::Texture >,
	pub m_projectiles : Vec< projectile::Projectile<'a> >,
	pub m_orbs : Vec< orb::Orb<'a> >,
	pub m_orb_textures : Vec< &'a sdl2::render::Texture >,
	pub m_controllers : Vec< controller::Controller >,
	pub m_spawners : Vec< spawner::Spawner<'a> >,
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
		
		for orb in &self.m_orbs {
			(orb as &drawable::Drawable).draw(_renderer, &_cam_pos);
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
			act.m_cool_down -= 0.016667;
			
			if act.m_wants_to_attack && act.m_cool_down <= 0.0 {
				act.m_cool_down = act.m_cool_down_max;
				self.m_projectiles.push(act.m_projectile_builder.create_projectile(act.m_sprite.m_location, math::Vector{x: 10.0, y: 10.0}));
			}
		}
		
		
		//player is handled extra
		{
			let player_act = &mut self.m_player.m_actor;
			player_act.m_cool_down -= 0.016667;
				
			if player_act.m_wants_to_attack && player_act.m_cool_down <= 0.0 {
				player_act.m_cool_down = player_act.m_cool_down_max;
				let mut dir = math::Vector{x:1.0, y: 0.0};
				dir.rotate(player_act.m_sprite.m_angle);
				self.m_projectiles.push(player_act.m_projectile_builder.create_projectile(player_act.m_sprite.m_location, dir));
			}
		}
		
		//process projectiles
		for proj in &mut self.m_projectiles{
			(proj as &mut actor::Dynamic).process();
		}
		
		// Process orbs
		for orb in &mut self.m_orbs {
			(orb as &mut actor::Dynamic).process();
			// Check collision with player
			if (self.m_player.m_actor.m_sprite.m_location - orb.m_sprite.m_location).len() < 0.45 * (self.m_player.m_actor.m_sprite.m_sprite_size.0 as f32) {
				self.m_player.m_inventory[orb.m_quality as usize] += 1;
				orb.m_quality = -1;
			}
		}
		
		//collision of projectiles
		for act in &mut self.m_game_objects {
			for proj in &mut self.m_projectiles {
				if (act.m_sprite.m_location - proj.m_sprite.m_location).len() < 0.45 * (act.m_sprite.m_sprite_size.0 as f32){
					proj.m_is_finished = true;
					act.m_life -= proj.m_damage;
					if act.m_life <= 0.0 {
						let id: i32 = 2 / (1 + math::get_rand(6));
						self.m_orbs.push(orb::Orb::new(act.m_sprite.m_location, self.m_orb_textures[id as usize], id));
					}
				}
			}
		}

		// spawner
		for spawner in &mut self.m_spawners{
			spawner.process();

			if spawner.m_wants_to_spawn{
				self.m_game_objects.push(actor::Actor::new( spawner.m_location, &spawner.m_actor_builder.m_texture, 50.0, &spawner.m_actor_builder.m_projectile_builder, 1.0));
				spawner.m_wants_to_spawn = false;
			//	self.
			}
		}
		
		for act in &mut self.m_game_objects{
			self.m_controllers[0].think(act, &self.m_player);
		}
		
		//remove all finished objects in the world
		self.m_projectiles.retain(|x| !x.m_is_finished);
		self.m_orbs.retain(|x| x.m_quality >= 0);
		self.m_game_objects.retain(|x| x.m_life > 0.0);
	}
}

impl<'a> World<'a>{
	/*pub fn set_player(&mut self, _player : player::Player<'a>){
		self.m_player = _player;
	}*/
	
	pub fn add_actor(&mut self, _actor : actor::Actor<'a>){
		self.m_game_objects.push(_actor);
	}
	
	/*pub fn spawn_projectile(&mut self, _projectile: projectile::Projectile<'a>){
		self.m_projectiles.push(_projectile);
	}*/
	
	
	//constructs a world with the given ground textures
	pub fn new(mut _ground_textures : Vec < &'a sdl2::render::Texture >, mut _orb_textures : Vec < &'a sdl2::render::Texture >, _player : player::Player<'a>) -> World<'a> {
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
			m_orbs: Vec::new(),
			m_orb_textures: _orb_textures,
			m_spawners : Vec::new(),
			m_controllers : Vec::new(),
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