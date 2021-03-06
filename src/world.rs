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
use building;

pub struct World<'a>
{
	pub m_player : player::Player<'a>,
	pub m_ground_tiles : Vec< drawable::Sprite<'a> >,
	pub m_ground_tile_ids : Vec< i32 >,
	pub m_game_objects : Vec< actor::Actor<'a> >,
	pub m_projectiles : Vec< projectile::Projectile<'a> >,
	pub m_orbs : Vec< orb::Orb<'a> >,
	pub m_textures : Vec< &'a sdl2::render::Texture >,
	pub m_buildings : Vec< Option<building::Building<'a>> >,
	pub m_controllers : Vec< controller::Controller >,
	pub m_spawners : Vec< spawner::Spawner<'a> >,
	pub m_screen_shake_ampl : f32,
}

impl <'a> drawable::Drawable for World<'a>
{
	fn draw(&self, _renderer : &mut sdl2::render::Renderer, _cam_pos : &Point){
		let shake_cam = Point::new(
			_cam_pos.x() + (math::get_rand(30) as f32 * self.m_screen_shake_ampl) as i32,
			_cam_pos.y() + (math::get_rand(30) as f32 * self.m_screen_shake_ampl) as i32);
	
		for act in &self.m_ground_tiles{
			(act as &drawable::Drawable).draw(_renderer, &shake_cam);
		}
		
		for building in &self.m_buildings {
			if let Some(b) = building.as_ref() {
				b.draw(_renderer, &shake_cam);
			}
		}
		
		for proj in &self.m_projectiles{
			(proj as &drawable::Drawable).draw(_renderer, &shake_cam);
		}
		
		for orb in &self.m_orbs {
			(orb as &drawable::Drawable).draw(_renderer, &shake_cam);
		}
		
		for act in &self.m_game_objects{
			(act as &drawable::Drawable).draw(_renderer, &shake_cam);
		}
		
		(&self.m_player as &drawable::Drawable).draw(_renderer, &shake_cam);
	}
}

impl <'a> actor::Dynamic for World<'a>
{
	fn process(&mut self)
	{
		// Damp the screenshake
//		if self.m_player.m_actor.m_life <= 0.0 {
//			self.m_screen_shake_ampl = 5.0;
//		} else {
			self.m_screen_shake_ampl *= 0.95;
//		}

		for act in &mut self.m_game_objects{
			act.m_cool_down -= 0.016667;
			
			if act.m_wants_to_attack && act.m_cool_down <= 0.0 {
				act.m_cool_down = act.m_cool_down_max;
				let mut dir = math::Vector{x:-1.0, y: 0.0};
				dir.rotate(act.m_sprite.m_angle, 90.0);
				let loc = act.m_sprite.m_location + dir * (act.m_sprite.m_sprite_size.0 as f32) * 0.46;
				self.m_projectiles.push(act.m_projectile_builder.create_projectile(loc, dir));
			}
		}
		
		
		//player is handled extra
		{
			let player_act = &mut self.m_player.m_actor;
			player_act.m_cool_down -= 0.016667;
				
			if player_act.m_wants_to_attack && player_act.m_cool_down <= 0.0 {
				player_act.m_cool_down = player_act.m_cool_down_max;
				let mut dir = math::Vector{x:1.0, y: 0.0};
				dir.rotate(player_act.m_sprite.m_angle, 45.0);
				self.m_projectiles.push(player_act.m_projectile_builder.create_projectile(player_act.m_sprite.m_location + dir * 80.0, dir));
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
		for proj in &mut self.m_projectiles {
			for act in &mut self.m_game_objects {
				if (act.m_sprite.m_location - proj.m_sprite.m_location).len() < 0.45 * (act.m_sprite.m_sprite_size.0 as f32){
					proj.m_is_finished = true;
					act.m_life -= proj.m_damage;
					if act.m_life <= 0.0 {
						let id: i32 = 2 / (1 + math::get_rand(4));
						let tex = &self.m_textures[(id + TEX_SOUL0 as i32) as usize];
						self.m_orbs.push(orb::Orb::new(act.m_sprite.m_location, tex, id));
					}
				}
			}
			//player with smaller collision box
			if (self.m_player.m_actor.m_sprite.m_location - proj.m_sprite.m_location).len() < 0.35 * (self.m_player.m_actor.m_sprite.m_sprite_size.0 as f32){
				self.m_screen_shake_ampl = 0.5;
				proj.m_is_finished = true;
				self.m_player.m_actor.m_life -= proj.m_damage;
				if self.m_player.m_actor.m_life <= 0.0 {
					println!("Game Over!");
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

		let mut enemyInd = 3527389;
		
		//search a suitable opponent
		for i in 0..self.m_game_objects.len(){
			let dist_to_player = (self.m_player.m_actor.m_sprite.m_location - self.m_game_objects[i].m_sprite.m_location).len();
			if dist_to_player < 1000.0 && self.m_game_objects[i].m_is_hostile{
				enemyInd = i;
				break;
			}
		}
		
		for i in 0..self.m_game_objects.len(){
			if(self.m_game_objects[i].m_is_hostile){
				self.m_controllers[0].think(&mut self.m_game_objects[i], &self.m_player.m_actor);
			} else {
				if enemyInd != 3527389
				{
					if(i < enemyInd){
						let (a,b) = self.m_game_objects.split_at_mut(enemyInd);
						self.m_controllers[1].think(&mut (a[i]), &mut(b[0]));
					} else{
						let (a,b) = self.m_game_objects.split_at_mut(i);
						self.m_controllers[1].think(&mut (b[0]), &mut(a[enemyInd]));
					
					}
				}
			}
		}
		
		// spawn a golem when a building is finished
		for i in 0..self.m_buildings.len() {
			let mut set_to_none = false;
			if let Some(b) = self.m_buildings[i].as_ref() {
				if b.is_completed(){
					self.m_game_objects.push(actor::Actor::new_h( b.m_sprite.m_location, &self.m_spawners[0].m_actor_builder.m_texture, 50.0, &self.m_spawners[0].m_actor_builder.m_projectile_builder, 3.0, false));
					set_to_none = true;
				}
			}
			if set_to_none {
				self.m_buildings[i] = None;
			}
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
	
	
	//constructs a world with the given ground textures
	pub fn new(mut _textures : Vec < &'a sdl2::render::Texture >, _player : player::Player<'a>) -> World<'a> {
		let mut ground_tiles = Vec::new();
		let mut ground_tile_ids = Vec::new();
		let mut buildings = Vec::new();
		for x in 0..MAP_NUM_TILES_X {
			for y in 0..MAP_NUM_TILES_Y {
				let id: i32 = math::get_rand(1);
				ground_tiles.push(drawable::Sprite::new( math::Vector{x : (x as f32) * 350.0, y : (y as f32) * 350.0}, _textures[(id + TEX_FIRST_GROUND) as usize]));
				ground_tile_ids.push(id);
				buildings.push(None);
			}
		}
		
		World {
			m_player: _player,
			m_ground_tiles: ground_tiles,
			m_ground_tile_ids: ground_tile_ids,
			m_game_objects: Vec::new(),
			m_textures: _textures,
			m_projectiles: Vec::new(),
			m_orbs: Vec::new(),
			m_buildings: buildings,
			m_spawners : Vec::new(),
			m_controllers : Vec::new(),
			m_screen_shake_ampl : 0.0,
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
	
	pub fn add_building(&mut self, _pos : math::Vector) {
		let pos = math::Vector{x: f32::floor((_pos.x + 175.0) / 350.0) * 350.0,
							   y: f32::floor((_pos.y + 175.0) / 350.0) * 350.0};
		let build = building::Building::new(pos, &self.m_textures[TEX_GOLEM_ALTAR], [4,2,1],
											&vec![&self.m_textures[TEX_SOUL0], &self.m_textures[TEX_SOUL1], &self.m_textures[TEX_SOUL2]],
											&vec![&self.m_textures[TEX_SOUL0_R], &self.m_textures[TEX_SOUL1_R], &self.m_textures[TEX_SOUL2_R]]);
		let x = f32::floor((pos.x + 175.0) / 350.0) as i32;
		let y = f32::floor((pos.y + 175.0) / 350.0) as i32;
		if x >= 0 && y >= 0 && x < MAP_NUM_TILES_X && y < MAP_NUM_TILES_Y {
			let index = (x * MAP_NUM_TILES_Y + y) as usize;
			self.m_buildings[index] = Some(build);
		}
		
		self.m_screen_shake_ampl = 2.0;
	}
	
	pub fn get_building(&mut self, _pos : math::Vector) -> Option<&mut building::Building<'a>> {
		let x = f32::floor((_pos.x + 175.0) / 350.0) as i32;
		let y = f32::floor((_pos.y + 175.0) / 350.0) as i32;
		if x >= 0 && y >= 0 && x < MAP_NUM_TILES_X && y < MAP_NUM_TILES_Y {
			self.m_buildings[(x * MAP_NUM_TILES_Y + y) as usize].as_mut()
		} else {
			None
		}
	}
}