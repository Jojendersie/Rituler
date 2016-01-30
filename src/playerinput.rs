extern crate sdl2;

use std::f32;
use sdl2::keyboard::Scancode;
use math;
use world;
use constants::*;

pub fn handle_player_input(_sdl_context: &sdl2::Sdl, _keyboard: &sdl2::keyboard::KeyboardState, _world: &mut world::World) {
	let &mut player_pos: &mut math::Vector;
	
	{
		let player = &mut _world.m_game_objects[0];
		// The player is always in the middle of the screen -> rotation depends only on window setting
		let (_, mx, my) = _sdl_context.mouse().mouse_state();
		player.m_sprite.m_angle = f32::atan2((my - WIN_HEIGHT/2) as f32,
									 (mx - WIN_WIDTH/2) as f32) * 180.0 / f32::consts::PI + 45.0;
		
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
		
		player.m_sprite.m_location = player.m_sprite.m_location + 5.0 * move_dir;
		player_pos = player.m_sprite.m_location;
	}
	
	// If the player is on a sand-ground tile and presses SPACE build a golem altar
	if _keyboard.is_scancode_pressed(Scancode::Space) {
		// Check if the underlying map type is correct
		let tile_id = _world.get_tile(player_pos);
		if tile_id == 1 {
			_world.set_tile(player_pos, 2);
		}
	}
}