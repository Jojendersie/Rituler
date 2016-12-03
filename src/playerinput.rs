extern crate sdl2;

use std::f32;
use sdl2::keyboard::Scancode;
use math;
use world;
use constants::*;
use std::cmp::min;

pub fn handle_player_input(_sdl_context: &sdl2::Sdl, _keyboard: &sdl2::keyboard::KeyboardState, _world: &mut world::World, _left_mouse_down: bool) {
	let mut constructing = false;
	let player_pos = _world.m_player.m_actor.m_sprite.m_location;
	// If the player is on a sand-ground tile and presses SPACE build a golem altar
	if _keyboard.is_scancode_pressed(Scancode::Space) {
		// Check if the underlying map type is correct
		let tile_id = _world.get_tile(player_pos);
		let mut no_building = true;
		let mut inventory = [0; 3];
		for o in 0..3 {
			inventory[o] = _world.m_player.m_inventory[o];
		}
		{
			if let Some(ref b) = world.get_building(player_pos) {
				for o in 0..3 {
					let num_souls = b.m_req_resources[o] - b.m_resources[o];
					b.m_resources[o] += min(inventory[o], num_souls);
					inventory[o] -= min(inventory[o], num_souls);
				}
				no_building = false;
			}
		}
		for o in 0..3 {
			_world.m_player.m_inventory[o] = inventory[o];
		}
		if tile_id == 1 && no_building {
			constructing = true;
			_world.m_player.m_construction_progress += 0.01;
			if _world.m_player.m_construction_progress >= 1.0 {
				_world.add_building(player_pos);
				_world.m_player.m_construction_progress = 0.0;
			}
		}
	} else {
		_world.m_player.m_construction_progress = 0.0;
	}
	
	if !constructing {
		let player = &mut _world.m_player;
		// The player is always in the middle of the screen -> rotation depends only on window setting
		let (mouse_state, mx, my) = _sdl_context.mouse().mouse_state();
		player.m_actor.m_sprite.m_angle = f32::atan2((my - WIN_HEIGHT/2) as f32,
									 (mx - WIN_WIDTH/2) as f32) * 180.0 / f32::consts::PI + 45.0;
		
		player.m_actor.m_wants_to_attack = _left_mouse_down || mouse_state.left();
	//	if mouse_state.left() {println!("alah uhagbar");};
		
		let mut move_dir = math::Vector{x:0.0, y:0.0};
		if _keyboard.is_scancode_pressed(Scancode::W) {
			move_dir.y -= 1.0;
		}
		if _keyboard.is_scancode_pressed(Scancode::S) {
			move_dir.y += 1.0;
		}
		if _keyboard.is_scancode_pressed(Scancode::D) {
			move_dir.x += 1.0;
		}
		if _keyboard.is_scancode_pressed(Scancode::A) {
			move_dir.x -= 1.0;
		}
		move_dir.normalize();
		
		player.m_actor.m_sprite.m_location = player_pos + 5.0 * move_dir;
	}
}